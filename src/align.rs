use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pymethods};
use rammap::api::{Aligner as RustAligner, Preset as RustPreset, Strand as RustStrand, Mapping as RustMapping, CigarOp as RustCigarOp};
use rammap::align::index::Index as RustIndex;
use std::path::PathBuf;
use rayon::prelude::*;

/// The mapping presets available in `rammappy`.
/// 
/// These presets configure the aligner for different sequencing technologies 
/// and use cases, tuning heuristics and scoring.
/// 
/// Examples:
///     >>> from rammappy import Index, Aligner, Preset
///     >>> index = Index.build([(b"target1", b"ATGC...")])
///     >>> aligner = Aligner(index, preset=Preset.MapOnt)
#[gen_stub_pyclass_enum]
#[pyclass(module = "rammappy._rammappy", eq, eq_int, from_py_object)]
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

/// Strand orientation of an alignment.
/// 
/// Represents whether the query mapped to the forward or reverse strand of the target.
///
/// Attributes:
///     Forward: The forward strand.
///     Reverse: The reverse complement strand.
#[gen_stub_pyclass_enum]
#[pyclass(module = "rammappy._rammappy", eq, eq_int, from_py_object)]
#[derive(Clone, PartialEq)]
pub enum Strand {
    Forward,
    Reverse,
}

impl From<RustStrand> for Strand {
    fn from(s: RustStrand) -> Self {
        match s {
            RustStrand::Forward => Strand::Forward,
            RustStrand::Reverse => Strand::Reverse,
        }
    }
}

impl From<Strand> for RustStrand {
    fn from(s: Strand) -> Self {
        match s {
            Strand::Forward => RustStrand::Forward,
            Strand::Reverse => RustStrand::Reverse,
        }
    }
}

/// BAM CIGAR operation encodings as a Python Enum.
/// 
/// The values correspond to the official BAM specification.
#[gen_stub_pyclass_enum]
#[pyclass(eq, eq_int, module = "rammappy._rammappy")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CigarOp {
    M = 0,
    I = 1,
    D = 2,
    N = 3,
    S = 4,
    H = 5,
    P = 6,
    EQ = 7,
    X = 8,
    B = 9,
}

impl From<u8> for CigarOp {
    fn from(op: u8) -> Self {
        match op {
            0 => CigarOp::M,
            1 => CigarOp::I,
            2 => CigarOp::D,
            3 => CigarOp::N,
            4 => CigarOp::S,
            5 => CigarOp::H,
            6 => CigarOp::P,
            7 => CigarOp::EQ,
            8 => CigarOp::X,
            9 => CigarOp::B,
            _ => CigarOp::M,
        }
    }
}

/// Structured CIGAR operation element (length and operation type).
///
/// Attributes:
///     len (int): Operation length.
///     op (CigarOp): Operation type enum.
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy", get_all, from_py_object)]
#[derive(Clone)]
pub struct CigarElement {
    pub len: u32,
    pub op: CigarOp,
}

impl From<RustCigarOp> for CigarElement {
    fn from(op: RustCigarOp) -> Self {
        CigarElement {
            len: op.len,
            op: op.op.into(),
        }
    }
}

