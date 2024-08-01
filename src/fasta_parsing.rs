use crate::command_line::WfaArgs;
use crate::command_line::{AlgorithmType::ExactMatch, AlgorithmType::Wfa, PalinArgs};
use crate::exact_matches::match_exact;
use crate::output::PalindromeData;
use crate::wfa::wfa_palins;
use anyhow::{anyhow, bail, ensure, Ok, Result};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};
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

#[derive(Debug)]
pub struct FastaIterator<T: Read> {
    lines_reader: Lines<BufReader<T>>,
    curr_name: String,
    filter: String,
}

impl<T: Read> Iterator for FastaIterator<T> {
    type Item = Result<Fasta>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut seq = String::new();

        let mut is_filter_off = false;

        for line in self.lines_reader.by_ref() {
            
            let line = match line {
                Result::Ok(line) => line,
                Err(err) => return Some(Err(anyhow!("Invalid line/file format: {err}"))),
            };

            if line.contains(&self.filter) {
                is_filter_off = true;
            } else if line.starts_with('>') && !line.contains(&self.filter) {
                is_filter_off = false;
            }

            if is_filter_off {
                if line.starts_with('>') {
                    let mut name = line.strip_prefix('>').unwrap().to_owned();
                    if seq.is_empty() {
                        name.clone_into(&mut self.curr_name);
                        continue;
                    }
                    
                    mem::swap(&mut name, &mut self.curr_name);

                    return Some(Ok(Fasta {
                        name,
                        sequence: seq,
                    }));
                } else if !self.curr_name.is_empty(){
                    seq += &line;
                } else {
                    return Some(Err(anyhow!("Invalid fasta format")));
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

impl<T: Read> FastaIterator<T> {
    pub fn new(bufreader: BufReader<T>, filter: String) -> Self {
        Self {
            lines_reader: bufreader.lines(),
            curr_name: String::new(),
            filter,
        }
    }
}

pub fn parse_fasta(args: &PalinArgs) -> Result<Vec<PalindromeData>> {
    let mut output = Vec::new();
    let reader = get_reader(args)?;
    let iterator = FastaIterator::new(reader, args.filter.clone());

    match &args.command {
        Wfa(cmds) => run_wfa(args, cmds, iterator, &mut output)?,
        ExactMatch => run_exact_match(args, iterator, &mut output)?,
    }

    Ok(output)
}

pub fn get_reader(args: &PalinArgs) -> Result<BufReader<Box<dyn Read>>> {
    let file = File::open(&args.input_file)?;

    if args.fgz {
        Ok(BufReader::new(Box::new(GzDecoder::new(file))))
    } else if args.fa {
        return Ok(BufReader::new(Box::new(file)));
    } else {
        bail!("Invalid file format")
    }
}

fn run_wfa<T: Read>(args: &PalinArgs, wfa_args: &WfaArgs, iterator: FastaIterator<T>, output: &mut Vec<PalindromeData>) -> Result<()> {
    ensure!(wfa_args.match_bonus > 0.0, "Match bonus not positive");
    ensure!(wfa_args.mismatch_penalty > 0.0, "Mismatch penalty not positive");
    ensure!(
        0.0 < wfa_args.mismatch_len_ratio && wfa_args.mismatch_len_ratio < 1.0,
        "Mismatch-length ratio not between 0 and 1"
    );
    ensure!(wfa_args.x_drop > 0.0, "X-drop not positive");

    let mut palins = Vec::new();
    for line in iterator {
        wfa_palins(line?, output, args, wfa_args)?;
        output.append(&mut palins);
        palins.clear();
    }

    Ok(())
}

fn run_exact_match<T: Read>(args: &PalinArgs, iterator: FastaIterator<T>, output: &mut Vec<PalindromeData>, ) -> Result<()> {
    let mut palins = Vec::new();
    for line in iterator {
        match_exact(line?, output, args)?;
        output.append(&mut palins);
        palins.clear();
    }

    Ok(())
}
