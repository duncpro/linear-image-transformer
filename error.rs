#[derive(Debug)]
pub enum AnyError {
    IO(std::io::Error),
    Text(std::string::FromUtf8Error)
}

impl From<std::io::Error> for AnyError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<std::string::FromUtf8Error> for AnyError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::Text(value)
    }
}

