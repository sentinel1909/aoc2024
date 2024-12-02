// errors/src/lib.rs

// enum type to represent possible error variants
#[derive(Debug)]
pub enum AppError {
    IOError(std::io::Error), 
}

// implement the From trait for the IOError variant
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IOError(error)
    }
}

