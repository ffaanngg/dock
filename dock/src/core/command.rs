use std::fmt::Debug;

pub trait Command: Debug {
    fn name(&self) -> String;
}
