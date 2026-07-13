use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pymethods};
use pyo3_stub_gen::define_stub_info_gatherer;
use rammap::api::{Aligner as RustAligner, Preset as RustPreset, Strand, Mapping as RustMapping};
use rayon::prelude::*;
use rammap::align::sketch::{Sketcher, MinimizerSketcher as RustMinimizerSketcher};
use rammap::align::syncmer::SyncmerSketcher as RustSyncmerSketcher;
use rammap::align::strobemer::RandstrobeSketcher as RustRandstrobeSketcher;

/// Python wrapper for the core `Minimizer` struct.
/// We #[derive(Clone)] so it can be returned easily as a vector to Python.
/// The attributes `x` and `y` are directly exposed as read-only properties to avoid Python getter overhead.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct Minimizer {
    #[pyo3(get)]
    pub x: u64,
    #[pyo3(get)]
    pub y: u64,
}

/// A Sketcher for Minimizers.
/// This exposes the underlying `MinimizerSketcher` from the Rust core.
#[gen_stub_pyclass]
#[pyclass]
pub struct MinimizerSketcher { inner: RustMinimizerSketcher }

#[gen_stub_pymethods]
#[pymethods]
impl MinimizerSketcher {
    /// Initialize a new MinimizerSketcher with k-mer size `k` and window size `w`.
    /// The Rust struct fields are directly instantiated as the Rust core does not expose a `new` function for this type.
    #[new]
    pub fn new(k: usize, w: usize) -> Self {
        Self { inner: RustMinimizerSketcher { k, w, is_hpc: false } }
    }
    
    /// Extract seeds/minimizers from a byte string sequence.
    /// Bytes (`&Bound<'_, PyBytes>`) are preferred here instead of `String` for zero-copy FFI when interfacing with Python `bytes`.
    fn sketch(&self, seq: &Bound<'_, PyBytes>) -> Vec<Minimizer> {
        let seq_bytes = seq.as_bytes();
        let mut out = Vec::new();
        self.inner.sketch(seq_bytes, seq_bytes.len(), 0, &mut out);
        out.into_iter().map(|m| Minimizer { x: m.x, y: m.y }).collect()
    }
}

/// A Sketcher for Syncmers.
#[gen_stub_pyclass]
#[pyclass]
pub struct SyncmerSketcher { inner: RustSyncmerSketcher }

#[gen_stub_pymethods]
#[pymethods]
impl SyncmerSketcher {
    #[new]
    fn new(k: usize, s: usize) -> Self {
        Self { inner: RustSyncmerSketcher::new(k, s) }
    }
    
    fn sketch(&self, seq: &Bound<'_, PyBytes>) -> Vec<Minimizer> {
        let seq_bytes = seq.as_bytes();
        let mut out = Vec::new();
        self.inner.sketch(seq_bytes, seq_bytes.len(), 0, &mut out);
        out.into_iter().map(|m| Minimizer { x: m.x, y: m.y }).collect()
    }
}

/// A Sketcher for Randstrobes.
#[gen_stub_pyclass]
#[pyclass]
pub struct RandstrobeSketcher { inner: RustRandstrobeSketcher }

#[gen_stub_pymethods]
#[pymethods]
impl RandstrobeSketcher {
    #[new]
    fn new(k: usize, w_min: usize, w_max: usize) -> Self {
        Self { inner: RustRandstrobeSketcher::new(k, w_min, w_max) }
    }
    
    fn sketch(&self, seq: &Bound<'_, PyBytes>) -> Vec<Minimizer> {
        let seq_bytes = seq.as_bytes();
        let mut out = Vec::new();
        self.inner.sketch(seq_bytes, seq_bytes.len(), 0, &mut out);
        out.into_iter().map(|m| Minimizer { x: m.x, y: m.y }).collect()
    }
}

/// The mapping presets available in `rammap`.
/// Exposed as a Python Enum for seamless instantiation in Python.
#[gen_stub_pyclass_enum]
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Clone, PartialEq)]
pub enum Preset {
    MapOnt,
    MapHifi,
    Sr,
    Splice,
    Asm5,
    Asm10,
    Asm20,
    MapPb,
}

