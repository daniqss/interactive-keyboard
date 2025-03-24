pub mod animal;
pub mod error;
pub mod notes;
pub mod prelude;
use error::Error;
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
        .invoke_handler(tauri::generate_handler![play_note, select_animal])
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
            let sound = animal.sound(&state.sounds, &note)?;

            #[cfg(test)]
            println!("Playing note {} from animal {}", note, animal);

            tauri::async_runtime::spawn(async move { play_sound(sound).await });
            Ok(())
        }
        None => Err(Error::Generic(format!("Invalid note: {}", note))),
    }
}

async fn play_sound(sound: impl rodio::Source<Item = i16> + Send + 'static) -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    sink.append(sound);
    sink.sleep_until_end();
    Ok(())
}

#[tauri::command]
fn select_animal(animal: String, state: State<'_, Mutex<AppState>>) -> Result<()> {
    let mut state = state.lock().unwrap();
    let animal = Animal::new(&animal)
        .ok_or_else(|| Error::Generic(format!("Invalid animal: {}", animal)))?;

    state.animal = animal;
    println!("Animal selected: {}", animal);

    Ok(())
}