/// Python representation of an alignment `Mapping`.
/// 
/// Mappings are lazily evaluated: the actual Rust-level objects are preserved 
/// until accessed via the Python properties.
///
/// Attributes:
///     target_name (bytes): The name of the target sequence.
///     target_id (int): Target sequence numeric index.
///     target_len (int): Target sequence length.
///     target_start (int): Start coordinate on the target.
///     target_end (int): End coordinate on the target.
///     query_start (int): Start coordinate on the query.
///     query_end (int): End coordinate on the query.
///     strand (Strand): Orientation of the alignment.
///     score (int): Alignment score.
///     mapq (int): Mapping quality (0-255).
///     is_primary (bool): True if this is the primary alignment.
///     is_supplementary (bool): True if this is a supplementary alignment.
///     is_spliced (bool): True if this alignment contains splice junctions.
///     trans_strand (Strand | None): Transcript strand for splice alignments, if known.
///     matches (int): Number of matching bases.
///     block_len (int): Alignment block length.
///     edit_distance (int): Edit distance (NM tag).
///     divergence (float): Sequence divergence (0.0 = identical).
///     cigar (bytes | None): The CIGAR string, if requested during alignment.
///     cigar_ops (list[CigarElement] | None): Structured CIGAR operations, if requested.
///     cs (bytes | None): CS tag string, if requested.
///     md (bytes | None): MD tag string, if requested.
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy", unsendable)]
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
    fn strand(&self) -> Strand {
        self.inner.strand.into()
    }

    #[getter]
    fn score(&self) -> i32 { self.inner.score }

    #[getter]
    fn mapq(&self) -> i32 { self.inner.mapq }

    #[getter]
    fn is_primary(&self) -> bool { self.inner.is_primary }

    #[getter]
    fn is_supplementary(&self) -> bool { self.inner.is_supplementary }

    #[getter]
    fn is_spliced(&self) -> bool { self.inner.is_spliced }

    #[getter]
    fn trans_strand(&self) -> Option<Strand> {
        self.inner.trans_strand.map(|s| s.into())
    }

    #[getter]
    fn matches(&self) -> usize { self.inner.matches }

    #[getter]
    fn block_len(&self) -> usize { self.inner.block_len }

    #[getter]
    fn edit_distance(&self) -> u32 { self.inner.edit_distance }

    #[getter]
    fn target_id(&self) -> usize { self.inner.target_id }

    #[getter]
    fn divergence(&self) -> f64 { self.inner.divergence }

    /// Returns the optional CIGAR string as a lazy byte array.
    #[getter]
    fn cigar<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.inner.cigar.as_ref().map(|s| PyBytes::new(py, s.as_bytes()))
    }

    /// Returns the structured CIGAR operations.
    #[getter]
    fn cigar_ops(&self) -> Option<Vec<CigarElement>> {
        self.inner.cigar_ops.as_ref().map(|ops| ops.iter().map(|&op| op.into()).collect())
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

    fn __str__(&self) -> String {
        let strand = match self.inner.strand {
            RustStrand::Forward => "+",
            RustStrand::Reverse => "-",
        };
        let primary = if self.inner.is_primary { "*" } else { "" };
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}{}",
            self.inner.query_start,
            self.inner.query_end,
            strand,
            self.inner.target_name,
            self.inner.target_len,
            self.inner.target_start,
            self.inner.target_end,
            self.inner.score,
            primary
        )
    }

    fn __repr__(&self) -> String {
        let strand = match self.inner.strand {
            RustStrand::Forward => "+",
            RustStrand::Reverse => "-",
        };
        format!(
            "<Mapping: target='{}' [{}:{}] query=[{}:{}] strand='{}' mapq={} score={}>",
            self.inner.target_name,
            self.inner.target_start,
            self.inner.target_end,
            self.inner.query_start,
            self.inner.query_end,
            strand,
            self.inner.mapq,
            self.inner.score
        )
    }
}

/// A lazy iterator that provides `Mapping` objects.
/// 
/// Instead of allocating a list, we hold an iterator of Rust mappings 
/// and materialize Python wrapper objects only when requested via `next()`.
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy", unsendable)]
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

/// The `Index` object represents a genomic sequence index.
/// 
/// It holds the internal Rust Index for alignment. You can construct an index
/// from a collection of sequences, or load it from a previously saved file.
/// 
/// Examples:
///     >>> from rammappy import Index
///     >>> index = Index.build([(b"target1", b"ATGC...")])
///     >>> index.save("my_index.mmi")
///     >>> loaded_index = Index.load("my_index.mmi")
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy", unsendable)]
#[derive(Clone)]
pub struct Index {
    inner: RustIndex,
}

