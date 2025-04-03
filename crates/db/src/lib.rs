pub mod clients;
pub mod command;
pub mod connection;
pub mod constants;
pub mod db;
pub mod frame;
pub mod hnswlib;
pub mod objstore;
pub mod parser;
pub mod server;
pub mod shutdown;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Default)]
pub struct RagtrieverDB {}

impl RagtrieverDB {
    pub fn new() -> RagtrieverDB {
        RagtrieverDB {}
    }
}
