use crate::{command::Command, config::AppConfig};
use std::string::ToString;

/// Represents a default help message
///
/// Contains the configuration and attributes required to construct the default help message.
///
/// The default help message can be overridden by the user if required by creating a custom struct and implementing the `HelpMessage` trait.
///
#[derive(Debug)]
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
pub trait HelpMessage {
    fn get_help(&self) -> String;
    fn get_help_colored(&self) -> String;
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
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn build_commands_colored(&self) -> String {
        self.commands
            .iter()
            .map(ToString::to_string)
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
