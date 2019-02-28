extern crate assert_cli;
extern crate tempfile;

#[cfg(test)]
mod tests {
    use assert_cli::Assert;
    extern crate tempfile;
    use std;

    #[test]
    fn test_filter_all_reads(){
        let tf: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        let t = tf.path().to_str().unwrap();
        Assert::main_binary()
            .with_args(&[
                "filter",
                "-b",
                "tests/data/2seqs.bad_read.1.bam",
                "-o",
                t]).succeeds().unwrap();
        Assert::command(&["samtools","view",t])
            .stdout().contains("1\t99\tseq1").unwrap();
    }

    #[test]
    fn test_filter_filter_out(){
        let tf: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        let t = tf.path().to_str().unwrap();
        Assert::main_binary()
            .with_args(&[
                "filter",
                "--min-read-percent-identity-pair",
                "0.99",
                "-b",
                "tests/data/2seqs.bad_read.1.bam",
                "-o",
                t,
                "--proper-pairs-only"]).succeeds().unwrap();
        Assert::command(&["samtools","view",t])
            .stdout().doesnt_contain("1\t99\tseq1").unwrap();
    }

    #[test]
    fn test_contig_tempdir_index_creation(){
        let tf: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        let t_full = tf.path().to_str().unwrap();
        std::fs::copy("tests/data/7seqs.fna", t_full).unwrap();
        let t = tf.path().file_name().unwrap().to_str().unwrap();
        Assert::main_binary()
            .with_args(&[
                "contig",
                "--min-read-percent-identity-pair",
                "0.95",
                "--contig-end-exclusion",
                "0",
                "-r",
                t_full,
                "-1",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "-2",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--proper-pairs-only",
            ])
            .succeeds()
            .stdout().contains(format!(
                "{}/reads_for_seq1_and_seq2.1.fq.gz\tgenome1~random_sequence_length_11000\t0
{}/reads_for_seq1_and_seq2.1.fq.gz\tgenome1~random_sequence_length_11010\t0
{}/reads_for_seq1_and_seq2.1.fq.gz\tgenome2~seq1\t1.2
{}/reads_for_seq1_and_seq2.1.fq.gz\tgenome3~random_sequence_length_11001\t0
{}/reads_for_seq1_and_seq2.1.fq.gz\tgenome4~random_sequence_length_11002\t0
{}/reads_for_seq1_and_seq2.1.fq.gz\tgenome5~seq2\t1.2
{}/reads_for_seq1_and_seq2.1.fq.gz\tgenome6~random_sequence_length_11003\t0",
                t, t, t, t, t, t, t).as_str()).unwrap();
    }

