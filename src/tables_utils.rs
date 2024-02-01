use substreams_entity_change::tables::{Row, ToValue};

pub trait SetOptional {
    fn set_optional<T>(&mut self, name: &str, value: Option<T>) -> &mut Self
    where
        T: ToValue;
}

impl SetOptional for Row {
    fn set_optional<T>(&mut self, name: &str, value: Option<T>) -> &mut Self
    where
        T: ToValue,
    {
        if let Some(value) = value {
            self.set(name, value);
        }
        self
    }
}
