pub trait Command {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn disabled(&self) -> bool;
    fn call(&self);
}

impl std::fmt::Debug for dyn Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Display for dyn Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name(), self.description())
    }
}
