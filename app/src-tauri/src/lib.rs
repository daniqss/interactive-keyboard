pub mod animal;
pub mod error;
pub mod notes;
pub mod prelude;
use std::sync::Mutex;

use animal::{Animal, AnimalSounds};
use notes::Note;
use prelude::*;
use rodio::{OutputStream, Sink};
use serialport::SerialPortInfo;
use tauri::{Manager, State};

struct AppState {
    pub animal: Animal,
    pub port: Option<SerialPortInfo>,
    pub sounds: AnimalSounds,
    pub sink: Sink,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(port: Option<SerialPortInfo>) -> Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Teclado Interactivo")?;

            let (_stream, stream_handle) = OutputStream::try_default()?;
            let path = app.path();

            app.manage(Mutex::new(AppState {
                animal: Animal::Elephant,
                port,
                sounds: AnimalSounds::new(path)?,
                sink: Sink::try_new(&stream_handle)?,
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

#[tauri::command]
fn play_note(note: String, state: State<'_, Mutex<AppState>>) {
    let state = state.lock().unwrap();

    println!("port is {:?}", state.port);

    let animal = state.animal;
    match Note::new(&note) {
        Some(note) => {
            let sound = match animal.sound(&state.sounds, &note) {
                Ok(sound) => sound,
                Err(e) => {
                    println!("Error loading sound: {}", e);
                    return;
                }
            };

            println!("Playing sound with animal {} and note {}", animal, note);
            state.sink.append(sound);
        }
        None => println!("Invalid note"),
    }
}
