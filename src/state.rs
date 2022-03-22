use super::value::AtomicValue;
use super::{functions, decorators, extensions};
use std::collections::HashMap;

const MAX_STACK_DEPTH: usize = 32;

#[derive(Clone)]
pub struct UserFunction {
    pub name: String,
    pub arguments: Vec<String>,
    pub definition: String
}

#[derive(Clone)]
pub struct ParserState {
    pub depth : usize,
    pub variables: HashMap<String, AtomicValue>,
    pub constants: HashMap<String, AtomicValue>,
    pub extensions: Vec<extensions::Extension>,
    pub functions: functions::FunctionTable,
    pub user_functions: HashMap<String, UserFunction>,
    pub decorators: decorators::DecoratorTable,
}

impl ParserState {
    pub fn new() -> ParserState {
        let mut state = ParserState {
            depth: 0,
            variables: HashMap::new(),
            constants: HashMap::new(),

            extensions: Vec::new(),
            functions: functions::FunctionTable::new(),
            user_functions: HashMap::new(),
            decorators: decorators::DecoratorTable::new(),
        };

        // Set up constants
        state.constants.insert("pi".to_string(), AtomicValue::Float(std::f64::consts::PI));
        state.constants.insert("e".to_string(), AtomicValue::Float(std::f64::consts::E));
        state.constants.insert("tau".to_string(), AtomicValue::Float(std::f64::consts::TAU));

        return state;
    }

    pub fn is_depth_ok(&self) -> bool {
        return self.depth < MAX_STACK_DEPTH;
    }
}