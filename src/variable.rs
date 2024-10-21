use std::fmt;
use std::marker::PhantomData;

/// `Variable<T>` represents a variable of type `T` in a formula
#[derive(Debug)]
pub struct Variable<T> {
    name: String,
    var_type: PhantomData<T>,
}

impl<T> Variable<T> {
    /// Create an instance of `Variable<T>`
    ///
    /// # Example
    ///
    /// ```
    /// use r3::variable::Variable;
    ///
    /// let var : Variable<bool> = Variable::new("x");
    /// ```
    pub fn new(name: &str) -> Self {
        Variable {
            name: String::from(name),
            var_type: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl<T> fmt::Display for Variable<T> {
    /// Note: Placeholder name `<empty-name>` is used if `self.name` is empty
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_string = match self.name() {
            "" => "<empty-name>",
            non_empty => non_empty,
        };
        write!(f, "{}", display_string)
    }
}
