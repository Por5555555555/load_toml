use std::path::Path;

use anyhow::Result;

pub trait ModelStrut {
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    fn write(&self) -> Result<()>;
}
