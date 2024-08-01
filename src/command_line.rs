use clap::command;
use clap::ArgGroup;
use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(group = ArgGroup::new("file_type")
    .required(true)
    .args(&["fa", "fgz"]))]
pub struct PalinArgs {
    #[arg(short = 'l', long = "len", default_value_t = 10)]
    ///Minimum palindrome length
    pub min_len: usize,

    #[arg(short, long = "gap", default_value_t = 3)]
    ///Maximum gap length in a palindrome
    pub gap_len: usize,

    #[arg(short, long = "input", required = true)]
    ///Input file path
    pub input_file: String,

    /// Indicates the input file should be read in FASTA format
    #[arg(long)]
    pub fa: bool,

    /// Indicates the input file should be read in compressed FASTA (gz) format
    #[arg(long)]
    pub fgz: bool,

    #[arg(short, long = "output", default_value = "output.tsv")]
    ///Output file path. File does not need to exist. Defaults to a new tsv file
    pub output_file: String,

    #[arg(short, long, default_value = "")]
    ///Filters the Fasta file, such as one specific chromosome
    pub filter: String,

    ///Decide which algorithm should be used, defaults to WFA
    #[clap(subcommand)]
    pub command: AlgorithmType,
}

#[derive(Debug, Subcommand)]
pub enum AlgorithmType {
    ///Use WFA algorithm, allows mismatches and gaps. Add in additional parameters
    Wfa(WfaArgs),
    ///Use exact match algorithm, only allows perfect palindromes. No additional parameters
    ExactMatch,
}

#[derive(Debug, Args)]
pub struct WfaArgs {
    ///Bonus for matches in scoring, must be positive
    #[arg(short = 'b', long = "match", default_value_t = 1.0)]
    pub match_bonus: f32,

    ///Penalty for mismatches in scoring, must be positive as penalty is subtracted.
    #[arg(short = 'p', long = "mismatch", default_value_t = 1.5)]
    pub mismatch_penalty: f32,

    ///Maximum score drop after enough mismatches, depends on match/mismatch values, must be positive
    #[arg(short, long, default_value_t = 2.0)]
    pub x_drop: f32,

    ///Maximum ratio of mismatches to length of palindrome, must be between 0 and 1
    #[arg(short = 'r', long, default_value_t = 0.3)]
    pub mismatch_len_ratio: f32,
}
