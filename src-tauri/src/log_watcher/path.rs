use std::env;
use std::path::PathBuf;

/// VRChatログディレクトリのパスを取得
/// Windows: %USERPROFILE%\AppData\LocalLow\VRChat\VRChat\
pub fn get_vrchat_log_path() -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    {
        let userprofile = env::var("USERPROFILE")
            .map_err(|_| "USERPROFILE environment variable not found".to_string())?;

        let log_path = PathBuf::from(userprofile)
            .join("AppData")
            .join("LocalLow")
            .join("VRChat")
            .join("VRChat");

        if log_path.exists() {
            Ok(log_path)
        } else {
            Err(format!("VRChat log directory not found at {:?}", log_path))
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("VRChat is only available on Windows".to_string())
    }
}

/// 全てのログファイルを取得
/// output_log_*.txt の全てを最終更新日時順（古い順）で返す
pub fn get_all_log_files() -> Result<Vec<PathBuf>, String> {
    let log_dir = get_vrchat_log_path()?;

    let mut log_files: Vec<PathBuf> = std::fs::read_dir(&log_dir)
        .map_err(|e| format!("Failed to read log directory: {}", e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with("output_log") && name.ends_with(".txt"))
                .unwrap_or(false)
        })
        .collect();

    if log_files.is_empty() {
        return Err("No VRChat log files found".to_string());
    }

    // 最終更新日時でソート（古い順）
    log_files.sort_by_key(|path| std::fs::metadata(path).and_then(|m| m.modified()).ok());

    Ok(log_files)
}

/// 最新のログファイルを取得
/// output_log_*.txt の中で最新のものを返す
pub fn get_latest_log_file() -> Result<PathBuf, String> {
    let log_files = get_all_log_files()?;

    log_files
        .last()
        .cloned()
        .ok_or_else(|| "Failed to get latest log file".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_get_vrchat_log_path() {
        // この環境でVRChatがインストールされているかはわからないので、
        // パスの構築だけテスト
        let result = get_vrchat_log_path();
        // エラーでもOK（VRChatがインストールされていない環境）
        if let Ok(path) = result {
            assert!(path.to_str().unwrap().contains("VRChat"));
        }
    }
}
