use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rammap::align::sketch::{Sketcher, MinimizerSketcher as RustMinimizerSketcher};
use rammap::align::syncmer::SyncmerSketcher as RustSyncmerSketcher;
use rammap::align::strobemer::RandstrobeSketcher as RustRandstrobeSketcher;

/// Represents a k-mer sketch (minimizer, syncmer, etc.).
/// 
/// Contains the genomic coordinate and the hash value.
/// 
/// Attributes:
///     x (int): The 64-bit integer combining the genomic coordinate and other metadata.
///     y (int): The 64-bit hash value of the k-mer.
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy", from_py_object)]
#[derive(Clone)]
pub struct Minimizer {
    #[pyo3(get)]
    pub x: u64,
    #[pyo3(get)]
    pub y: u64,
}

/// A sketcher that extracts minimizers from sequences.
///
/// Minimizers are the lexicographically smallest k-mers in a sliding window of size w.
/// 
/// Examples:
///     >>> from rammappy.sketch import MinimizerSketcher
///     >>> sketcher = MinimizerSketcher(k=15, w=10)
///     >>> sketcher.sketch(b"ATGCGTACGATCGATC")
///     [<Minimizer object at ...>, ...]
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy")]
pub struct MinimizerSketcher { inner: RustMinimizerSketcher }

#[gen_stub_pymethods]
#[pymethods]
impl MinimizerSketcher {
    /// Initialize a new MinimizerSketcher.
    ///
    /// Args:
    ///     k (int): The k-mer size.
    ///     w (int): The window size.
    ///
    /// Returns:
    ///     MinimizerSketcher: The initialized sketcher.
    #[new]
    pub fn new(k: usize, w: usize) -> Self {
        Self { inner: RustMinimizerSketcher { k, w, is_hpc: false } }
    }
    
    /// Extract minimizers from a byte string sequence.
    ///
    /// Args:
    ///     seq (bytes): The sequence to sketch.
    ///
    /// Returns:
    ///     list[Minimizer]: A list of minimizer objects.
    fn sketch(&self, seq: &Bound<'_, PyBytes>) -> Vec<Minimizer> {
        let seq_bytes = seq.as_bytes();
        let mut out = Vec::new();
        self.inner.sketch(seq_bytes, seq_bytes.len(), 0, &mut out);
        out.into_iter().map(|m| Minimizer { x: m.x, y: m.y }).collect()
    }
}

/// A sketcher that extracts syncmers from sequences.
///
/// Syncmers provide a more evenly spaced sampling of sequences compared to minimizers.
///
/// Examples:
///     >>> from rammappy.sketch import SyncmerSketcher
///     >>> sketcher = SyncmerSketcher(k=15, s=5)
///     >>> sketcher.sketch(b"ATGCGTACGATCGATC")
///     [<Minimizer object at ...>, ...]
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy")]
pub struct SyncmerSketcher { inner: RustSyncmerSketcher }

#[gen_stub_pymethods]
#[pymethods]
impl SyncmerSketcher {
    /// Create a new SyncmerSketcher.
    ///
    /// Args:
    ///     k (int): The k-mer size.
    ///     s (int): The s-mer size.
    ///
    /// Returns:
    ///     SyncmerSketcher: The initialized sketcher.
    #[new]
    fn new(k: usize, s: usize) -> Self {
        Self { inner: RustSyncmerSketcher::new(k, s) }
    }
    
    /// Extract syncmers from a byte string sequence.
    ///
    /// Args:
    ///     seq (bytes): The sequence to sketch.
    ///
    /// Returns:
    ///     list[Minimizer]: A list of syncmer objects.
    fn sketch(&self, seq: &Bound<'_, PyBytes>) -> Vec<Minimizer> {
        let seq_bytes = seq.as_bytes();
        let mut out = Vec::new();
        self.inner.sketch(seq_bytes, seq_bytes.len(), 0, &mut out);
        out.into_iter().map(|m| Minimizer { x: m.x, y: m.y }).collect()
    }
}

/// A sketcher that extracts randstrobes from sequences.
///
/// Randstrobes are combinations of k-mers that provide more resilient matching.
///
/// Examples:
///     >>> from rammappy.sketch import RandstrobeSketcher
///     >>> sketcher = RandstrobeSketcher(k=15, w_min=10, w_max=30)
///     >>> sketcher.sketch(b"ATGCGTACGATCGATC")
///     [<Minimizer object at ...>, ...]
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy")]
pub struct RandstrobeSketcher { inner: RustRandstrobeSketcher }

#[gen_stub_pymethods]
#[pymethods]
impl RandstrobeSketcher {
    /// Create a new RandstrobeSketcher.
    ///
    /// Args:
    ///     k (int): The k-mer size.
    ///     w_min (int): The minimum window size.
    ///     w_max (int): The maximum window size.
    ///
    /// Returns:
    ///     RandstrobeSketcher: The initialized sketcher.
    #[new]
    fn new(k: usize, w_min: usize, w_max: usize) -> Self {
        Self { inner: RustRandstrobeSketcher::new(k, w_min, w_max) }
    }
    
    /// Extract randstrobes from a byte string sequence.
    ///
    /// Args:
    ///     seq (bytes): The sequence to sketch.
    ///
    /// Returns:
    ///     list[Minimizer]: A list of randstrobe objects.
    fn sketch(&self, seq: &Bound<'_, PyBytes>) -> Vec<Minimizer> {
        let seq_bytes = seq.as_bytes();
        let mut out = Vec::new();
        self.inner.sketch(seq_bytes, seq_bytes.len(), 0, &mut out);
        out.into_iter().map(|m| Minimizer { x: m.x, y: m.y }).collect()
    }
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    if let Err(e) = m.add_class::<Minimizer>() { println!("Error adding Minimizer: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<MinimizerSketcher>() { println!("Error adding MinimizerSketcher: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<SyncmerSketcher>() { println!("Error adding SyncmerSketcher: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<RandstrobeSketcher>() { println!("Error adding RandstrobeSketcher: {:?}", e); return Err(e); }
    Ok(())
}
