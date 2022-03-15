use std::error::Error;

pub trait Logger {
    async fn write_to_log(&self, &str to_write) -> Result<(), Box<dyn Error>>;
}

pub struct InfoLogger {}

impl Logger for InfoLogger {
    async fn write_to_log(&self, to_write: &str) -> Result<(), Box<dyn Error>> {
        info!(to_write);
        Ok(())
    }
}