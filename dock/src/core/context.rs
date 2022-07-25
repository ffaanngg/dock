//! The part of the Dock application that handles context which is passed to commands when invoked

use crate::{command::Command, App};
use std::{env::consts, io::Result, path::PathBuf};

/// Holds contextual information about a command execution
///
/// A `Context` information is automatically created and passed into the command callback when invoked.
/// The `Context` API also provides a short and intuitive interface to properties that are usually accessed manually
/// such as system information and environment variables.
///
/// An instance of this struct is automatically formed by `App` and should not be manually created.
///
/// # Examples
///
/// ```rs,norun
///
/// println!("This command was invoked on a {} computer", ctx.env.os)
///
/// ```
/// where ctx is an instance of `Context` passed into a callback.
///
///
#[allow(unused)]
pub struct Context {
    /// The command that was invoked
    pub command: Box<dyn Command>,
    /// Information about the environment of program execution
    pub env: Environment,
    /// Application struct
    pub app: App,
}

impl Context {

    /// Construct a new Context instance based on the command and the application
    #[must_use]
    pub fn new(command: Box<dyn Command>, app: App) -> Self {
        Self {
            command,
            env: Environment::default(),
            app,
        }
    }
}

/// Holds core information about the execution environment.
#[allow(unused)]
pub struct Environment {
    /// The operating system on which the app is run
    os: String,
    /// Local environment variables
    env: std::env::Vars,
    /// Argv passed for the execution
    argv: std::env::Args,
    /// The current working directory
    current_dir: Result<PathBuf>,
    /// The full filesystem path of the current running executable
    current_exe: Result<PathBuf>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            os: consts::OS.to_string(),
            env: std::env::vars(),
            argv: std::env::args(),
            current_dir: std::env::current_dir(),
            current_exe: std::env::current_exe(),
        }
    }
}
