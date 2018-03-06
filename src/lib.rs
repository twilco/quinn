extern crate bytes;
extern crate byteorder;
extern crate rand;
extern crate openssl;
extern crate slab;
#[macro_use]
extern crate failure;
extern crate digest;
extern crate blake2;
extern crate constant_time_eq;
extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;

mod varint;
mod memory_stream;
mod transport_parameters;

mod frame;
use frame::Frame;

mod from_bytes;
use from_bytes::{FromBytes, BytesExt};

mod endpoint;
pub use endpoint::{Endpoint, Config, PersistentState, ListenConfig, Event, Io};


mod transport_error;
pub use transport_error::Error as TransportError;

pub const VERSION: u32 = 0xff000009;
