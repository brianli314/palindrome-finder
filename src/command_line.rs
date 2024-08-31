use std::fmt::Display;

use clap::{command, ArgGroup, Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(group = ArgGroup::new("file_type")
    .required(true)
    .args(&["fa", "fgz"]))]
pub struct PalinArgs {
    #[arg(short = 'l', long, default_value_t = 10)]
    ///Minimum palindrome arm length
    pub len: usize,

    #[arg(short, long = "gap", default_value_t = 3)]
    ///Maximum gap length in a palindrome
    pub gap_len: usize,

    #[arg(short, long = "input", required = true)]
    ///Input file path
    pub input_file: String,

    /// Indicates the input file should be read in Fasta format
    #[arg(long)]
    pub fa: bool,

    /// Indicates the input file should be read in compressed fasta gzip format
    #[arg(long)]
    pub fgz: bool,

    #[arg(short, long = "output", default_value = "output.tsv")]
    ///Output file path. File does not need to exist.
    pub output_file: String,

    ///Decide which algorithm should be used
    #[clap(subcommand)]
    pub command: AlgorithmType,
}

#[derive(Debug, Subcommand)]
pub enum AlgorithmType {
    ///Use WFA algorithm, allows mismatches and gaps.
    Wfa(WfaArgs),
    ///Use fixed-mismatches algorithm, only allows x mismatches rather than a ratio depending on length
    FixedMismatch(FixedArgs),
}

#[derive(Debug, Args)]
pub struct FixedArgs{
    #[arg(short = 'm', long = "mismatches", default_value_t = 4)]
    pub mismatches: u32,
}

#[derive(Debug, Args)]
pub struct WfaArgs {
    ///Bonus for matches in scoring, must be positive
    #[arg(short = 'b', long = "match", default_value_t = 1.0)]
    pub match_bonus: f32,

    ///Penalty for mismatches in scoring, must be positive (since value is subtracted)
    #[arg(short = 'p', long = "mismatch", default_value_t = 4.0)]
    pub mismatch_penalty: f32,

    ///Maximum score drop allowed before pruning
    #[arg(short, long, default_value_t = 20.0)]
    pub x_drop: f32,

    ///Max percentage of mismatches allowed in a palindrome, must be between 0 and 1
    #[arg(short = 'm', long, default_value_t = 0.05)]
    pub mismatch_proportion: f32,
}

impl Display for PalinArgs{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Len: {}\nGap: {}\n{}",
            self.len, self.gap_len, self.command
        )
    }
}

impl Display for AlgorithmType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            AlgorithmType::Wfa(cmds) => 
                write!(
                    f,
                    "Match bonus: {}\nMismatch penalty: {}\nX-drop: {}\nMax mismatch proportion: {}",
                    cmds.match_bonus, cmds.mismatch_penalty, cmds.x_drop, cmds.mismatch_proportion
            ),
            AlgorithmType::FixedMismatch(cmds) => 
                write!(
                    f,
                    "Mismatches allowed: {}",
                    cmds.mismatches
                ),
        }
        
    }
}
