pub trait Command {
    fn name(&self) -> String;
    fn call(&self);
}

impl std::fmt::Debug for dyn Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