impl From<Preset> for RustPreset {
    fn from(preset: Preset) -> Self {
        match preset {
            Preset::MapOnt => RustPreset::MapOnt,
            Preset::MapHifi => RustPreset::MapHifi,
            Preset::Sr => RustPreset::Sr,
            Preset::Splice => RustPreset::Splice,
            Preset::Asm5 => RustPreset::Asm5,
            Preset::Asm10 => RustPreset::Asm10,
            Preset::Asm20 => RustPreset::Asm20,
            Preset::MapPb => RustPreset::MapPb,
        }
    }
}

/// Python representation of an alignment `Mapping`.
/// By wrapping `RustMapping` internally, we enforce lazy-evaluation semantics.
/// The fields of the underlying struct are mapped via Python property getters.
#[gen_stub_pyclass]
#[pyclass(unsendable)]
pub struct Mapping {
    // Hold the underlying Rust mapping object directly.
    inner: RustMapping,
}

#[gen_stub_pymethods]
#[pymethods]
impl Mapping {
    /// Return the target name as Python `bytes`.
    #[getter]
    fn target_name<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.inner.target_name.as_bytes())
    }

    #[getter]
    fn target_start(&self) -> usize { self.inner.target_start }
    
    #[getter]
    fn target_end(&self) -> usize { self.inner.target_end }

    #[getter]
    fn target_len(&self) -> usize { self.inner.target_len }

    #[getter]
    fn query_start(&self) -> usize { self.inner.query_start }

    #[getter]
    fn query_end(&self) -> usize { self.inner.query_end }

    #[getter]
    fn strand(&self) -> i8 {
        match self.inner.strand {
            Strand::Forward => 1,
            Strand::Reverse => -1,
        }
    }

    #[getter]
    fn score(&self) -> i32 { self.inner.score }

    #[getter]
    fn mapq(&self) -> i32 { self.inner.mapq }

    #[getter]
    fn is_primary(&self) -> bool { self.inner.is_primary }

    /// Returns the optional CIGAR string as a lazy byte array.
    #[getter]
    fn cigar<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.inner.cigar.as_ref().map(|s| PyBytes::new(py, s.as_bytes()))
    }

    /// Returns the optional cs string as a lazy byte array.
    #[getter]
    fn cs<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.inner.cs.as_ref().map(|s| PyBytes::new(py, s.as_bytes()))
    }

    /// Returns the optional MD string as a lazy byte array.
    #[getter]
    fn md<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.inner.md.as_ref().map(|s| PyBytes::new(py, s.as_bytes()))
    }
}

/// A lazy iterator that yields Mapping wrapper objects on demand.
/// Instead of allocating a `Vec<Mapping>` (which copies the mappings and increases peak memory),
/// we hold an iterator of Rust mappings and materialize Python wrapper objects only when requested via `next()`.
#[gen_stub_pyclass]
#[pyclass(unsendable)]
pub struct MappingIterator {
    iter: std::vec::IntoIter<RustMapping>,
}

#[gen_stub_pymethods]
#[pymethods]
impl MappingIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Mapping> {
        slf.iter.next().map(|m| Mapping { inner: m })
    }
}

/// The core aligner object. 
/// It encapsulates the `RustAligner` from `rammap-core` and executes the alignments.
#[gen_stub_pyclass]
#[pyclass]
pub struct Aligner {
    inner: RustAligner,
}

#[gen_stub_pymethods]
#[pymethods]
impl Aligner {
    /// Create a new aligner instance with predefined target genomes.
    /// Using Python `bytes` for genomic data sidesteps UTF-8 validation overhead present in normal strings.
    #[new]
    #[pyo3(signature = (targets, preset=Preset::MapOnt, do_cigar=true, do_cs=true, do_md=true))]
    fn new(targets: Vec<(Bound<'_, PyBytes>, Bound<'_, PyBytes>)>, preset: Preset, do_cigar: bool, do_cs: bool, do_md: bool) -> PyResult<Self> {
        let preset_enum: RustPreset = preset.into();

        let seqs = targets.into_iter().map(|(name, seq)| {
            (String::from_utf8_lossy(name.as_bytes()).to_string(), seq.as_bytes().to_vec())
        }).collect();
        
        let mut inner = RustAligner::from_seqs(seqs, preset_enum);
        
        {
            let cfg = inner.output_config_mut();
            cfg.do_cigar = do_cigar;
            cfg.do_cs = do_cs;
            cfg.do_md = do_md;
        }

        Ok(Aligner { inner })
    }

