use std;
use rust_htslib::bam;
use mosdepth_genome_coverage_estimators::*;
use bam_generator::*;

use rust_htslib::bam::record::Cigar;


pub fn contig_coverage<R: NamedBamReader,
                       G: NamedBamReaderGenerator<R>>(
    bam_readers: Vec<G>,
    print_stream: &mut std::io::Write,
    coverage_estimators: &mut Vec<CoverageEstimator>,
    print_zero_coverage_contigs: bool,
    flag_filtering: bool) {

    for mut bam_generator in bam_readers {
        let mut bam_generated = bam_generator.start();

        let stoit_name = &(bam_generated.name().to_string());
        let mut record: bam::record::Record = bam::record::Record::new();
        let mut last_tid: i32 = -2; // no such tid in a real BAM file
        let mut ups_and_downs: Vec<i32> = Vec::new();
        let header = bam_generated.header().clone();
        let target_names = header.target_names();

        let mut num_mapped_reads: u64 = 0;

        let mut process_previous_contigs = |last_tid, tid,
        coverage_estimators: &mut Vec<CoverageEstimator>,
        ups_and_downs| {
            if last_tid != -2 {
                for estimator in coverage_estimators.iter_mut() {
                    estimator.add_contig(&ups_and_downs)
                }
                let coverages: Vec<f32> = coverage_estimators.iter_mut()
                    .map(|estimator| estimator.calculate_coverage(0)).collect();
                if print_zero_coverage_contigs ||
                    coverages.iter().any(|&coverage| coverage > 0.0) {
                        print_contig(
                            stoit_name, std::str::from_utf8(target_names[last_tid as usize]).unwrap(),
                            print_stream);
                        for (coverage, mut estimator) in coverages.iter().zip(coverage_estimators.iter_mut()) {
                            estimator.print_coverage(
                                &stoit_name,
                                std::str::from_utf8(target_names[last_tid as usize]).unwrap(),
                                &coverage,
                                print_stream);
                            estimator.setup();
                        }
                        write!(print_stream, "\n");
                    }
            }
            if print_zero_coverage_contigs {
                print_previous_zero_coverage_contigs(
                    match last_tid { -2 => -1, _ => last_tid},
                    tid, stoit_name, coverage_estimators, &target_names, print_stream);
            }
        };


        // for record in records
        while bam_generated.read(&mut record).is_ok() {
            debug!("Starting with a new read.. {:?}", record);
            if flag_filtering &&
                (record.is_secondary() ||
                 record.is_supplementary() ||
                 !record.is_proper_pair()) {
                    continue;
                }
            // if reference has changed, print the last record
            let tid = record.tid();
            if tid != -1 { // if mapped
                num_mapped_reads += 1;
                // if reference has changed, print the last record
                if tid != last_tid {
                    process_previous_contigs(last_tid, tid, coverage_estimators, ups_and_downs);
                    ups_and_downs = vec![0; header.target_len(tid as u32).expect("Corrupt BAM file?") as usize];
                    debug!("Working on new reference {}",
                           std::str::from_utf8(target_names[tid as usize]).unwrap());
                    last_tid = tid;
                }

                // for each chunk of the cigar string
                debug!("read name {:?}", std::str::from_utf8(record.qname()).unwrap());
                let mut cursor: usize = record.pos() as usize;
                for cig in record.cigar().iter() {
                    debug!("Found cigar {:} from {}", cig, cursor);
                    match cig {
                        Cigar::Match(_) | Cigar::Diff(_) | Cigar::Equal(_) => {
                            // if M, X, or = increment start and decrement end index
                            debug!("Adding M, X, or = at {} and {}", cursor, cursor + cig.len() as usize);
                            ups_and_downs[cursor] += 1;
                            let final_pos = cursor + cig.len() as usize;
                            if final_pos < ups_and_downs.len() { // True unless the read hits the contig end.
                                ups_and_downs[final_pos] -= 1;
                            }
                            cursor += cig.len() as usize;
                        },
                        Cigar::Del(_) | Cigar::RefSkip(_) => {
                            // if D or N, move the cursor
                            cursor += cig.len() as usize;
                        },
                        Cigar::Ins(_) | Cigar::SoftClip(_) | Cigar::HardClip(_) | Cigar::Pad(_) => {}
                    }
                }
                debug!("At end of loop")
            }
        }

        process_previous_contigs(last_tid, target_names.len() as i32, coverage_estimators, ups_and_downs);

        info!("In sample '{}', found {} reads mapped out of {} total ({:.*}%)",
              stoit_name, num_mapped_reads,
              bam_generated.num_detected_primary_alignments(), 2,
              (num_mapped_reads * 100) as f64 / bam_generated.num_detected_primary_alignments() as f64);

        bam_generated.finish();
    }
}


