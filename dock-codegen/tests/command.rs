#[allow(unused_imports)]
use dock::{command::Command, App};

use dock_codegen::command;

#[test]
fn test_command() {
    #[command(description = "A dummy command for integeration testing.")]
    fn dummy() {
        println!("Dummy command called")
    }

    let _app = App::from_crate()
        .register_command(dummy);
    
}
