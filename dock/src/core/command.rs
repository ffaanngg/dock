//! The part of the application that handles command logic

use dyn_clone::DynClone;

/// The trait that all structs formed from the `command` attribute macro implement.
///
/// This trait provides an interface for `App` to the command created.
///
/// Property getters
///
/// `name` - The name of the command.
/// `description` - The description of the command showed in the help message.
/// `disabled` - The attribute that specifies if a certain command is enabled or not. Defaults to false.
///
/// The `call` associate function invokes the callback of the command and passes the Context formed by `App` to it.
pub trait Command: DynClone {
    /// Get the name of the command
    fn name(&self) -> String;
    /// Get the description of the command
    fn description(&self) -> String;
    /// Get the enabled status of the command
    fn disabled(&self) -> bool;
    /// Invoke the command
    fn call(&self);
}

dyn_clone::clone_trait_object!(Command);

impl std::fmt::Debug for dyn Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl dyn Command {
    /// Displays the command in default representation
    pub fn display(&self) -> String {
        format!("{} {}", self.name(), self.description())
    }

    /// Displays the command in the colored representation
    pub fn display_colored(&self) -> String {
        format!(
            "{} {}",
            crate::Color::Green.paint(self.name()),
            self.description()
        )
    }
}
