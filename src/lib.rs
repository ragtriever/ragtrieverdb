pub mod db;
pub mod objstore;

#[derive(Debug, Default)]
pub struct RagtrieverDB {}

impl RagtrieverDB {
    pub fn new() -> RagtrieverDB {
        RagtrieverDB {}
    }
}
