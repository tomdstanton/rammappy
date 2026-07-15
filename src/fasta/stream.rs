use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rammap::fasta::stream::{
    FastaStreamer as RustFastaStreamer, FastqStreamer as RustFastqStreamer,
};

/// Streaming FASTA parser. Push bytes via `push`; pop completed
/// `(name, seq)` pairs via `next_record`; flush the
/// in-flight record (if any) at end-of-input via `finalize`.
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy")]
pub struct FastaStreamer {
    inner: RustFastaStreamer,
}

#[gen_stub_pymethods]
#[pymethods]
impl FastaStreamer {
    #[new]
    #[pyo3(signature = (rna_to_dna=true))]
    fn new(rna_to_dna: bool) -> Self {
        Self {
            inner: RustFastaStreamer::new().with_rna_to_dna(rna_to_dna),
        }
    }

    /// Feed a chunk of bytes. Records that complete inside this chunk are
    /// queued for `next_record`.
    fn push(&mut self, chunk: &Bound<'_, PyBytes>) {
        self.inner.push(chunk.as_bytes());
    }

    /// Pop a completed record from the internal queue.
    fn next_record<'py>(&mut self, py: Python<'py>) -> Option<(String, Bound<'py, PyBytes>)> {
        self.inner
            .next_record()
            .map(|(n, s)| (n, PyBytes::new(py, &s)))
    }

    /// Flush the trailing partial line + the in-flight record. Call once after
    /// the last `push`. The flushed record (if any) is appended to the queue
    /// — drain with `next_record`.
    fn finalize(&mut self) {
        self.inner.finalize();
    }
}

/// Streaming FASTQ parser. 4-line records: `@name`, sequence, `+`, quality.
/// Quality scores are consumed and discarded; only `(name, seq)` is yielded.
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy")]
pub struct FastqStreamer {
    inner: RustFastqStreamer,
}

#[gen_stub_pymethods]
#[pymethods]
impl FastqStreamer {
    #[new]
    #[pyo3(signature = (rna_to_dna=true))]
    fn new(rna_to_dna: bool) -> Self {
        Self {
            inner: RustFastqStreamer::new().with_rna_to_dna(rna_to_dna),
        }
    }

    /// Feed a chunk of bytes.
    fn push(&mut self, chunk: &Bound<'_, PyBytes>) {
        self.inner.push(chunk.as_bytes());
    }

    /// Pop a completed record from the internal queue.
    fn next_record<'py>(&mut self, py: Python<'py>) -> Option<(String, Bound<'py, PyBytes>)> {
        self.inner
            .next_record()
            .map(|(n, s)| (n, PyBytes::new(py, &s)))
    }

    /// Flush the trailing partial line + the in-flight record.
    fn finalize(&mut self) {
        self.inner.finalize();
    }
}
