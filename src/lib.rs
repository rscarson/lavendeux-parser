// Mostly for error type derivisions
#[macro_use]
extern crate derive_more;

mod calculator;
mod functions;
mod decorators;
mod extensions;
mod token;
mod value;
mod state;
mod errors;

pub use errors::ParserError;
pub use token::Token;
pub use state::ParserState;
pub use value::AtomicValue;
pub use value::IntegerType;
pub use value::FloatType;
pub use extensions::Extension;