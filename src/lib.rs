#![allow(deprecated)]
use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

pub mod align;
pub mod sketch;
pub mod io;

define_stub_info_gatherer!(stub_info);

#[pymodule]
fn _rammappy(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    align::register(m)?;
    sketch::register(m)?;
    io::register(m)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3_stub_gen::Result;

    #[test]
    fn generate_stubs() -> Result<()> {
        let info = stub_info()?;
        info.generate()?;
        Ok(())
    }
}
