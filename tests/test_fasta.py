import pytest
import rammappy
from pathlib import Path


def test_record():
    r = rammappy.Record("seq1", "desc1", b"ACGT", b"IIII")
    assert r.name == "seq1"
    assert r.description == "desc1"
    assert r.sequence == b"ACGT"
    assert r.quality == b"IIII"


def test_fastx_reader(tmp_path: Path):
    fa_file = tmp_path / "test.fa"
    fa_file.write_text(">seq1\nACGT\n>seq2\nGGGG")

    reader = rammappy.FastxReader(str(fa_file))
    records = list(reader)

    assert len(records) == 2
    assert records[0].name == "seq1"
    assert records[0].sequence == b"ACGT"
    assert records[0].quality is None

    assert records[1].name == "seq2"
    assert records[1].sequence == b"GGGG"
    assert records[1].quality is None


def test_read_fasta(tmp_path: Path):
    fa_file = tmp_path / "test.fa"
    fa_file.write_text(">seq1\nACGT\n>seq2\nGGGG")

    seqs = rammappy.read_fasta(str(fa_file))
    assert len(seqs) == 2
    assert seqs[0] == ("seq1", b"ACGT")
    assert seqs[1] == ("seq2", b"GGGG")


def test_parse_fasta_bytes():
    data = b">seq1\nACGT\n>seq2\nGGGG\n"
    seqs = rammappy.parse_fasta_bytes(data)
    assert len(seqs) == 2
    assert seqs[0] == ("seq1", b"ACGT")
    assert seqs[1] == ("seq2", b"GGGG")


def test_fasta_streamer():
    streamer = rammappy.FastaStreamer(rna_to_dna=False)
    streamer.push(b">seq1\nACGT\n>seq2\n")

    # First record should be ready
    r1 = streamer.next_record()
    assert r1 is not None
    assert r1 == ("seq1", b"ACGT")

    # Second is not ready yet
    assert streamer.next_record() is None

    # Push the rest
    streamer.push(b"GGGG\n")
    streamer.finalize()

    r2 = streamer.next_record()
    assert r2 is not None
    assert r2 == ("seq2", b"GGGG")


def test_fastq_streamer():
    streamer = rammappy.FastqStreamer(rna_to_dna=False)
    streamer.push(b"@r1\nACGT\n+\n!!!!\n@r2\nG")

    r1 = streamer.next_record()
    assert r1 is not None
    assert r1 == ("r1", b"ACGT")

    assert streamer.next_record() is None

    streamer.push(b"GGG\n+\nIIII\n")
    streamer.finalize()

    r2 = streamer.next_record()
    assert r2 is not None
    assert r2 == ("r2", b"GGGG")
