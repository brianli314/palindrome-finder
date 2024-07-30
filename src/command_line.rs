
use clap::command;
use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct PalinArgs{
    #[arg(short, long, required=true)]
    ///Minimum palindrome length
    pub min_len: usize,

    #[arg(short, long, required=true)]
    ///Maximum gap length in a palindrome
    pub gap_len: usize,
    
    #[arg(short, long, required=true)]
    ///Input file, should be fasta format
    pub input_file: String,
    
    #[arg(short, long, required=true)]
    ///Output file, should be .tsv
    pub output_file: String,

    #[arg(short, long, default_value="")]
    ///Filters the Fasta file, such as one specific chromosome
    pub filter: String,

    ///Decide which algorithm should be used, defaults to WFA
    #[clap(subcommand)]
    pub command: AlgorithmType


}

#[derive(Debug, Subcommand)]
pub enum AlgorithmType{
    ///Use WFA algorithm, allows mismatches and gaps
    Wfa(WfaCommand),
    ///Use exact match algorithm, only allows perfect matches
    ExactMatch,
}

#[derive(Debug, Args)]
pub struct WfaCommand{
    ///Bonus for matches in scoring
    #[arg(short='b', long, default_value_t = 1.0)]
    pub match_bonus: f32,
    ///Penalty for mismatches in scoring
    #[arg(short='p', long, default_value_t = -1.5)]
    pub mismatch_penalty: f32,
    ///Maximum score drop after enough mismatches, depends on match/mismatch values
    #[arg(short, long, default_value_t = 2.0)]
    pub x_drop: f32,
    ///Maximum ratio of mismatches to length of palindrome
    #[arg(short='r', long, default_value_t = 0.3)]
    pub mismatch_len_ratio: f32

}