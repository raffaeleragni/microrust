pub mod model;
pub mod serial;

pub mod prelude {
    pub use crate::model::Test;
    pub use crate::serial::{from_avro, to_avro};
}
