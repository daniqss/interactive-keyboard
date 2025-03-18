pub mod animal;
pub mod error;
pub mod prelude;
use std::sync::Mutex;

use animal::Animal;
use prelude::*;
use serialport::SerialPortInfo;
use tauri::{Manager, State};

#[tauri::command]
fn play_note(note: String, state: State<'_, Mutex<AppState>>) {
    let state = state.lock().unwrap();

    // println!("port is {:?}", state.port); {
    if let Some(port) = &state.port {
        println!("port is {:?}", port);
    } else {
        println!("no port");
    }
    match note.as_str() {
        "do" => println!("DO! {}", state.animal),
        "re" => println!("RE! {}", state.animal),
        "mi" => println!("MI!"),
        "fa" => println!("FA!"),
        "sol" => println!("SOL! {}", state.animal),
        "la" => println!("LA!"),
        "si" => println!("SI!"),
        "do-sharp" => println!("DO#!"),
        _ => unreachable!(),
    }
}

struct AppState {
    pub animal: animal::Animal,
    pub port: Option<SerialPortInfo>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(port: Option<SerialPortInfo>) -> Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Teclado Interactivo")?;
            // main_window.set_decorations(false)?;

            app.manage(Mutex::new(AppState {
                animal: Animal::Elephant,
                port,
            }));

            println!("app running!");
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![play_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
