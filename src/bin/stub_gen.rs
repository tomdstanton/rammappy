fn main() -> pyo3_stub_gen::Result<()> {
    let info = _rammappy::stub_info()?;
    info.generate()?;
    Ok(())
}
