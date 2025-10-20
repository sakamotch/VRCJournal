mod handlers;

use crate::db::operations;
use crate::types::{LogEvent, ProcessedEvent};
use rusqlite::Connection;
use std::collections::HashMap;

/// Context passed to event handlers
pub(super) struct HandlerContext<'a> {
    pub current_my_account_id: &'a mut Option<i64>,
    pub current_user_id: &'a mut Option<i64>,
    pub current_instance_id: &'a mut Option<i64>,
    pub user_ids: &'a mut HashMap<String, i64>,
    pub instance_user_ids: &'a mut HashMap<i64, i64>,
    pub display_name_to_user_id: &'a mut HashMap<String, i64>,
    pub pending_avatars: &'a mut HashMap<String, (i64, String)>,
}

/// Event handler for processing log events
pub struct EventHandler {
    current_my_account_id: Option<i64>,   // Current local account
    current_user_id: Option<i64>,         // Current user (corresponds to my_account)
    current_instance_id: Option<i64>,     // Current active instance
    user_ids: HashMap<String, i64>,       // vrchat_user_id -> users.id mapping
    instance_user_ids: HashMap<i64, i64>, // user_id -> instance_users.id mapping
    display_name_to_user_id: HashMap<String, i64>, // display_name -> users.id mapping
    pending_avatars: HashMap<String, (i64, String)>, // display_name -> (avatar_id, timestamp) for avatars seen before PlayerJoined
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            current_my_account_id: None,
            current_user_id: None,
            current_instance_id: None,
            user_ids: HashMap::new(),
            instance_user_ids: HashMap::new(),
            display_name_to_user_id: HashMap::new(),
            pending_avatars: HashMap::new(),
        }
    }

    /// Restore state from database
    pub fn restore_previous_state(&mut self, conn: &Connection) -> Result<(), rusqlite::Error> {
        // Restore the most recently authenticated local account
        if let Some((my_account_id, user_id)) = operations::get_latest_authenticated_account(conn)?
        {
            self.current_my_account_id = Some(my_account_id);
            self.current_user_id = Some(user_id);
            println!(
                "EventHandler initialized with my_account_id: {}, user_id: {}",
                my_account_id, user_id
            );

            // Find active instance for this account
            if let Some(instance_id) = operations::get_latest_active_instance(conn, my_account_id)?
            {
                self.current_instance_id = Some(instance_id);
                println!("Found active instance: {}", instance_id);

                // Restore users currently in the instance
                let users = operations::get_instance_active_users(conn, instance_id)?;
                for (vrchat_user_id, user_id, instance_user_id) in users {
                    self.user_ids.insert(vrchat_user_id, user_id);
                    self.instance_user_ids.insert(user_id, instance_user_id);

                    // Restore display_name mapping
                    let display_name = operations::get_user_display_name(conn, user_id)?;
                    self.display_name_to_user_id.insert(display_name, user_id);
                }

                println!("Restored {} users in current instance", self.user_ids.len());
            }
        } else {
            println!("No local account found. Waiting for authentication event.");
        }

        Ok(())
    }

    /// Process a log event
    pub fn process_event(
        &mut self,
        conn: &Connection,
        event: LogEvent,
    ) -> Result<Option<ProcessedEvent>, rusqlite::Error> {
        let mut ctx = HandlerContext {
            current_my_account_id: &mut self.current_my_account_id,
            current_user_id: &mut self.current_user_id,
            current_instance_id: &mut self.current_instance_id,
            user_ids: &mut self.user_ids,
            instance_user_ids: &mut self.instance_user_ids,
            display_name_to_user_id: &mut self.display_name_to_user_id,
            pending_avatars: &mut self.pending_avatars,
        };

        match event {
            LogEvent::UserAuthenticated {
                timestamp,
                user_id,
                display_name,
            } => handlers::user_authenticated::handle(
                conn,
                &mut ctx,
                &timestamp.to_rfc3339(),
                &user_id,
                &display_name,
            ),
            LogEvent::JoiningWorld {
                timestamp,
                world_id,
                instance_id,
            } => handlers::joining_world::handle(
                conn,
                &mut ctx,
                &timestamp.to_rfc3339(),
                &world_id,
                &instance_id,
            ),
            LogEvent::EnteringRoom {
                timestamp,
                world_name,
            } => handlers::entering_room::handle(conn, &ctx, &timestamp.to_rfc3339(), &world_name),
            LogEvent::DestroyingPlayer {
                timestamp,
                display_name,
            } => handlers::destroying_player::handle(
                conn,
                &mut ctx,
                &timestamp.to_rfc3339(),
                &display_name,
            ),
            LogEvent::PlayerJoined {
                timestamp,
                display_name,
                user_id,
            } => handlers::player_joined::handle(
                conn,
                &mut ctx,
                &timestamp.to_rfc3339(),
                &display_name,
                &user_id,
            ),
            LogEvent::AvatarChanged {
                timestamp,
                display_name,
                avatar_name,
            } => handlers::avatar_changed::handle(
                conn,
                &mut ctx,
                &timestamp.to_rfc3339(),
                &display_name,
                &avatar_name,
            ),
            LogEvent::ScreenshotTaken {
                timestamp,
                file_path,
            } => {
                handlers::screenshot_taken::handle(conn, &ctx, &timestamp.to_rfc3339(), &file_path)
            }
            LogEvent::EventSyncFailed { timestamp } => {
                handlers::event_sync_failed::handle(conn, &ctx, &timestamp.to_rfc3339())
            }
        }
    }
}
