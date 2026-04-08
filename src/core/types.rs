pub mod node;
pub mod param;
pub mod ramp;
pub mod spare;

pub use node::*;
pub use param::*;
pub use ramp::*;
pub use spare::*;

use std::sync::atomic::AtomicUsize;

pub static NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
