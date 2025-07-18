// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Write;
use tauri::command;
use std::process::Command;


#[command]
fn save_to_env_file(
    key_id: String,
    application_key: String,
    endpoint: String,
    region: String,
    bucket_name: String,
    duration_time: String,
    prefix: Option<String>,
) -> Result<(), String> {
    let prefix = prefix.unwrap_or_default();
    let content = format!(
        "KEY_ID={}\nAPPLICATION_KEY={}\nENDPOINT={}\nREGION={}\nBUCKET_NAME={}\nDURATION_TIME={}\nPREFIX={}",
        key_id, application_key, endpoint, region, bucket_name, duration_time, prefix
    );

    fs::File::create(".env")
        .and_then(|mut file| file.write_all(content.as_bytes()))
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
fn load_env_file() -> Result<String, String> {
    fs::read_to_string(".env").map_err(|e| e.to_string())
}

#[command]
fn sign() {
    let mut child = Command::new("go")
        .args(["run", "main.go"])
        .current_dir("../src-go")
        .spawn()
        .expect("failed to execute the sign module");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_to_env_file,load_env_file,sign])
        .run(tauri::generate_context!())
        .expect("error while running application");
}