    #[test]
    #[ignore] // known failure, cannot currently take multiple references
    fn test_contig_multiple_references(){
        Assert::main_binary()
            .with_args(&[
                "contig",
                "--min-read-percent-identity-pair",
                "0.95",
                "--contig-end-exclusion",
                "0",
                "-r",
                "tests/data/7seqs.fna",
                "tests/data/2seqs.fasta",
                "-1",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "-2",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
            ])
            .succeeds()
            .stdout().contains("7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome1~random_sequence_length_11000\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome1~random_sequence_length_11010\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome2~seq1\t1.2
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome3~random_sequence_length_11001\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome4~random_sequence_length_11002\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome5~seq2\t1.2
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome6~random_sequence_length_11003\t0
2seqs.fasta/reads_for_seq1_and_seq2.1.fq.gz\tseq1\t1.2
2seqs.fasta/reads_for_seq1_and_seq2.1.fq.gz\tseq2\t1.2").unwrap();
    }

    #[test]
    #[ignore] // cannot currently take multiple references
    fn test_coupled_reads_input(){
        Assert::main_binary()
            .with_args(&[
                "contig",
                "--min-read-percent-identity",
                "0.95",
                "--contig-end-exclusion",
                "0",
                "-r",
                "tests/data/7seqs.fna",
                "tests/data/2seqs.fasta",
                "-1",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "-2",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "-c",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
            ])
            .succeeds()
            .stdout().contains("7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome1~random_sequence_length_11000\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome1~random_sequence_length_11010\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome2~seq1\t1.2
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome3~random_sequence_length_11001\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome4~random_sequence_length_11002\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome5~seq2\t1.2
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome6~random_sequence_length_11003\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome1~random_sequence_length_11000\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome1~random_sequence_length_11010\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome2~seq1\t1.2
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome3~random_sequence_length_11001\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome4~random_sequence_length_11002\t0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome5~seq2\t1.2
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome6~random_sequence_length_11003\t0
2seqs.fasta/reads_for_seq1_and_seq2.1.fq.gz\tseq1\t1.2
2seqs.fasta/reads_for_seq1_and_seq2.1.fq.gz\tseq2\t1.2
2seqs.fasta/reads_for_seq1_and_seq2.1.fq.gz\tseq1\t1.2
2seqs.fasta/reads_for_seq1_and_seq2.1.fq.gz\tseq2\t1.2").unwrap();
    }

    #[test]
    fn test_unfiltered_interleaved_input(){
        Assert::main_binary()
            .with_args(&[
                "contig",
                "--contig-end-exclusion",
                "0",
                "-r",
                "tests/data/2seqs.fasta",
                "--interleaved",
                "tests/data/bad_reads.interleaved.fq",
            ])
            .succeeds()
            .stdout().contains(
                "2seqs.fasta/bad_reads.interleaved.fq\tseq1\t0.899\n\
                 2seqs.fasta/bad_reads.interleaved.fq\tseq2\t0"
            ).unwrap();
    }

    #[test]
    fn test_filtered_interleaved_input(){
        Assert::main_binary()
            .with_args(&[
                "contig",
                "-r",
                "tests/data/2seqs.fasta",
                "--interleaved",
                "tests/data/bad_reads.all.interleaved.fa",
                "--min-read-aligned-length-pair",
                "300",
                "--contig-end-exclusion",
                "0",
                "--proper-pairs-only",
            ])
            .succeeds()
            .stdout().contains("2seqs.fasta/bad_reads.all.interleaved.fa\tseq1\t1.2
2seqs.fasta/bad_reads.all.interleaved.fa\tseq2\t1.5").unwrap();
    }

    #[test]
    fn test_single_reads_input(){
        Assert::main_binary()
            .with_args(&[
                "contig",
                "-r",
                "tests/data/2seqs.fasta",
                "--single",
                "tests/data/bad_reads.interleaved.fq",
                "--contig-end-exclusion",
                "0",
            ])
            .succeeds()
            .stdout().contains(
                "2seqs.fasta/bad_reads.interleaved.fq\tseq1\t0.899\n\
                 2seqs.fasta/bad_reads.interleaved.fq\tseq2\t0"
            ).unwrap();
    }

    #[test]
    fn test_genome_coupled_read_input_argparsing(){
        // Had trouble (because of a bug in clap? with this previously)
        Assert::main_binary()
            .with_args(&[
                "genome",
                "--coupled",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--reference",
                "tests/data/7seqs.fna",
                "-s","~"]).succeeds().unwrap();
    }

    #[test]
    fn test_contig_coupled_read_input_argparsing(){
        Assert::main_binary()
            .with_args(&[
                "contig",
                "--coupled",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--reference",
                "tests/data/7seqs.fna"]).succeeds().unwrap();
    }

    #[test]
    fn test_cache_bam_files(){
        let td = tempfile::TempDir::new().unwrap();
        Assert::main_binary()
            .with_args(&[
                "contig",
                "--coupled",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--reference",
                "tests/data/7seqs.fna",
                "--bam-file-cache-directory",
                td.path().to_str().unwrap()
            ]).succeeds().stdout().contains(
                "Sample	Contig	Mean
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome1~random_sequence_length_11000	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome1~random_sequence_length_11010	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome2~seq1	1.4117647
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome3~random_sequence_length_11001	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome4~random_sequence_length_11002	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome5~seq2	1.2435294
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome6~random_sequence_length_11003	0").unwrap();
        assert!(td.path()
                .join("7seqs.fna.reads_for_seq1_and_seq2.1.fq.gz.bam")
                .is_file());
    }

    #[test]
    fn test_non_existant_cache_bam_files(){
        Assert::main_binary()
            .with_args(&[
                "contig",
                "--coupled",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--reference",
                "tests/data/7seqs.fna",
                "--bam-file-cache-directory",
                "/no/no/no/165"
            ]).fails().unwrap();
    }

    #[test]
    fn test_unwriteable_cache_bam_files(){
        Assert::main_binary()
            .with_args(&[
                "contig",
                "--coupled",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--reference",
                "tests/data/7seqs.fna",
                "--bam-file-cache-directory",
                "/"
            ]).fails().unwrap();
    }

    #[test]
    fn test_make(){
        let td = tempfile::TempDir::new().unwrap();
        Assert::main_binary()
            .with_args(&[
                "make",
                "--coupled",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--reference",
                "tests/data/7seqs.fna",
                "--output-directory",
                td.path().to_str().unwrap()
            ]).succeeds().unwrap();
        assert!(td.path()
                .join("7seqs.fna.reads_for_seq1_and_seq2.1.fq.gz.bam")
                .is_file());
    }

    #[test]
    fn test_make_with_mkdir(){
        let td = tempfile::TempDir::new().unwrap();
        Assert::main_binary()
            .with_args(&[
                "make",
                "--coupled",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--reference",
                "tests/data/7seqs.fna",
                "--output-directory",
                format!("{}/unmade_directory",td.path().to_str().unwrap()).as_str()
            ]).succeeds().unwrap();
        assert!(td.path()
                .join("unmade_directory")
                .join("7seqs.fna.reads_for_seq1_and_seq2.1.fq.gz.bam")
                .is_file());
    }

    #[test]
    fn test_relative_abundance_all_mapped() {
        Assert::main_binary()
            .with_args(&[
                "genome",
                "--coupled",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--reference",
                "tests/data/7seqs.fna",
                "-s","~"]).succeeds().stdout().contains(
                "Sample	Genome	Relative Abundance (%)
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	unmapped	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome1	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome2	53.16792
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome3	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome4	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome5	46.832077
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome6	0"
            ).unwrap();
    }

    #[test]
    fn test_relative_abundance_and_mean() {
        Assert::main_binary()
            .with_args(&[
                "genome",
                "-m",
                "relative_abundance",
                "mean",
                "-b",
                "tests/data/7seqs.reads_for_seq1_and_seq2.bam",
                "-s",
                "~"]).succeeds().stdout().contains(
                "Sample	Genome	Relative Abundance (%)	Mean
7seqs.reads_for_seq1_and_seq2	unmapped	0	NA
7seqs.reads_for_seq1_and_seq2	genome1	0	0
7seqs.reads_for_seq1_and_seq2	genome2	53.16792	1.4117647
7seqs.reads_for_seq1_and_seq2	genome3	0	0
7seqs.reads_for_seq1_and_seq2	genome4	0	0
7seqs.reads_for_seq1_and_seq2	genome5	46.832077	1.2435294
7seqs.reads_for_seq1_and_seq2	genome6	0	0").unwrap();
    }

    #[test]
    fn test_contig_dense_output_simple() {
        Assert::main_binary()
            .with_args(&[
                "contig",
                "-b",
                "tests/data/7seqs.reads_for_seq1_and_seq2.bam",
                "--output-format",
                "dense"]).succeeds().stdout().contains(
                "Contig	7seqs.reads_for_seq1_and_seq2 Mean
genome1~random_sequence_length_11000	0
genome1~random_sequence_length_11010	0
genome2~seq1	1.4117647
genome3~random_sequence_length_11001	0
genome4~random_sequence_length_11002	0
genome5~seq2	1.2435294
genome6~random_sequence_length_11003	0").unwrap();
    }

    #[test]
    fn test_genome_dense_output_simple() {
        Assert::main_binary()
            .with_args(&[
                "genome",
                "-m",
                "relative_abundance",
                "-b",
                "tests/data/7seqs.reads_for_seq1_and_seq2.bam",
                "-s",
                "~",
                "--output-format",
                "dense"]).succeeds().stdout().contains(
                "Genome	7seqs.reads_for_seq1_and_seq2 Relative Abundance (%)
unmapped	0
genome1	0
genome2	53.16792
genome3	0
genome4	0
genome5	46.832077
genome6	0").unwrap();
    }

    #[test]
    fn test_genome_unknown_reason() {
        Assert::main_binary()
            .with_args(&[
                "genome",
                "-m",
                "relative_abundance",
                "mean",
                "variance",
                "-r",
                "tests/data/7seqs.fna",
                "-c",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--single",
                "tests/data/reads_for_seq1_and_seq2.fna",
                "-s",
                "~"]).succeeds().stdout().contains(
                "Sample	Genome	Relative Abundance (%)	Mean	Variance
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	unmapped	0	NA	NA
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome1	0	0	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome2	53.16792	1.4117647	1.3049262
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome3	0	0	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome4	0	0	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome5	46.832077	1.2435294	0.6862065
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome6	0	0	0
7seqs.fna/reads_for_seq1_and_seq2.fna	unmapped	0	NA	NA
7seqs.fna/reads_for_seq1_and_seq2.fna	genome1	0	0	0
7seqs.fna/reads_for_seq1_and_seq2.fna	genome2	53.16792	1.4117647	1.3049262
7seqs.fna/reads_for_seq1_and_seq2.fna	genome3	0	0	0
7seqs.fna/reads_for_seq1_and_seq2.fna	genome4	0	0	0
7seqs.fna/reads_for_seq1_and_seq2.fna	genome5	46.832077	1.2435294	0.6862065
7seqs.fna/reads_for_seq1_and_seq2.fna	genome6	0	0	0
").unwrap();
    }

    #[test]
    fn test_contig_unknown_reason() {
        Assert::main_binary()
            .with_args(&[
                "contig",
                "-m",
                "mean",
                "variance",
                "-r",
                "tests/data/7seqs.fna",
                "-c",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--single",
                "tests/data/reads_for_seq1_and_seq2.fna"]).succeeds().stdout().contains(
                "Sample	Contig	Mean	Variance
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome1~random_sequence_length_11000	0	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome1~random_sequence_length_11010	0	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome2~seq1	1.4117647	1.3049262
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome3~random_sequence_length_11001	0	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome4~random_sequence_length_11002	0	0
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome5~seq2	1.2435294	0.6862065
7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz	genome6~random_sequence_length_11003	0	0
7seqs.fna/reads_for_seq1_and_seq2.fna	genome1~random_sequence_length_11000	0	0
7seqs.fna/reads_for_seq1_and_seq2.fna	genome1~random_sequence_length_11010	0	0
7seqs.fna/reads_for_seq1_and_seq2.fna	genome2~seq1	1.4117647	1.3049262
7seqs.fna/reads_for_seq1_and_seq2.fna	genome3~random_sequence_length_11001	0	0
7seqs.fna/reads_for_seq1_and_seq2.fna	genome4~random_sequence_length_11002	0	0
7seqs.fna/reads_for_seq1_and_seq2.fna	genome5~seq2	1.2435294	0.6862065
7seqs.fna/reads_for_seq1_and_seq2.fna	genome6~random_sequence_length_11003	0	0
").unwrap();
    }

    #[test]
    fn test_bwa_parameters() {
        Assert::main_binary()
            .with_args(&[
                "contig",
                "-r",
                "tests/data/2seqs.fasta",
                "--single",
                "tests/data/2seqs.fasta"]).succeeds().stdout().contains(
                "Sample	Contig	Mean\n\
                 2seqs.fasta/2seqs.fasta	seq1	1\n\
                 2seqs.fasta/2seqs.fasta	seq2	1\n").unwrap();

        Assert::main_binary()
            .with_args(&[
                "contig",
                "-r",
                "tests/data/2seqs.fasta",
                "--bwa-parameters",
                "'-k 5000'", // seed length longer than both sequences
                "--single",
                "tests/data/2seqs.fasta"]).succeeds().stdout().contains(
                "Sample	Contig	Mean\n\
                 2seqs.fasta/2seqs.fasta	seq1	0\n\
                 2seqs.fasta/2seqs.fasta	seq2	0\n").unwrap();
    }

    #[test]
    fn test_metabat_include_supplementary() {
        Assert::main_binary()
            .with_args(&[
                "contig",
                "-m",
                "metabat",
                "-b",
                "tests/data/k141_7.reheadered.bam", // includes a supplementary alignment
            ]).succeeds().stdout().contains(
                "contigName	contigLen	totalAvgDepth	k141_7.reheadered.bam	k141_7.reheadered.bam-var
k141_7	350	0.69	0.69	2.0843215").unwrap();
    }


    #[test]
    fn test_metabat_97_of_100_bases_should_fail() {
        // 100 bases match, 3 mismatch. Seems metabat uses > 0.97 not >= 0.97
        Assert::main_binary()
            .with_args(&[
                "contig",
                "-m",
                "metabat",
                "-b",
                "tests/data/k141_2005182.head11.bam", // includes a supplementary alignment
            ]).succeeds().stdout().contains(
                "contigName	contigLen	totalAvgDepth	k141_2005182.head11.bam	k141_2005182.head11.bam-var
k141_2005182	225	1.9333333	1.9333333	0.063063025").unwrap();
    }

    #[test]
    fn test_deletions_count_towards_perc_id() {
        Assert::main_binary()
            .with_args(&[
                "contig",
                "-m",
                "metabat",
                "-b",
                "tests/data/k141_109815.stray_read.bam", // If D not counted, perc ID filters out
            ]).succeeds().stdout().contains(
        "contigName	contigLen	totalAvgDepth	k141_109815.stray_read.bam	k141_109815.stray_read.bam-var
k141_109815	362	0.6273585	0.6273585	0.23488776").unwrap();
    }

    #[test]
    fn test_no_zeroes_missing_column_bug_genomes_and_contigs() {
        Assert::main_binary()
            .with_args(&[
                "genome",
                "-m",
                "trimmed_mean",
                "mean",
                "covered_fraction",
                "--no-zeros",
                "-r",
                "tests/data/2seqs.fasta",
                "-c",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--genome-fasta-files",
                "tests/data/genomes_dir/seq1.fna",
                "tests/data/genomes_dir/seq2.fna",
                "--trim-max",
                "0.01",
                "--trim-min",
                "0.009"
            ]).succeeds().stdout().contains(
                "2seqs.fasta/reads_for_seq1_and_seq2.1.fq.gz	seq1	0	1.4117647	0.78705883
2seqs.fasta/reads_for_seq1_and_seq2.1.fq.gz	seq2	0	1.2435294	0.84"
            ).unwrap();
    }

    #[test]
    fn test_no_zeroes_missing_column_bug_separator() {
        Assert::main_binary()
            .with_args(&[
                "genome",
                "-m",
                "trimmed_mean",
                "mean",
                "covered_fraction",
                "--no-zeros",
                "-r",
                "tests/data/2seqs.fasta",
                "-c",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "-s",
                "q",
                "--trim-max",
                "0.01",
                "--trim-min",
                "0.009"
            ]).succeeds().stdout().contains(
                "2seqs.fasta/reads_for_seq1_and_seq2.1.fq.gz	se	0	1.3276471	0.81352943"
            ).unwrap();
    }

    #[test]
    fn test_autoconcatenation() {
        Assert::main_binary()
            .with_args(&[
                "genome",
                "-m",
                "trimmed_mean",
                "mean",
                "covered_fraction",
                "--no-zeros",
                "-c",
                "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                "--genome-fasta-files",
                "tests/data/genomes_dir/seq1.fna",
                "tests/data/genomes_dir/seq2.fna",
                "--trim-max",
                "0.01",
                "--trim-min",
                "0.009"
            ]).succeeds().stdout().contains(
                "reads_for_seq1_and_seq2.1.fq.gz	seq1	0	1.4117647	0.78705883"
                //reads_for_seq1_and_seq2.1.fq.gz	seq2	0	1.2435294	0.84" //TODO: remove tmpfilename
            ).unwrap();
    }
}


// TODO: Add mismatching bases test
