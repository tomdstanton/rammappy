import pytest
from rammappy import Index, Aligner, Preset

def test_aligner_options(tmp_path):
    fasta = tmp_path / "test.fa"
    fasta.write_text(">seq1\nACGTACGTACGTACGT\n")
    
    idx = Index.build([(b"seq1", b"ACGTACGTACGTACGT")])
    aligner = Aligner(idx, preset=Preset.MapOnt)
    
    opts = aligner.options
    assert opts.seeding.min_mid_occ > 0
    
    # Mutate options
    s = opts.seeding
    s.min_mid_occ = 42
    opts.seeding = s
    
    c = opts.chaining
    c.min_cnt = 5
    opts.chaining = c
    
    aligner.options = opts
    
    opts2 = aligner.options
    assert opts2.seeding.min_mid_occ == 42
    assert opts2.chaining.min_cnt == 5

