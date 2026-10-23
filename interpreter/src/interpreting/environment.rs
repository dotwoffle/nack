use crate::interpreting::values::LazyValue;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Default)]
pub struct Environment {
    symbols: HashMap<String, LazyValue>,
}

impl Environment {
    pub fn get_symbol(&self, symbol: &str) -> Option<&LazyValue> {
        self.symbols.get(symbol)
    }

    pub fn create_symbol(
        mut self,
        symbol: String,
        eventual_value: LazyValue,
    ) -> Result<Environment, String> {
        match self.symbols.entry(symbol) {
            Entry::Occupied(entry) => Err(format!(
                "This environment already contains a symbol named '{}'",
                entry.key()
            )),
            Entry::Vacant(entry) => {
                entry.insert(eventual_value);
                Ok(self)
            }
        }
    }
}