#[gen_stub_pymethods]
#[pymethods]
impl Index {
    /// Build an index from target sequences.
    ///
    /// Args:
    ///     seqs (list[tuple[bytes, bytes]]): A list of tuples containing `(name, sequence)` as bytes.
    ///     w (int): Window size. Defaults to 10.
    ///     k (int): K-mer size. Defaults to 15.
    ///     is_hpc (bool): Homopolymer compressed. Defaults to False.
    ///     max_occ (int): Maximum occurrences. Defaults to 50000.
    ///
    /// Returns:
    ///     Index: The built index.
    #[staticmethod]
    #[pyo3(signature = (seqs, w=10, k=15, is_hpc=false, max_occ=50000))]
    fn build(seqs: Vec<(Bound<'_, PyBytes>, Bound<'_, PyBytes>)>, w: usize, k: usize, is_hpc: bool, max_occ: usize) -> Self {
        let rust_seqs = seqs.into_iter().map(|(name, seq)| {
            (String::from_utf8_lossy(name.as_bytes()).to_string(), seq.as_bytes().to_vec())
        }).collect();
        Index {
            inner: RustIndex::build(rust_seqs, w, k, is_hpc, max_occ),
        }
    }

    /// Load an index from file.
    ///
    /// Args:
    ///     path (os.PathLike): The file path to load the index from.
    ///
    /// Returns:
    ///     Index: The loaded index.
    #[staticmethod]
    fn load(path: PathBuf) -> PyResult<Self> {
        let path_str = path.to_str().ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Invalid path string"))?;
        match RustIndex::load(path_str) {
            Ok(idx) => Ok(Index { inner: idx }),
            Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e.to_string())),
        }
    }

    /// Save the index to a file.
    ///
    /// Args:
    ///     path (os.PathLike): The file path to save the index to.
    fn save(&self, path: PathBuf) -> PyResult<()> {
        let path_str = path.to_str().ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Invalid path string"))?;
        match self.inner.save(path_str) {
            Ok(_) => Ok(()),
            Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e.to_string())),
        }
    }

    /// Strip sequences from the index to save memory.
    ///
    /// This removes the actual sequence bytes from memory, which is useful when
    /// you only need to perform mapping and do not need base-level alignment (CIGAR).
    ///
    /// Examples:
    ///     >>> index.strip_sequences()
    fn strip_sequences(&mut self) {
        self.inner.strip_sequences();
    }

    #[getter]
    fn kmer_size(&self) -> usize { self.inner.kmer_size }

    #[getter]
    fn window_size(&self) -> usize { self.inner.window_size }

    #[getter]
    fn homopolymer_compressed(&self) -> bool { self.inner.homopolymer_compressed }

    /// Returns the sequence names in the index.
    ///
    /// Returns:
    ///     list[str]: The list of sequence names.
    #[getter]
    fn seq_names(&self) -> Vec<String> {
        self.inner.seqs.iter().map(|s| s.name.clone()).collect()
    }

    /// Returns the sequence for a given target name.
    ///
    /// Args:
    ///     name (str): The name of the target sequence.
    ///     start (int, optional): The 0-based start coordinate. Defaults to 0.
    ///     end (int, optional): The 0-based end coordinate. Defaults to the end of the sequence.
    ///
    /// Returns:
    ///     str: The requested sequence.
    ///
    /// Raises:
    ///     ValueError: If the sequence name is not found in the index.
    #[pyo3(signature = (name, start=None, end=None))]
    fn seq(&self, name: &str, start: Option<usize>, end: Option<usize>) -> PyResult<String> {
        let rid = self.inner.seqs.iter().position(|s| s.name == name)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(format!("Sequence not found: {}", name)))?;
        
        let seq_len = self.inner.seqs[rid].len;
        let s = start.unwrap_or(0).min(seq_len);
        let e = end.unwrap_or(seq_len).min(seq_len);
        
        if s >= e {
            return Ok(String::new());
        }
        
        let nt4 = self.inner.get_region_nt4(rid, s, e);
        let ascii: String = nt4.into_iter()
            .map(|b| RustIndex::NT4_TO_ASCII[b as usize] as char)
            .collect();
            
        Ok(ascii)
    }
}

/// The `Aligner` orchestrates the alignment process.
/// 
/// It encapsulates the alignment configuration and the index to map query sequences against reference targets.
/// 
/// Examples:
///     >>> from rammappy import Index, Aligner, Preset
///     >>> index = Index.build([(b"target1", b"ATGC...")])
///     >>> aligner = Aligner(index, preset=Preset.MapOnt)
///     >>> for mapping in aligner.map(b"query1", b"ATGC..."):
///     ...     print(mapping.score)
#[gen_stub_pyclass]
#[pyclass(module = "rammappy._rammappy")]
pub struct Aligner {
    inner: RustAligner,
}

