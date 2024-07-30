use crate::command_line::WfaCommand;
use crate::command_line::{PalinArgs, AlgorithmType::ExactMatch, AlgorithmType::Wfa};
use crate::exact_matches::match_exact;
//use crate::exact_matches::match_exact;
use crate::myers::wfa_palins;
use crate::output::PalindromeData;
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
    filter: String
}

impl Iterator for FastaIterator {
    type Item = Fasta;

    fn next(&mut self) -> Option<Self::Item> {
        let mut seq = String::new();
        let mut right_chromosome = false;
        for line in self.lines_reader.by_ref() {
            let line = line.expect("Failed to read from fasta!");
            
            if line.contains(&self.filter) {
                right_chromosome = true;
            } else if line.starts_with('>') && !line.contains(&self.filter) {
                right_chromosome = false;
            }

            if right_chromosome {
                if line.starts_with('>') {
                    let mut name = line.strip_prefix('>').unwrap().to_owned();
                    if seq.is_empty() {
                        name.clone_into(&mut self.curr_name);
                        continue;
                    }
                    mem::swap(&mut name, &mut self.curr_name);
                    return Some(Fasta {
                        name,
                        sequence: seq,
                    });
                } else {
                    seq += &line;
                }
            }
        }
        if seq.is_empty() {
            None
        } else {
            Some(Fasta {
                name: mem::take(&mut self.curr_name),
                sequence: seq,
            })
        }
    }
}

impl FastaIterator {
    pub fn new(bufreader: BufReader<File>, filter: String) -> Self {
        Self {
            lines_reader: bufreader.lines(),
            curr_name: String::new(),
            filter
        }
    }
}

pub fn parse_fasta(args: &PalinArgs) -> Vec<PalindromeData> {
    let file = match File::open(&args.input_file) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    let reader = BufReader::new(file);
    let mut output = Vec::new();
    let iterator = FastaIterator::new(reader, args.filter.clone());

    match &args.command{
        Wfa(cmds) => run_wfa(args, cmds, iterator, &mut output),
        ExactMatch => run_exact_match(args, iterator, &mut output),
    }
    output
}

fn run_wfa(args: &PalinArgs, cmds: &WfaCommand, iterator: FastaIterator, output: &mut Vec<PalindromeData>){
    let mut palins= Vec::new();
    for line in iterator{
        wfa_palins(line, output, args, cmds);
        output.append(&mut palins);
        palins.clear();
    }
}

fn run_exact_match(args: &PalinArgs, iterator: FastaIterator, output: &mut Vec<PalindromeData>){
    let mut palins = Vec::new();
    for line in iterator {
        match_exact(line, output, args);
        output.append(&mut palins);
        palins.clear();
    }
}
