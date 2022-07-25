//! The part of the application that handles config related mechanics

#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};
use std::fs;

/// Represents a table in Config.toml
#[derive(Serialize, Deserialize)]
pub struct CargoConfig {
    package: CargoConfigPackage,
}
/// Represents a table in Config.toml
#[derive(Serialize, Deserialize)]
pub struct CargoConfigPackage {
    name: Option<String>,
    description: Option<String>,
    authors: Option<Vec<String>>,
    version: Option<String>,
}

///Holds the application config
///
#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    /// Name of the application
    pub name: Option<String>,
    /// Description of the application
    pub description: Option<String>,
    /// Authors of the application
    pub authors: Option<Vec<String>>,
    /// The application version
    pub version: Option<String>,
}

impl AppConfig {

    /// Construct the base default config
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct the application config based on the crate details
    #[must_use]
    pub fn from_crate() -> Self {
        let config = toml::from_str::<CargoConfig>(
            fs::read_to_string("./Cargo.toml")
                .expect("Failed to read Cargo.toml")
                .as_str(),
        )
        .expect("Failed to parse Cargo.toml");

        Self {
            name: config.package.name,
            description: config.package.description,
            authors: config.package.authors,
            version: config.package.version,
        }
    }
}
