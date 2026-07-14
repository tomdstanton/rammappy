use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rammap::fasta::Reader as RustFastxReader;

/// A reader for parsing FASTA/FASTQ files.
/// 
/// The `FastxReader` allows parsing of uncompressed or gzip-compressed
/// FASTA/FASTQ files and acts as a Python iterator, yielding sequence records.
/// 
/// Examples:
///     >>> from rammappy import FastxReader
///     >>> reader = FastxReader("test.fa")
///     >>> for name, seq, qual in reader:
///     ...     print(f"{name}: {seq}")
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy", unsendable)]
pub struct FastxReader {
    reader: RustFastxReader<Box<dyn std::io::BufRead + Send>>,
}

#[gen_stub_pymethods]
#[pymethods]
impl FastxReader {
    /// Open a FASTA/FASTQ file for reading.
    /// 
    /// Args:
    ///     path (os.PathLike): The file path to read from. Supports `.gz` compression.
    /// 
    /// Returns:
    ///     FastxReader: An iterator over the sequence records.
    /// 
    /// Raises:
    ///     IOError: If the file cannot be opened.
    #[new]
    fn new(path: std::path::PathBuf) -> PyResult<Self> {
        let reader = rammap::fasta::open(path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        Ok(Self { reader })
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// Return the next sequence record.
    /// 
    /// Yields:
    ///     tuple[str, bytes, bytes | None]: A tuple containing the sequence name, 
    ///     the sequence itself (as bytes), and optionally the quality string (as bytes).
    fn __next__<'py>(mut slf: PyRefMut<'py, Self>, py: Python<'py>) -> Option<(String, Bound<'py, PyBytes>, Option<Bound<'py, PyBytes>>)> {
        match slf.reader.read_next() {
            Ok(Some(rec)) => {
                let name = rec.name().to_string();
                let seq = PyBytes::new(py, rec.sequence());
                let qual = rec.quality().map(|q| PyBytes::new(py, q));
                Some((name, seq, qual))
            },
            _ => None,
        }
    }
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    if let Err(e) = m.add_class::<FastxReader>() { println!("Error adding FastxReader: {:?}", e); return Err(e); }
    Ok(())
}
