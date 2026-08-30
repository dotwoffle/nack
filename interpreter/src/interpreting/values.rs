#[derive(Debug)]
pub enum CoreNackValue {
    Int(i64),
    Bool(bool),
}

pub struct NackValue {
    pub value_type: String,
    pub value: CoreNackValue,
}
