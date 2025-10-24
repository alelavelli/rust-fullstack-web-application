//! Enumeration module defines all the enumeration types used
//! by the application

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::error::ServiceAppError;

/// Enumeration with possible object sources used by the application
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum ObjectSourceType {
    AwsS3,
    GcpGS,
    LocalFileSystem,
}

impl TryFrom<&str> for ObjectSourceType {
    type Error = ServiceAppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "awss3" => Ok(Self::AwsS3),
            "gcpgs" => Ok(Self::GcpGS),
            "localfilesystem" => Ok(Self::LocalFileSystem),
            _ => Err(ServiceAppError::InvalidRequest(format!(
                "Cannot create ObjectSourceType from {value}"
            ))),
        }
    }
}

impl Display for ObjectSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ObjectSourceType::LocalFileSystem => "LocalFileSystem",
                ObjectSourceType::AwsS3 => "AwsS3",
                ObjectSourceType::GcpGS => "GcpGS",
            }
        )
    }
}

/// Define which type of configuration is used for the frontend
///
/// The best configuration depends on the specific use case.
/// Usually, integrated mode is useful during testing and to have a
/// single docker file for small releases.
///
/// External is preferred in production environments since the
/// decoupling allows to scale and update separately the frontend
/// and the backend.
#[derive(Debug, Clone)]
pub enum FrontendMode {
    /// Integrated means that the frontend pages are served
    /// as static content directly by the web server.
    /// The string contained in this variant is the root path
    /// of the folder that contains static files.
    Integrated(String),
    /// External means that the frontend is external to the web
    /// application and the web server is decoupled from it
    External,
}

impl TryFrom<&str> for FrontendMode {
    type Error = ServiceAppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "external" => Ok(Self::External),
            value if value.starts_with("integrated") => {
                let path = value.strip_prefix("integrated:").ok_or(ServiceAppError::InvalidRequest(format!("Cannot create FrontendMode from {value}. integrated:path is the expected format")))?;
                Ok(Self::Integrated(path.to_string()))
            }
            _ => Err(ServiceAppError::InvalidRequest(format!(
                "Cannot create FrontendMode from {value}"
            ))),
        }
    }
}
