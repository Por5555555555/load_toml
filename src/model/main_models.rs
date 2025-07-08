use std::path::Path;

use anyhow::Result;

pub trait ModelStrut {
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
    fn write(&self) -> Result<()>;
    fn test() -> Self
    where
        Self: Sized;
}