    /// Maps a single query sequence sequentially to the targets.
    fn map(&self, query_name: &Bound<'_, PyBytes>, query_seq: &Bound<'_, PyBytes>) -> MappingIterator {
        let query_name_str = String::from_utf8_lossy(query_name.as_bytes()).to_string();
        let map_result = self.inner.map_seq(&query_name_str, query_seq.as_bytes());
        
        MappingIterator {
            iter: map_result.mappings.into_iter()
        }
    }

    /// Performs zero-copy, highly parallelized batch alignments mapping over `queries`.
    /// 
    /// To bypass the GIL and utilize multiple threads for parallelism (via Rayon), we must avoid executing
    /// native Python code or invoking Python objects during the mapping loops. `Bound<'py, PyBytes>` offers zero-copy
    /// memory pointers to the Python memory regions, however, `Bound` relies on the GIL. We drop the GIL using `py.detach(|| ...)`
    /// but must carefully encapsulate the memory bounds outside the Rayon threads using Raw Pointers.
    fn map_batch(
        &self,
        py: Python<'_>,
        queries: Vec<(Bound<'_, PyBytes>, Bound<'_, PyBytes>)>
    ) -> PyResult<Vec<MappingIterator>> {
        // Prepare a vector of pointers and lengths (zero-copy)
        // Since the `Bound<'_, PyBytes>` arguments are held by PyO3 for the duration
        // of this function call, the bytes they point to will not be freed or modified.
        struct RawQuery {
            name_ptr: *const u8,
            name_len: usize,
            seq_ptr: *const u8,
            seq_len: usize,
        }
        
        // Safety: RawQuery is manually marked as Send and Sync so it can cross thread boundaries.
        // We only use the pointers while the GIL is temporarily released, meaning the Python
        // objects holding these bytes cannot be garbage collected.
        unsafe impl Send for RawQuery {}
        unsafe impl Sync for RawQuery {}

        let raw_queries: Vec<RawQuery> = queries.iter().map(|(name, seq)| {
            RawQuery {
                name_ptr: name.as_bytes().as_ptr(),
                name_len: name.as_bytes().len(),
                seq_ptr: seq.as_bytes().as_ptr(),
                seq_len: seq.as_bytes().len(),
            }
        }).collect();

        // Release the GIL via `detach` to allow other Python threads to execute concurrently.
        let iterators: Vec<MappingIterator> = py.detach(|| {
            // Utilize `rayon` to spawn multiple worker threads mapping in parallel.
            raw_queries.par_iter().map(|raw_q| {
                // Safety: Reconstructing the slice is safe because we know the pointer is valid
                // and the memory is immutable as long as the parent function is executing (GIL or no GIL).
                let name_bytes = unsafe { std::slice::from_raw_parts(raw_q.name_ptr, raw_q.name_len) };
                let seq_bytes = unsafe { std::slice::from_raw_parts(raw_q.seq_ptr, raw_q.seq_len) };
                
                let query_name_str = String::from_utf8_lossy(name_bytes);
                let map_result = self.inner.map_seq(&query_name_str, seq_bytes);
                
                MappingIterator {
                    iter: map_result.mappings.into_iter()
                }
            }).collect()
        });

        Ok(iterators)
    }
}

define_stub_info_gatherer!(stub_info);

/// The core PyO3 module initialization sequence. 
#[pymodule]
fn rammappy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    if let Err(e) = m.add_class::<Preset>() { println!("Error adding Preset: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<Aligner>() { println!("Error adding Aligner: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<Mapping>() { println!("Error adding Mapping: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<MappingIterator>() { println!("Error adding MappingIterator: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<Minimizer>() { println!("Error adding Minimizer: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<MinimizerSketcher>() { println!("Error adding MinimizerSketcher: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<SyncmerSketcher>() { println!("Error adding SyncmerSketcher: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<RandstrobeSketcher>() { println!("Error adding RandstrobeSketcher: {:?}", e); return Err(e); }
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
