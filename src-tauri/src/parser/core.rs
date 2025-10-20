use crate::types::LogEvent;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use regex::Regex;

pub struct LogParser {
    auth_regex: Regex,
    joining_regex: Regex,
    entering_room_regex: Regex,
    player_joined_regex: Regex,
    avatar_changed_regex: Regex,
    screenshot_regex: Regex,
    leaving_instance_regex: Regex,
    event_sync_failed_regex: Regex,
}

impl LogParser {
    pub fn new() -> Self {
        Self {
            // 2025.10.13 09:53:16 Debug      -  User Authenticated: DisplayName (usr_xxx)
            auth_regex: Regex::new(
                r"(\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2}) .* User Authenticated: (.+?) \((usr_[a-f0-9\-]+)\)"
            ).unwrap(),

            // 2025.10.13 09:53:22 Debug      -  [Behaviour] Joining wrld_xxx:instance_id~region(jp)
            joining_regex: Regex::new(
                r"(\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2}) .* \[Behaviour\] Joining (wrld_[a-f0-9\-]+):(.+)"
            ).unwrap(),

            // 2025.10.13 10:55:55 Debug      -  [Behaviour] Joining or Creating Room: VRChat Home
            entering_room_regex: Regex::new(
                r"(\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2}) .* \[Behaviour\] Joining or Creating Room: (.+)"
            ).unwrap(),

            // 2025.10.13 11:02:36 Debug      -  [Behaviour] OnPlayerJoined DisplayName (usr_xxx)
            player_joined_regex: Regex::new(
                r"(\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2}) .* \[Behaviour\] OnPlayerJoined (.+?) \((usr_[a-f0-9\-]+)\)"
            ).unwrap(),

            // 2025.10.13 11:02:36 Debug      -  [Behaviour] Switching DisplayName to avatar AvatarName
            avatar_changed_regex: Regex::new(
                r"(\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2}) .* \[Behaviour\] Switching (.+?) to avatar (.+)"
            ).unwrap(),

            // 2025.10.15 15:48:41 Debug      -  [VRC Camera] Took screenshot to: D:\path\to\screenshot.png
            screenshot_regex: Regex::new(
                r"(\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2}) .* \[VRC Camera\] Took screenshot to: (.+)"
            ).unwrap(),

            // 2025.10.15 15:49:00 Debug      -  [Behaviour] Destroying DisplayName
            leaving_instance_regex: Regex::new(
                r"(\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2}) .* \[Behaviour\] Destroying (.+)"
            ).unwrap(),

            // 2025.10.19 08:10:44 Error      -  [Behaviour] Master is not sending any events! Moving to a new instance.
            event_sync_failed_regex: Regex::new(
                r"(\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2}) .* \[Behaviour\] Master is not sending any events! Moving to a new instance\."
            ).unwrap(),
        }
    }

    pub fn parse_line(&self, line: &str) -> Option<LogEvent> {
        if let Some(caps) = self.auth_regex.captures(line) {
            return Some(LogEvent::UserAuthenticated {
                timestamp: parse_timestamp(&caps[1])?,
                display_name: caps[2].to_string(),
                user_id: caps[3].to_string(),
            });
        }

        if let Some(caps) = self.joining_regex.captures(line) {
            return Some(LogEvent::JoiningWorld {
                timestamp: parse_timestamp(&caps[1])?,
                world_id: caps[2].to_string(),
                instance_id: caps[3].to_string(),
                world_name: String::new(), // 次の "EnteringRoom" イベントで更新
            });
        }

        if let Some(caps) = self.entering_room_regex.captures(line) {
            return Some(LogEvent::EnteringRoom {
                timestamp: parse_timestamp(&caps[1])?,
                world_name: caps[2].to_string(),
            });
        }

        if let Some(caps) = self.player_joined_regex.captures(line) {
            return Some(LogEvent::PlayerJoined {
                timestamp: parse_timestamp(&caps[1])?,
                display_name: caps[2].to_string(),
                user_id: caps[3].to_string(),
            });
        }

        if let Some(caps) = self.avatar_changed_regex.captures(line) {
            return Some(LogEvent::AvatarChanged {
                timestamp: parse_timestamp(&caps[1])?,
                display_name: caps[2].to_string(),
                avatar_name: caps[3].to_string(),
            });
        }

        if let Some(caps) = self.screenshot_regex.captures(line) {
            return Some(LogEvent::ScreenshotTaken {
                timestamp: parse_timestamp(&caps[1])?,
                file_path: caps[2].to_string(),
            });
        }

        if let Some(caps) = self.leaving_instance_regex.captures(line) {
            return Some(LogEvent::DestroyingPlayer {
                timestamp: parse_timestamp(&caps[1])?,
                display_name: caps[2].to_string(),
            });
        }

        if let Some(caps) = self.event_sync_failed_regex.captures(line) {
            return Some(LogEvent::EventSyncFailed {
                timestamp: parse_timestamp(&caps[1])?,
            });
        }

        None
    }
}