#[gen_stub_pymethods]
#[pymethods]
impl Aligner {
    /// Create a new aligner instance using an already built index.
    /// 
    /// Args:
    ///     index (Index): The built index object.
    ///     preset (Preset): The preset configuration (e.g. `Preset.MapOnt`). Defaults to `Preset.MapOnt`.
    ///     do_cigar (bool): Whether to compute CIGAR strings. Defaults to `True`.
    ///     do_cs (bool): Whether to compute `cs` tags. Defaults to `True`.
    ///     do_md (bool): Whether to compute `md` tags. Defaults to `True`.
    /// 
    /// Returns:
    ///     Aligner: The initialized aligner object.
    #[new]
    #[pyo3(signature = (index, preset=Preset::MapOnt, do_cigar=true, do_cs=true, do_md=true))]
    fn new(index: &Index, preset: Preset, do_cigar: bool, do_cs: bool, do_md: bool) -> PyResult<Self> {
        let preset_enum: RustPreset = preset.into();
        
        let mut buf = Vec::new();
        index.inner.save_part(&mut buf).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        let mut cursor = std::io::Cursor::new(buf);
        let mut inner = RustAligner::from_index_reader(&mut cursor, preset_enum)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        
        {
            let cfg = inner.output_config_mut();
            cfg.do_cigar = do_cigar;
            cfg.do_cs = do_cs;
            cfg.do_md = do_md;
        }

        Ok(Aligner { inner })
    }

