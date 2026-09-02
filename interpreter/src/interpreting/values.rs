#[derive(Debug, PartialEq)]
pub enum CoreNackValue {
    Int(i64),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub struct NackValue {
    pub value_type: String,
    pub value: CoreNackValue,
}
