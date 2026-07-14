import rammappy


def test_minimizer_sketcher():
    seq = b"ACGTACGTACGTACGTACGTACGT"
    sketcher = rammappy.MinimizerSketcher(k=5, w=5)
    seeds = sketcher.sketch(seq)
    
    assert len(seeds) > 0, "Should find at least one seed"
    assert hasattr(seeds[0], 'x') and hasattr(seeds[0], 'y'), "Seed should have x and y attributes"

def test_syncmer_sketcher():
    seq = b"ACGTACGTACGTACGTACGTACGT"
    sketcher = rammappy.SyncmerSketcher(k=5, s=3)
    seeds = sketcher.sketch(seq)
    
    assert len(seeds) > 0, "Should find at least one seed"

def test_zero_copy_threaded_batch_alignment():
    target = b"ACGT" * 100
    queries = [
        (b"query1", b"ACGT" * 20),
        (b"query2", b"CGTA" * 20),
        (b"query3", b"GTAC" * 20),
    ]

    aligner = rammappy.Aligner([(b"contig1", target)], preset=rammappy.Preset.Sr)
    
    # Call map_batch which drops the GIL and processes in parallel via rayon
    batch_results = aligner.map_batch(queries)
    
    assert len(batch_results) == 3, "Should return results for all 3 queries"
    
    for i, mappings in enumerate(batch_results):
        first = next(iter(mappings), None)
        assert first is not None, f"Query {i+1} should have at least one hit"
        assert first.target_name == b"contig1", "Hit should be on contig1"
        assert first.score > 0, "Hit score should be positive"
