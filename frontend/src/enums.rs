use std::fmt::Display;

pub enum HttpStatus {
    /// 1xx
    Info(u16),
    /// 2xx
    Success(u16),
    /// 3xx
    Redirect(u16),
    /// 4xx
    ClientError(u16),
    /// 5xx
    ServerError(u16),
}

impl From<u16> for HttpStatus {
    fn from(value: u16) -> Self {
        match value {
            100..200 => HttpStatus::Info(value),
            200..300 => HttpStatus::Success(value),
            300..400 => HttpStatus::Redirect(value),
            400..500 => HttpStatus::ClientError(value),
            _ => HttpStatus::ServerError(value),
        }
    }
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            HttpStatus::Info(code) => write!(f, "Informational ({})", code),
            HttpStatus::Success(code) => write!(f, "Success ({})", code),
            HttpStatus::Redirect(code) => write!(f, "Redirect ({})", code),
            HttpStatus::ClientError(code) => write!(f, "Client Error ({})", code),
            HttpStatus::ServerError(code) => write!(f, "Server Error ({})", code),
        }
    }
}
