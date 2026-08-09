#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,
    // TODO: Float, Bool, Str, Function, etc.
}

impl Type {
    pub fn name(&self) -> &'static str {
        match self {
            Type::Int => "int",
        }
    }
}