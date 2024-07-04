use libsql::Builder;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet() -> () {
    println!("Hello from Rust!");
    // let db = Builder::new_local("new-local.db").build().await.unwrap();
    // let conn = db.connect().unwrap();

    // format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
//     let url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
// let token = env::var("LIBSQL_AUTH_TOKEN").unwrap_or_default();

    greet().await;

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
