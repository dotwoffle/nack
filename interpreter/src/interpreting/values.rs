pub trait NackValue {
    fn get_type(&self) -> &String;
}

pub struct NackPrimitive<T> {
    pub value: T,
    nack_type: String,
}

impl<T> NackPrimitive<T> {
    pub fn new(value: T, nack_type: String) -> NackPrimitive<T> {
        NackPrimitive { value, nack_type }
    }
}

impl<T> NackValue for NackPrimitive<T> {
    fn get_type(&self) -> &String {
        &self.nack_type
    }
}