fn print_previous_zero_coverage_contigs(
    last_tid: i32,
    current_tid: i32,
    stoit_name: &str,
    coverage_estimators: &Vec<CoverageEstimator>,
    target_names: &Vec<&[u8]>,
    print_stream: &mut std::io::Write) {
    let mut my_tid = last_tid + 1;
    while my_tid < current_tid {
        print_contig(stoit_name,
                     std::str::from_utf8(target_names[my_tid as usize]).unwrap(),
                     print_stream);
        for ref coverage_estimator in coverage_estimators.iter() {
            coverage_estimator.print_zero_coverage(print_stream);
        }
        write!(print_stream, "\n");
        my_tid += 1;
    };
}

fn print_contig<'a >(stoit_name: &str,
                     contig: &str,
                     print_stream: &'a mut std::io::Write) -> &'a mut std::io::Write {
    write!(print_stream, "{}\t{}",
           stoit_name,
           contig).unwrap();
    return print_stream;
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::str;

    #[test]
    fn test_one_genome_two_contigs_first_covered_no_zeros(){
        let mut stream = Cursor::new(Vec::new());
        contig_coverage(
            generate_named_bam_readers_from_bam_files(
                vec!["tests/data/7seqs.reads_for_seq1_and_seq2.bam"]),
            &mut stream,
            &mut vec!(CoverageEstimator::new_estimator_mean(0.0,0)),
            false,
            false);
        assert_eq!(
            "7seqs.reads_for_seq1_and_seq2\tgenome2~seq1\t1.2\n7seqs.reads_for_seq1_and_seq2\tgenome5~seq2\t1.2\n",
            str::from_utf8(stream.get_ref()).unwrap())
    }

    #[test]
    fn test_one_genome_two_contigs_first_covered(){
        let mut stream = Cursor::new(Vec::new());
        contig_coverage(
            generate_named_bam_readers_from_bam_files(
                vec!["tests/data/7seqs.reads_for_seq1_and_seq2.bam"]),
            &mut stream,
            &mut vec!(CoverageEstimator::new_estimator_mean(0.0,0)),
            true,
            false);
        assert_eq!(
            "7seqs.reads_for_seq1_and_seq2\tgenome1~random_sequence_length_11000\t0.0\n7seqs.reads_for_seq1_and_seq2\tgenome1~random_sequence_length_11010\t0.0\n7seqs.reads_for_seq1_and_seq2\tgenome2~seq1\t1.2\n7seqs.reads_for_seq1_and_seq2\tgenome3~random_sequence_length_11001\t0.0\n7seqs.reads_for_seq1_and_seq2\tgenome4~random_sequence_length_11002\t0.0\n7seqs.reads_for_seq1_and_seq2\tgenome5~seq2\t1.2\n7seqs.reads_for_seq1_and_seq2\tgenome6~random_sequence_length_11003\t0.0\n",
            str::from_utf8(stream.get_ref()).unwrap())
    }

    #[test]
    fn test_flag_filtering(){
        let mut stream = Cursor::new(Vec::new());
        contig_coverage(
            generate_named_bam_readers_from_bam_files(
                vec!["tests/data/1.bam"]),
            &mut stream,
            &mut vec!(CoverageEstimator::new_estimator_mean(0.0,0)),
            false,
            true);
        assert_eq!(
            "",
            str::from_utf8(stream.get_ref()).unwrap())
    }

    #[test]
    fn test_one_contig_variance(){
        let mut stream = Cursor::new(Vec::new());
        contig_coverage(
            generate_named_bam_readers_from_bam_files(
                vec!["tests/data/2seqs.reads_for_seq1.bam"]),
            &mut stream,
            &mut vec!(CoverageEstimator::new_estimator_variance(0.0,0)),
            true,
            false);
        assert_eq!(
            "2seqs.reads_for_seq1\tseq1\t0.9489489\n".to_owned()+
                "2seqs.reads_for_seq1\tseq2\t0.0\n",
            str::from_utf8(stream.get_ref()).unwrap())
    }

    #[test]
    fn test_streaming_bam_file(){
        let mut stream = Cursor::new(Vec::new());
        contig_coverage(
            vec![
                generate_named_bam_readers_from_read_couple(
                    "tests/data/7seqs.fna",
                    "tests/data/reads_for_seq1_and_seq2.1.fq.gz",
                    "tests/data/reads_for_seq1_and_seq2.2.fq.gz",
                    4,
                    None)],
            &mut stream,
            &mut vec!(CoverageEstimator::new_estimator_mean(0.0,0)),
            false,
            false);
        assert_eq!(
            "7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome2~seq1\t1.2\n7seqs.fna/reads_for_seq1_and_seq2.1.fq.gz\tgenome5~seq2\t1.2\n",
            str::from_utf8(stream.get_ref()).unwrap())
    }

    #[test]
    fn test_multiple_coverage_methods(){
        let mut stream = Cursor::new(Vec::new());
        contig_coverage(
            generate_named_bam_readers_from_bam_files(
                vec!["tests/data/2seqs.reads_for_seq1.bam"]),
            &mut stream,
            &mut vec!(
                CoverageEstimator::new_estimator_mean(0.0,0),
                CoverageEstimator::new_estimator_variance(0.0,0)
            ),
            true,
            false);
        assert_eq!(
            "2seqs.reads_for_seq1\tseq1\t1.2\t0.9489489\n".to_owned()+
                "2seqs.reads_for_seq1\tseq2\t0.0\t0.0\n",
            str::from_utf8(stream.get_ref()).unwrap())
    }

    #[test]
    fn test_julian_error(){
        let mut stream = Cursor::new(Vec::new());
        contig_coverage(
            // has unmapped reads, which caused problems with --no-flag-filter.
            generate_named_bam_readers_from_bam_files(
                vec!["tests/data/2seqs.reads_for_seq1.with_unmapped.bam"]),
            &mut stream,
            &mut vec!(CoverageEstimator::new_estimator_mean(0.0,0)),
            true,
            false);
        assert_eq!(
            "2seqs.reads_for_seq1.with_unmapped\tseq1\t1.499
2seqs.reads_for_seq1.with_unmapped\tseq2\t1.5\n",
            str::from_utf8(stream.get_ref()).unwrap())
    }

    #[test]
    fn test_trimmed_mean_bug(){
        let mut stream = Cursor::new(Vec::new());
        contig_coverage(
            generate_named_bam_readers_from_bam_files(
                vec!["tests/data/2seqs.reads_for_seq1.bam"]),
            &mut stream,
            &mut vec!(
                CoverageEstimator::new_estimator_trimmed_mean(0.0,0.05,0.0,0)
            ),
            true,
            false);
        assert_eq!(
            "2seqs.reads_for_seq1\tseq1\t0.0\n".to_owned()+
                "2seqs.reads_for_seq1\tseq2\t0.0\n",
            str::from_utf8(stream.get_ref()).unwrap())
    }

    #[test]
    fn test_multiple_outputs_one_zero_no_print_zeroes(){
        let mut stream = Cursor::new(Vec::new());
        contig_coverage(
            generate_named_bam_readers_from_bam_files(
                vec!["tests/data/2seqs.reads_for_seq1.bam"]),
            &mut stream,
            &mut vec!(
                CoverageEstimator::new_estimator_mean(0.0,0),
                // covered fraction is 0.727, so go lower so trimmed mean is 0,
                // mean > 0.
                CoverageEstimator::new_estimator_trimmed_mean(0.0,0.05,0.0,0)
            ),
            false,
            false);
        assert_eq!(
            "2seqs.reads_for_seq1\tseq1\t1.2\t0.0\n",
            str::from_utf8(stream.get_ref()).unwrap())
    }

    #[test]
    fn test_multiple_outputs_one_zero_no_print_zeroes_reverse_order(){
        let mut stream = Cursor::new(Vec::new());
        contig_coverage(
            generate_named_bam_readers_from_bam_files(
                vec!["tests/data/2seqs.reads_for_seq1.bam"]),
            &mut stream,
            &mut vec!(
                // covered fraction is 0.727, so go lower so trimmed mean is 0,
                // mean > 0.
                CoverageEstimator::new_estimator_trimmed_mean(0.0,0.05,0.0,0),
                CoverageEstimator::new_estimator_mean(0.0,0),
            ),
            false,
            false);
        assert_eq!(
            "2seqs.reads_for_seq1\tseq1\t0.0\t1.2\n",
            str::from_utf8(stream.get_ref()).unwrap())
    }

    #[test]
    fn test_contig_end_exclusion(){
        // From https://bitbucket.org/berkeleylab/metabat/issues/48/jgi_summarize_bam_contig_depths-coverage
        let mut stream = Cursor::new(Vec::new());
        contig_coverage(
            generate_named_bam_readers_from_bam_files(
                vec!["tests/data/7seqs.reads_for_seq1_and_seq2.bam"]),
            &mut stream,
            &mut vec!(
                CoverageEstimator::new_estimator_mean(0.0,75),
                // covered fraction is 0.727, so go lower so trimmed mean is 0,
                // mean > 0.
                CoverageEstimator::new_estimator_variance(0.0,75)
            ),
            false,
            false);
        assert_eq!(
            "7seqs.reads_for_seq1_and_seq2\tgenome2~seq1\t1.4117647\t1.3049262\n".to_owned()+
            "7seqs.reads_for_seq1_and_seq2\tgenome5~seq2\t1.2435294\t0.6862065\n",
            str::from_utf8(stream.get_ref()).unwrap())
    }
}
