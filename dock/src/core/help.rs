//! Part of the Dock application that handles help command logic

use dyn_clone::DynClone;
use std::fmt::Debug;

use crate::{command::Command, config::AppConfig};

/// Represents a default help message
///
/// Contains the configuration and attributes required to construct the default help message.
///
/// The default help message can be overridden by the user if required by creating a custom struct and implementing the `HelpMessage` trait.
///
#[derive(Debug, Clone)]
pub struct DefaultHelpStructure {
    config: Box<AppConfig>,
    commands: Vec<Box<dyn Command>>,
}

/// This trait handles the methods required to print the help command.
///
/// The user can create a custom `HelpStructure` struct and implement this trait to use it with their Dock application.
///
/// If colored output is enabled, `get_help_colored` is called, otherwise `get_Help` is called.
///
/// ```rs,no_run
/// pub struct CustomHelpStructure{}
///
/// impl HelpMessage for CustomHelpStructure{
///     // --snip--
/// }
/// ````
#[allow(clippy::module_name_repetitions)]
pub trait HelpMessage: DynClone {
    /// Get the default help message
    fn get_help(&self) -> String;
    /// Get the color formatted version of the help message
    fn get_help_colored(&self) -> String;
}

impl Debug for dyn HelpMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_help())
    }
}

impl HelpMessage for DefaultHelpStructure {
    fn get_help(&self) -> String {
        format!(
            r#"
{}

{}


{}      
        "#,
            self.build_header(),
            self.build_commands(),
            self.build_footer()
        )
    }

    fn get_help_colored(&self) -> String {
        format!(
            r#"
{}

{}


{}      
        "#,
            self.build_header_colored(),
            self.build_commands_colored(),
            self.build_footer_colored()
        )
    }
}

impl DefaultHelpStructure {
    /// Create a new instance of `DefaultHelpStructure` using the application config and commands
    #[must_use]
    pub fn new(config: AppConfig, commands: Vec<Box<dyn Command>>) -> Self {
        Self {
            config: Box::new(config),
            commands,
        }
    }

    fn build_header(&self) -> String {
        let config = self.config.clone();
        format!(
            r#"
{} {}
{}
        "#,
            config.name.unwrap_or_default(),
            config.version.unwrap_or_default(),
            config.description.unwrap_or_default(),
        )
    }

    fn build_header_colored(&self) -> String {
        let config = self.config.clone();

        let name = config.name.unwrap_or_default();
        let version = config.version.unwrap_or_default();
        let description = config.description.unwrap_or_default();

        format!(
            "{} {}\n{}",
            crate::Color::Blue.paint(name),
            crate::Color::Blue.paint(version),
            description,
        )
    }

    fn build_commands(&self) -> String {
        self.commands
            .iter()
            .map(|i| i.display())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn build_commands_colored(&self) -> String {
        self.commands
            .iter()
            .map(|i| i.display_colored())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn build_footer(&self) -> String {
        self.config.authors.as_ref().unwrap_or(&vec![]).join(", ")
    }

    fn build_footer_colored(&self) -> String {
        format!(
            "{}",
            crate::Color::Blue.paint(self.config.authors.as_ref().unwrap_or(&vec![]).join(", "))
        )
    }
}
