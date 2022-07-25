#[allow(unused_imports)]
use dock::command::Command;

use dock_codegen::command;

#[test]
fn test_proc_macro() {
    #[command(description = "Description of the test command")]
    fn test() {
        println!("Printed from test function of proc-macro")
    }

    assert_eq!("test", test.name());
    println!("-> {}", test.description());
}