    /// Create an aligner from a FASTA file.
    ///
    /// Args:
    ///     path (os.PathLike): The file path to the FASTA file.
    ///     preset (Preset): The preset configuration (e.g. `Preset.MapOnt`). Defaults to `Preset.MapOnt`.
    ///
    /// Returns:
    ///     Aligner: The initialized aligner object.
    #[staticmethod]
    #[pyo3(signature = (path, preset=Preset::MapOnt))]
    fn from_fasta(path: PathBuf, preset: Preset) -> PyResult<Self> {
        let path_str = path.to_str().ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Invalid path string"))?;
        let preset_enum: RustPreset = preset.into();
        match RustAligner::from_fasta(path_str, preset_enum) {
            Ok(inner) => Ok(Aligner { inner }),
            Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e.to_string())),
        }
    }

    /// Create an aligner from an index file.
    ///
    /// Args:
    ///     path (os.PathLike): The file path to the saved index file.
    ///     preset (Preset): The preset configuration (e.g. `Preset.MapOnt`). Defaults to `Preset.MapOnt`.
    ///
    /// Returns:
    ///     Aligner: The initialized aligner object.
    #[staticmethod]
    #[pyo3(signature = (path, preset=Preset::MapOnt))]
    fn from_index(path: PathBuf, preset: Preset) -> PyResult<Self> {
        let path_str = path.to_str().ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Invalid path string"))?;
        let preset_enum: RustPreset = preset.into();
        match RustAligner::from_index(path_str, preset_enum) {
            Ok(inner) => Ok(Aligner { inner }),
            Err(e) => Err(pyo3::exceptions::PyIOError::new_err(e.to_string())),
        }
    }

    /// Maps a single query sequence sequentially to the targets.
    /// 
    /// Args:
    ///     query_name (bytes): The name of the query sequence.
    ///     query_seq (bytes): The query sequence.
    /// 
    /// Returns:
    ///     MappingIterator: An iterator over the generated mappings.
    fn map(&self, query_name: &Bound<'_, PyBytes>, query_seq: &Bound<'_, PyBytes>) -> MappingIterator {
        let query_name_str = String::from_utf8_lossy(query_name.as_bytes()).to_string();
        let map_result = self.inner.map_seq(&query_name_str, query_seq.as_bytes());
        
        MappingIterator {
            iter: map_result.mappings.into_iter()
        }
    }

    /// Performs highly parallelized batch alignments mapping over multiple queries.
    /// 
    /// Bypasses the GIL to utilize multiple threads for parallelism (via Rayon).
    /// 
    /// Args:
    ///     queries (list[tuple[bytes, bytes]]): A list of tuples containing `(name, sequence)` as bytes.
    /// 
    /// Returns:
    ///     list[MappingIterator]: A list of iterators, one for each query sequence.
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
        unsafe impl Send for RawQuery {}
        unsafe impl Sync for RawQuery {}

        let mut raw_queries = Vec::with_capacity(queries.len());
        for (name, seq) in queries {
            raw_queries.push(RawQuery {
                name_ptr: name.as_bytes().as_ptr(),
                name_len: name.as_bytes().len(),
                seq_ptr: seq.as_bytes().as_ptr(),
                seq_len: seq.as_bytes().len(),
            });
        }
        
        // Release the GIL via `detach` to allow other Python threads to execute concurrently.
        let iterators: Vec<MappingIterator> = py.detach(|| {
            // Utilize `rayon` to spawn multiple worker threads mapping in parallel.
            raw_queries.par_iter().map(|raw_q| {
                // Safety: Reconstructing the slice is safe because we know the pointer is valid
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

    /// Load splice junctions from a BED file.
    /// 
    /// Args:
    ///     path (os.PathLike | str): Path to the BED file.
    #[pyo3(signature = (path))]
    fn load_junctions_bed(&mut self, path: PathBuf) -> PyResult<()> {
        let path_str = path.to_str().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Invalid path for BED file")
        })?;
        self.inner.load_junctions_bed(path_str).map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Failed to load junctions: {}", e))
        })
    }

    /// Load splice junctions from a SPSC file.
    /// 
    /// Args:
    ///     path (os.PathLike | str): Path to the SPSC file.
    ///     scale (float | None): Optional scaling factor.
    #[pyo3(signature = (path, scale=None))]
    fn load_junctions_spsc(&mut self, path: PathBuf, scale: Option<f32>) -> PyResult<()> {
        let path_str = path.to_str().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Invalid path for SPSC file")
        })?;
        self.inner.load_junctions_spsc(path_str, scale).map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Failed to load junctions: {}", e))
        })
    }

    /// Returns the sequence names in the index.
    ///
    /// Returns:
    ///     list[str]: The list of sequence names.
    #[getter]
    fn seq_names(&self) -> Vec<String> {
        self.inner.index().seqs.iter().map(|s| s.name.clone()).collect()
    }

    /// Returns the sequence for a given target name.
    ///
    /// Args:
    ///     name (str): The name of the target sequence.
    ///     start (int, optional): The 0-based start coordinate. Defaults to 0.
    ///     end (int, optional): The 0-based end coordinate. Defaults to the end of the sequence.
    ///
    /// Returns:
    ///     str: The requested sequence.
    ///
    /// Raises:
    ///     ValueError: If the sequence name is not found in the index.
    #[pyo3(signature = (name, start=None, end=None))]
    fn seq(&self, name: &str, start: Option<usize>, end: Option<usize>) -> PyResult<String> {
        let index = self.inner.index();
        let rid = index.seqs.iter().position(|s| s.name == name)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(format!("Sequence not found: {}", name)))?;
        
        let seq_len = index.seqs[rid].len;
        let s = start.unwrap_or(0).min(seq_len);
        let e = end.unwrap_or(seq_len).min(seq_len);
        
        if s >= e {
            return Ok(String::new());
        }
        
        let nt4 = index.get_region_nt4(rid, s, e);
        let ascii: String = nt4.into_iter()
            .map(|b| RustIndex::NT4_TO_ASCII[b as usize] as char)
            .collect();
            
        Ok(ascii)
    }
}



pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    if let Err(e) = m.add_class::<Preset>() { println!("Error adding Preset: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<Strand>() { println!("Error adding Strand: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<CigarOp>() { println!("Error adding CigarOp: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<CigarElement>() { println!("Error adding CigarElement: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<Index>() { println!("Error adding Index: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<Aligner>() { println!("Error adding Aligner: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<Mapping>() { println!("Error adding Mapping: {:?}", e); return Err(e); }
    if let Err(e) = m.add_class::<MappingIterator>() { println!("Error adding MappingIterator: {:?}", e); return Err(e); }

    Ok(())
}
