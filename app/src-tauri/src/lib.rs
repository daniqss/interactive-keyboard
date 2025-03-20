pub mod animal;
pub mod error;
pub mod notes;
pub mod prelude;
use error::Error;
use std::{fs::File, io::BufReader, sync::Mutex, thread};

use animal::{Animal, AnimalSounds};
use notes::Note;
use prelude::*;
use rodio::{source::Speed, Decoder, OutputStream, Sink};
use serialport::SerialPortInfo;
use tauri::{Manager, State};

struct AppState {
    pub animal: Animal,
    pub port: Option<SerialPortInfo>,
    pub sounds: AnimalSounds,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(port: Option<SerialPortInfo>) -> Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Teclado Interactivo")?;

            let path = app.path();

            app.manage(Mutex::new(AppState {
                animal: Animal::Elephant,
                port,
                sounds: AnimalSounds::new(path)?,
            }));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![play_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
fn play_note(note: String, state: State<'_, Mutex<AppState>>) -> Result<()> {
    let state = state.lock().unwrap();

    println!("port is {:?}", state.port);

    let animal = state.animal;
    match Note::new(&note) {
        Some(note) => {
            let sound = match animal.sound(&state.sounds, &note) {
                Ok(sound) => sound,
                Err(e) => return Err(e.into()),
            };

            println!("Playing sound with animal {} and note {}", animal, note);
            thread::spawn(move || {
                let (_stream, stream_handle) = match OutputStream::try_default() {
                    Ok(stream) => stream,
                    Err(e) => {
                        eprintln!("Failed to get default output stream: {}", e);
                        return;
                    }
                };
                let sink = match Sink::try_new(&stream_handle) {
                    Ok(sink) => sink,
                    Err(e) => {
                        eprintln!("Failed to create sink: {}", e);
                        return;
                    }
                };
                sink.append(sound);
                sink.sleep_until_end();
            });
            println!("Sound played successfully");

            Ok(())
        }
        None => Err(Error::Generic(format!("Note {} not found", note))),
    }
}
