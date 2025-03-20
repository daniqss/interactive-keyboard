#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic: {0}")]
    Generic(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    TauriPath(#[from] tauri::Error),

    #[error(transparent)]
    SerialPort(#[from] serialport::Error),

    #[error(transparent)]
    Rodio(#[from] rodio::decoder::DecoderError),

    #[error(transparent)]
    RodioStream(#[from] rodio::StreamError),

    #[error(transparent)]
    RodioPlay(#[from] rodio::PlayError),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
