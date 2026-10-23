/// This enum represents all the different possible value types.
#[derive(Debug, PartialEq)]
pub enum CoreNackValue {
    /// A signed integer.
    Int(i64),
    /// A boolean.
    Bool(bool),
}

/// This struct represents a Nack language value, with an associated type.
#[derive(Debug, PartialEq)]
pub struct NackValue {
    /// This value's type.
    pub value_type: String,
    /// The actual underlying value.
    pub value: CoreNackValue,
}

pub struct LazyValue {
    value: Option<NackValue>,
    evaluator: fn() -> NackValue,
}

impl LazyValue {
    pub fn new(evaluator: fn() -> NackValue) -> LazyValue {
        Self {
            value: None,
            evaluator,
        }
    }

    pub fn value(&mut self) -> &NackValue {
        self.value.get_or_insert_with(self.evaluator)
    }
}
