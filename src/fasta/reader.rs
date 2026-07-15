use crate::fasta::record::Record;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyfunction, gen_stub_pymethods};
use rammap::fasta::Reader as RustFastxReader;

/// A reader for parsing FASTA/FASTQ files.
///
/// The `FastxReader` allows parsing of uncompressed or gzip-compressed
/// FASTA/FASTQ files and acts as a Python iterator, yielding sequence records.
///
/// Examples:
///     >>> from rammappy import FastxReader
///     >>> reader = FastxReader("test.fa")
///     >>> for record in reader:
///     ...     print(f"{record.name}: {record.sequence}")
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
    ///     Record: The sequence record.
    fn __next__<'py>(&mut self) -> Option<Record> {
        match self.reader.read_next() {
            Ok(Some(rec)) => Some(Record { inner: rec }),
            _ => None,
        }
    }

    /// Read sequences until cumulative bases exceed batch_size.
    /// Returns (seqs, is_eof). Caller can call again for the next batch.
    fn read_batch<'py>(
        &mut self,
        py: Python<'py>,
        batch_size: u64,
    ) -> PyResult<(Vec<(String, Bound<'py, PyBytes>)>, bool)> {
        match self.reader.read_batch(batch_size) {
            Ok((seqs, is_eof)) => {
                let py_seqs = seqs
                    .into_iter()
                    .map(|(n, s)| (n, PyBytes::new(py, &s)))
                    .collect();
                Ok((py_seqs, is_eof))
            }
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
        }
    }
}

/// Read all sequences from a FASTA file into memory.
/// Returns a list of (name, sequence) pairs.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn read_fasta<'py>(
    py: Python<'py>,
    path: std::path::PathBuf,
) -> PyResult<Vec<(String, Bound<'py, PyBytes>)>> {
    match rammap::fasta::read_fasta(path) {
        Ok(seqs) => {
            let py_seqs = seqs
                .into_iter()
                .map(|(n, s)| (n, PyBytes::new(py, &s)))
                .collect();
            Ok(py_seqs)
        }
        Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e.to_string())),
    }
}

/// Parse FASTA from a byte slice (works with mmap or regular buffers)
/// Returns a list of (name, sequence) pairs.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn parse_fasta_bytes<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyBytes>,
) -> PyResult<Vec<(String, Bound<'py, PyBytes>)>> {
    match rammap::fasta::parse_fasta_bytes(data.as_bytes()) {
        Ok(seqs) => {
            let py_seqs = seqs
                .into_iter()
                .map(|(n, s)| (n, PyBytes::new(py, &s)))
                .collect();
            Ok(py_seqs)
        }
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
    }
}
