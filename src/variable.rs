use std::fmt;
use std::marker::PhantomData;

/// Type-alias for a (unique) identifier for a variable
pub type VarId = u32;

/// A variable in a formula
/// Variable<T> represents a variable of type T
#[derive(Debug)]
pub struct Variable<T> {
    name: String,
    id: VarId,
    var_type: PhantomData<T>,
}

impl<T> Variable<T> {
    pub fn new(name: &str, id: VarId) -> Self {
        Variable {
            name: String::from(name),
            id,
            var_type: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn id(&self) -> VarId {
        self.id
    }
}

impl<T> fmt::Display for Variable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
