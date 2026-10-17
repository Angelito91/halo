// The Halo Programming Language — string builtin code generation, split by
// responsibility (`helpers` owns the macro/libc shims used by the rest).

mod case;
mod helpers;
mod replace;
mod search;
mod substr;
mod trim;

pub use case::*;
pub use replace::*;
pub use search::*;
pub use substr::*;
pub use trim::*;
