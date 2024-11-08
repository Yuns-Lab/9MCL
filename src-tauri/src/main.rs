#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod network_core;
use network_core::NetworkCore;
use std::collections::HashMap;

#[tauri::command]
async fn network_request(
    useheader: bool,
    url: &str,
    headers: &str,
) -> Result<String, String> {
    let networkc = NetworkCore::new();

    if useheader {
        // Parse headers from `&str` to `HashMap<String, String>`
        let headers_map: HashMap<String, String> = headers
            .split(';')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, ':');
                Some((
                    parts.next()?.trim().to_string(),
                    parts.next()?.trim().to_string(),
                ))
            })
            .collect();

        match networkc.get_request_with_headers(url, headers_map).await {
            Ok(response_body) => Ok(response_body),
            Err(e) => Err(format!("Error: {:?}", e)),
        }
    } else {
        match networkc.get_request(url).await {
            Ok(response_body) => Ok(response_body),
            Err(e) => Err(format!("Error: {:?}", e)),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![network_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
