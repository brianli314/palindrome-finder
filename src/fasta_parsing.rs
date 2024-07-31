use crate::command_line::WfaCommand;
use crate::command_line::{AlgorithmType::ExactMatch, AlgorithmType::Wfa, PalinArgs};
use crate::exact_matches::match_exact;
use crate::output::PalindromeData;
use crate::wfa::wfa_palins;
use anyhow::{anyhow, ensure, Ok, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::mem;

#[derive(Debug, Clone)]
pub struct Fasta {
    pub name: String,
    pub sequence: String,
}
impl Fasta {
    pub fn new(name: String, sequence: String) -> Self {
        Self { name, sequence }
    }
    pub fn get_sequence(&self) -> &str {
        &self.sequence
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct FastaIterator {
    lines_reader: Lines<BufReader<File>>,
    curr_name: String,
    filter: String,
}

impl Iterator for FastaIterator {
    type Item = Result<Fasta>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut seq = String::new();

        let mut filter_off = false;
        let mut is_first_line = true;
        
        for line in self.lines_reader.by_ref() {
            let line = line.expect("Failed to read from fasta!");

            if is_first_line {
                is_first_line = false;
                
                if !line.starts_with('>'){
                    return Some(Err(anyhow!("Not a fasta sequence")));
                }
            } 

            if line.contains(&self.filter) {
                filter_off = true;
            } else if line.starts_with('>') && !line.contains(&self.filter) {
                filter_off = false;
            }

            if filter_off {
                if line.starts_with('>') {
                    let mut name = line.strip_prefix('>').unwrap().to_owned();
                    if seq.is_empty() {
                        name.clone_into(&mut self.curr_name);
                        continue;
                    }
                    mem::swap(&mut name, &mut self.curr_name);

                    if name.is_empty(){
                        return Some(Err(anyhow!("Not a fasta sequence")));
                    }
                    
                    return Some(Ok(Fasta {
                        name,
                        sequence: seq,
                    }));
                } else {
                    seq += &line;
                }
            }
        }
        if seq.is_empty() {
            None
        } else {
            Some(Ok(Fasta {
                name: mem::take(&mut self.curr_name),
                sequence: seq,
            }))
        }
    }
}

impl FastaIterator {
    pub fn new(bufreader: BufReader<File>, filter: String) -> Self {
        Self {
            lines_reader: bufreader.lines(),
            curr_name: String::new(),
            filter,
        }
    }
}

pub fn parse_fasta(args: &PalinArgs) -> Result<Vec<PalindromeData>> {
    let file = File::open(&args.input_file)?;

    let reader = BufReader::new(file);
    let mut output = Vec::new();
    let iterator = FastaIterator::new(reader, args.filter.clone());

    match &args.command {
        Wfa(cmds) => run_wfa(args, cmds, iterator, &mut output)?,
        ExactMatch => run_exact_match(args, iterator, &mut output)?,
    }

    Ok(output)
}

fn run_wfa(
    args: &PalinArgs,
    cmds: &WfaCommand,
    iterator: FastaIterator,
    output: &mut Vec<PalindromeData>,
) -> Result<()> {
    ensure!(cmds.match_bonus > 0.0, "Match bonus not positive");
    ensure!(cmds.mismatch_penalty < 0.0, "Mismatch bonus not negative");
    ensure!(
        0.0 < cmds.mismatch_len_ratio && cmds.mismatch_len_ratio < 1.0,
        "Mismatch-length ratio not between 0 and 1"
    );
    ensure!(cmds.x_drop > 0.0, "X-drop not positive");

    let mut palins = Vec::new();
    for line in iterator {
        wfa_palins(line?, output, args, cmds)?;
        output.append(&mut palins);
        palins.clear();
    }

    Ok(())
}

fn run_exact_match(
    args: &PalinArgs,
    iterator: FastaIterator,
    output: &mut Vec<PalindromeData>,
) -> Result<()> {
    let mut palins = Vec::new();
    for line in iterator {
        match_exact(line?, output, args)?;
        output.append(&mut palins);
        palins.clear();
    }

    Ok(())
}
