use std::fmt;
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ApkParserError {
    Io(io::Error),
    Zip(zip::result::ZipError),
    Utf8(FromUtf8Error),
    InvalidManifest(String),
    InvalidSignature(String),
    InvalidIcon(String),
    InvalidSecurityConfig(String),
    Aapt2Error(String),
    Other(String),
}

impl fmt::Display for ApkParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApkParserError::Io(e) => write!(f, "IO error: {}", e),
            ApkParserError::Zip(e) => write!(f, "Zip error: {}", e),
            ApkParserError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            ApkParserError::InvalidManifest(e) => write!(f, "Invalid manifest: {}", e),
            ApkParserError::InvalidSignature(e) => write!(f, "Invalid signature: {}", e),
            ApkParserError::InvalidIcon(e) => write!(f, "Invalid icon: {}", e),
            ApkParserError::InvalidSecurityConfig(e) => write!(f, "Invalid security config: {}", e),
            ApkParserError::Aapt2Error(e) => write!(f, "AAPT2 error: {}", e),
            ApkParserError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl std::error::Error for ApkParserError {}

impl From<io::Error> for ApkParserError {
    fn from(err: io::Error) -> Self {
        ApkParserError::Io(err)
    }
}

impl From<zip::result::ZipError> for ApkParserError {
    fn from(err: zip::result::ZipError) -> Self {
        ApkParserError::Zip(err)
    }
}

impl From<FromUtf8Error> for ApkParserError {
    fn from(err: FromUtf8Error) -> Self {
        ApkParserError::Utf8(err)
    }
} 