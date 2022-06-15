use dock::command::Command;
use dock_codegen::command;

#[test]
fn test_proc_macro() {
    #[command]
    fn test() {}

    assert_eq!("test", test.name())
}
