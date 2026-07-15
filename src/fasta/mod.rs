pub mod reader;
pub mod record;
pub mod stream;

use pyo3::prelude::*;

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<record::Record>()?;
    m.add_class::<reader::FastxReader>()?;
    m.add_class::<stream::FastaStreamer>()?;
    m.add_class::<stream::FastqStreamer>()?;

    m.add_function(wrap_pyfunction!(reader::read_fasta, m)?)?;
    m.add_function(wrap_pyfunction!(reader::parse_fasta_bytes, m)?)?;

    Ok(())
}