/// タイムスタンプをパース (2025.10.13 09:53:16)
/// VRChatのログはローカル時刻で記録されているため、ローカルタイムゾーンとして解釈してUTCに変換
fn parse_timestamp(s: &str) -> Option<DateTime<Utc>> {
    NaiveDateTime::parse_from_str(s, "%Y.%m.%d %H:%M:%S")
        .ok()
        .and_then(|dt| Local.from_local_datetime(&dt).single())
        .map(|local_dt| local_dt.with_timezone(&Utc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_user_authenticated() {
        let parser = LogParser::new();
        let line = "2025.10.13 09:53:16 Debug      -  User Authenticated: TestUser (usr_12345678-abcd-ef01-2345-6789abcdef01)";

        let event = parser.parse_line(line).expect("Failed to parse");

        match event {
            LogEvent::UserAuthenticated {
                display_name,
                user_id,
                ..
            } => {
                assert_eq!(display_name, "TestUser");
                assert_eq!(user_id, "usr_12345678-abcd-ef01-2345-6789abcdef01");
            }
            _ => panic!("Expected UserAuthenticated event"),
        }
    }

    #[test]
    fn test_parse_joining_world() {
        let parser = LogParser::new();
        let line = "2025.10.13 09:53:22 Debug      -  [Behaviour] Joining wrld_abcdef01-2345-6789-abcd-ef0123456789:11859~friends(usr_xxx)~region(jp)";

        let event = parser.parse_line(line).expect("Failed to parse");

        match event {
            LogEvent::JoiningWorld {
                world_id,
                instance_id,
                ..
            } => {
                assert_eq!(world_id, "wrld_abcdef01-2345-6789-abcd-ef0123456789");
                assert_eq!(instance_id, "11859~friends(usr_xxx)~region(jp)");
            }
            _ => panic!("Expected JoiningWorld event"),
        }
    }

    #[test]
    fn test_parse_joining_world_simple() {
        let parser = LogParser::new();
        let line = "2025.10.13 09:53:22 Debug      -  [Behaviour] Joining wrld_abcdef01-2345-6789-abcd-ef0123456789:84455~region(jp)";

        let event = parser.parse_line(line).expect("Failed to parse");

        match event {
            LogEvent::JoiningWorld {
                world_id,
                instance_id,
                ..
            } => {
                assert_eq!(world_id, "wrld_abcdef01-2345-6789-abcd-ef0123456789");
                assert_eq!(instance_id, "84455~region(jp)");
            }
            _ => panic!("Expected JoiningWorld event"),
        }
    }

    #[test]
    fn test_parse_player_joined() {
        let parser = LogParser::new();
        let line = "2025.10.13 11:02:36 Debug      -  [Behaviour] OnPlayerJoined TestPlayer (usr_12345678-abcd-ef01-2345-6789abcdef01)";

        let event = parser.parse_line(line).expect("Failed to parse");

        match event {
            LogEvent::PlayerJoined {
                display_name,
                user_id,
                ..
            } => {
                assert_eq!(display_name, "TestPlayer");
                assert_eq!(user_id, "usr_12345678-abcd-ef01-2345-6789abcdef01");
            }
            _ => panic!("Expected PlayerJoined event"),
        }
    }

    #[test]
    fn test_parse_avatar_changed() {
        let parser = LogParser::new();
        let line =
            "2025.10.13 11:02:36 Debug      -  [Behaviour] Switching TestUser to avatar TestAvatar";

        let event = parser.parse_line(line).expect("Failed to parse");

        match event {
            LogEvent::AvatarChanged {
                display_name,
                avatar_name,
                ..
            } => {
                assert_eq!(display_name, "TestUser");
                assert_eq!(avatar_name, "TestAvatar");
            }
            _ => panic!("Expected AvatarChanged event"),
        }
    }

    #[test]
    fn test_parse_entering_room() {
        let parser = LogParser::new();
        let line =
            "2025.10.13 10:55:55 Debug      -  [Behaviour] Joining or Creating Room: VRChat Home";

        let event = parser.parse_line(line).expect("Failed to parse");

        match event {
            LogEvent::EnteringRoom { world_name, .. } => {
                assert_eq!(world_name, "VRChat Home");
            }
            _ => panic!("Expected EnteringRoom event"),
        }
    }

    #[test]
    fn test_parse_screenshot_taken() {
        let parser = LogParser::new();
        let line = "2025.10.15 15:48:41 Debug      -  [VRC Camera] Took screenshot to: D:\\VRChat\\Screenshots\\VRChat_2025-10-15_15-48-41.png";

        let event = parser.parse_line(line).expect("Failed to parse");

        match event {
            LogEvent::ScreenshotTaken { file_path, .. } => {
                assert_eq!(
                    file_path,
                    "D:\\VRChat\\Screenshots\\VRChat_2025-10-15_15-48-41.png"
                );
            }
            _ => panic!("Expected ScreenshotTaken event"),
        }
    }

    #[test]
    fn test_parse_destroying_player() {
        let parser = LogParser::new();
        let line = "2025.10.15 15:49:00 Debug      -  [Behaviour] Destroying TestPlayer";

        let event = parser.parse_line(line).expect("Failed to parse");

        match event {
            LogEvent::DestroyingPlayer { display_name, .. } => {
                assert_eq!(display_name, "TestPlayer");
            }
            _ => panic!("Expected DestroyingPlayer event"),
        }
    }

    #[test]
    fn test_parse_event_sync_failed() {
        let parser = LogParser::new();
        let line = "2025.10.19 08:10:44 Error      -  [Behaviour] Master is not sending any events! Moving to a new instance.";

        let event = parser.parse_line(line).expect("Failed to parse");

        match event {
            LogEvent::EventSyncFailed { .. } => {
                // Success
            }
            _ => panic!("Expected EventSyncFailed event"),
        }
    }

    #[test]
    fn test_parse_invalid_line() {
        let parser = LogParser::new();
        let line = "2025.10.13 11:02:36 Debug      -  Some random log line";

        let event = parser.parse_line(line);
        assert!(event.is_none());
    }
}
