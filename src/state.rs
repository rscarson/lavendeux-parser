use super::value::AtomicValue;
use super::{functions, decorators, extensions};
use std::collections::HashMap;

#[derive(Clone)]
pub struct ParserState {
    pub variables: HashMap<String, AtomicValue>,
    pub constants: HashMap<String, AtomicValue>,
    pub extensions: Vec<extensions::Extension>,
    pub functions: functions::FunctionTable,
    pub decorators: decorators::DecoratorTable,
}

impl ParserState {
    pub fn new() -> ParserState {
        let mut state = ParserState {
            variables: HashMap::new(),
            constants: HashMap::new(),

            extensions: Vec::new(),
            functions: functions::FunctionTable::new(),
            decorators: decorators::DecoratorTable::new(),
        };

        // Set up constants
        state.constants.insert("pi".to_string(), AtomicValue::Float(std::f64::consts::PI));
        state.constants.insert("e".to_string(), AtomicValue::Float(std::f64::consts::E));
        state.constants.insert("tau".to_string(), AtomicValue::Float(std::f64::consts::TAU));

        return state;
    }
}