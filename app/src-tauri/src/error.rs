#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic: {0}")]
    Generic(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Path(#[from] tauri::Error),

    #[error(transparent)]
    SerialPort(#[from] serialport::Error),

    #[error(transparent)]
    Rodio(#[from] rodio::decoder::DecoderError),
}
