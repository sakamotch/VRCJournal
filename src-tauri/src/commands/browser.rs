use tauri_plugin_opener::OpenerExt;

/// Open VRChat instance invite URL in default browser
#[tauri::command]
pub async fn open_invite_url(
    app: tauri::AppHandle,
    world_id: String,
    instance_id: String,
) -> Result<String, String> {
    let url = format!(
        "https://vrchat.com/home/launch?worldId={}&instanceId={}",
        world_id, instance_id
    );

    app.opener()
        .open_url(&url, None::<&str>)
        .map_err(|e| format!("Failed to open URL: {}", e))?;

    Ok(url)
}

/// Open VRChat user page in default browser
#[tauri::command]
pub async fn open_user_page(app: tauri::AppHandle, user_id: String) -> Result<String, String> {
    let url = format!("https://vrchat.com/home/user/{}", user_id);

    app.opener()
        .open_url(&url, None::<&str>)
        .map_err(|e| format!("Failed to open URL: {}", e))?;

    Ok(url)
}
