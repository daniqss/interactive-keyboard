pub mod animal;
pub mod audio;
pub mod error;
pub mod notes;
pub mod prelude;
pub mod serial;
pub mod utils;

use animal::{Animal, AnimalSounds};
use audio::play_note;
use prelude::*;
use serial::watch_serial;
use serialport::SerialPortInfo;
use tauri::{Manager, State};

struct AppState {
    pub animal: Arc<Animal>,
    pub port: Arc<Option<SerialPortInfo>>,
    pub sounds: Arc<AnimalSounds>,
    pub port_sender: Arc<Sender<Arc<Option<SerialPortInfo>>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(port: Option<SerialPortInfo>) -> Result<()> {
    let (port_sender, port_receiver) = mpsc::channel(1);
    let (note_sender, mut note_receiver) = mpsc::channel(100);

    let animal = Arc::new(Animal::Elephant);
    let port = Arc::new(port);
    let port_sender = Arc::new(port_sender);
    let note_sender = Arc::new(note_sender);

    tauri::Builder::default()
        .setup(move |app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Teclado Interactivo")?;

            let sounds = Arc::new(AnimalSounds::new(app.path())?);
            let state = Arc::new(Mutex::new(AppState {
                animal: Arc::clone(&animal),
                port: Arc::clone(&port),
                sounds: Arc::clone(&sounds),
                port_sender: Arc::clone(&port_sender),
            }));

            // thread to watch the serial port
            tauri::async_runtime::spawn(async move {
                let _ = watch_serial(port, port_receiver, note_sender).await;
            });

            // thread to receive notes from the serial port watcher
            tauri::async_runtime::spawn({
                let state: Arc<Mutex<AppState>> = Arc::clone(&state);

                async move {
                    loop {
                        if let Some(note) = note_receiver.recv().await {
                            let locked_state = state.lock().unwrap();
                            let animal = Arc::clone(&locked_state.animal);
                            let sounds = Arc::clone(&locked_state.sounds);

                            if let Err(e) = play_note(note, animal, sounds) {
                                eprintln!("Error playing note from esp32: {}", e);
                            };
                        } else {
                            println!("No note received on tauri thread");
                        }
                    }
                }
            });

            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            play_note_command,
            select_animal,
            reconnect_port
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
fn play_note_command(note: String, state: State<'_, Arc<Mutex<AppState>>>) -> Result<()> {
    let state = state.lock().map_err(|e| Error::Generic(e.to_string()))?;

    let animal = Arc::clone(&state.animal);
    let sounds = Arc::clone(&state.sounds);

    play_note(note, animal, sounds)
}

#[tauri::command]
async fn select_animal(animal: String, state: State<'_, Arc<Mutex<AppState>>>) -> Result<()> {
    let mut state = state.lock().map_err(|e| Error::Generic(e.to_string()))?;

    state.animal = Arc::new(
        Animal::new(&animal)
            .ok_or_else(|| Error::Generic(format!("Invalid animal: {}", animal)))?,
    );

    Ok(())
}

#[tauri::command]
async fn reconnect_port(app_state: State<'_, Arc<Mutex<AppState>>>) -> Result<String> {
    // Clone the port sender before locking the mutex
    let sender = {
        let state = app_state
            .lock()
            .map_err(|e| Error::Generic(e.to_string()))?;
        Arc::clone(&state.port_sender)
    };

    // get the new port outside of the mutex lock
    let new_port = utils::get_port();

    // if a new port is found, send it through the channel
    if let Some(port) = new_port {
        let port_arc = Arc::new(Some(port));

        if sender.send(Arc::clone(&port_arc)).await.is_err() {
            return Err(Error::Generic("Failed to send port".to_string()));
        }

        // Update the state in a separate lock
        let mut state = app_state
            .lock()
            .map_err(|e| Error::Generic(e.to_string()))?;
        state.port = port_arc;
    }

    // return manual serialization of the port
    let state = app_state
        .lock()
        .map_err(|e| Error::Generic(e.to_string()))?;
    Ok(format!(
        "{{ port: {} }}",
        match state.port.as_ref() {
            Some(port) => format!("{{ name: {}}}", port.port_name),
            None => "null".to_string(),
        }
    ))
}
