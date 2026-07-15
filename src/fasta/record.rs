use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rammap::fasta::Record as RustRecord;

/// A sequence record from a FASTA or FASTQ file.
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy")]
#[derive(Clone)]
pub struct Record {
    pub(crate) inner: RustRecord,
}

#[gen_stub_pymethods]
#[pymethods]
impl Record {
    #[new]
    #[pyo3(signature = (name, description, sequence, quality=None))]
    fn new(
        name: String,
        description: Option<String>,
        sequence: Vec<u8>,
        quality: Option<Vec<u8>>,
    ) -> Self {
        Self {
            inner: RustRecord::new(name, description, sequence, quality),
        }
    }

    #[getter]
    fn name(&self) -> &str {
        self.inner.name()
    }

    #[getter]
    fn description(&self) -> Option<&str> {
        self.inner.description()
    }

    #[getter]
    fn sequence<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.inner.sequence())
    }

    #[getter]
    fn quality<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.inner.quality().map(|q| PyBytes::new(py, q))
    }

    fn __repr__(&self) -> String {
        format!("<Record: {}>", self.inner.name())
    }
}
