use crate::db::operations;
use crate::parser::LogEvent;
use rusqlite::Connection;
use std::collections::HashMap;

use super::{handlers, types::ProcessedEvent};

/// Event processor: Processes LogEvents and stores them in the database
pub struct EventProcessor {
    current_my_account_id: Option<i64>,   // Current local account
    current_user_id: Option<i64>,         // Current user (corresponds to my_account)
    current_instance_id: Option<i64>,     // Current active instance
    user_ids: HashMap<String, i64>,       // vrchat_user_id -> users.id mapping
    instance_user_ids: HashMap<i64, i64>, // user_id -> instance_users.id mapping
    pending_avatars: HashMap<String, (i64, String)>, // display_name -> (avatar_id, timestamp) for avatars seen before PlayerJoined
}

impl EventProcessor {
    pub fn new() -> Self {
        Self {
            current_my_account_id: None,
            current_user_id: None,
            current_instance_id: None,
            user_ids: HashMap::new(),
            instance_user_ids: HashMap::new(),
            pending_avatars: HashMap::new(),
        }
    }

    /// Restore state from database to resume incremental log processing.
    pub fn restore_previous_state(&mut self, conn: &Connection) -> Result<(), rusqlite::Error> {
        // Restore the most recently authenticated local account
        if let Some((my_account_id, user_id)) = operations::get_latest_authenticated_account(conn)?
        {
            self.current_my_account_id = Some(my_account_id);
            self.current_user_id = Some(user_id);
            println!(
                "EventProcessor initialized with my_account_id: {}, user_id: {}",
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
        match event {
            LogEvent::UserAuthenticated {
                timestamp,
                user_id,
                display_name,
            } => handlers::user_authenticated::handle(
                conn,
                &timestamp.to_rfc3339(),
                &user_id,
                &display_name,
                &mut self.current_my_account_id,
                &mut self.current_user_id,
                &mut self.user_ids,
            ),
            LogEvent::JoiningWorld {
                timestamp,
                world_id,
                world_name,
                instance_id,
            } => handlers::joining_world::handle(
                conn,
                &timestamp.to_rfc3339(),
                &world_id,
                &world_name,
                &instance_id,
                self.current_my_account_id,
                self.current_user_id,
                &mut self.current_instance_id,
                &mut self.instance_user_ids,
                &mut self.pending_avatars,
            ),
            LogEvent::EnteringRoom {
                timestamp,
                world_name,
            } => handlers::entering_room::handle(
                conn,
                &timestamp.to_rfc3339(),
                &world_name,
                self.current_instance_id,
            ),
            LogEvent::DestroyingPlayer {
                timestamp,
                display_name,
            } => handlers::destroying_player::handle(
                conn,
                &timestamp.to_rfc3339(),
                &display_name,
                self.current_user_id,
                &mut self.current_instance_id,
                &self.user_ids,
                &mut self.instance_user_ids,
                &mut self.pending_avatars,
            ),
            LogEvent::PlayerJoined {
                timestamp,
                display_name,
                user_id,
            } => handlers::player_joined::handle(
                conn,
                &timestamp.to_rfc3339(),
                &display_name,
                &user_id,
                self.current_instance_id,
                &mut self.user_ids,
                &mut self.instance_user_ids,
                &mut self.pending_avatars,
            ),
            LogEvent::AvatarChanged {
                timestamp,
                display_name,
                avatar_name,
            } => handlers::avatar_changed::handle(
                conn,
                &timestamp.to_rfc3339(),
                &display_name,
                &avatar_name,
                self.current_user_id,
                self.current_instance_id,
                &self.user_ids,
                &mut self.pending_avatars,
            ),
            LogEvent::ScreenshotTaken {
                timestamp,
                file_path,
            } => handlers::screenshot_taken::handle(
                conn,
                &timestamp.to_rfc3339(),
                &file_path,
                self.current_instance_id,
            ),
            LogEvent::EventSyncFailed { timestamp } => handlers::event_sync_failed::handle(
                conn,
                &timestamp.to_rfc3339(),
                self.current_instance_id,
            ),
        }
    }
}

impl Default for EventProcessor {
    fn default() -> Self {
        Self::new()
    }
}
