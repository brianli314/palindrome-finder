use crate::command_line::PalinArgs;
use crate::output::BUFF_SIZE;
use anyhow::{anyhow, bail, Ok, Result};
use flate2::read::GzDecoder;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Read},
    mem,
};

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
}

impl<T: Read> Iterator for FastaIterator<T> {
    type Item = Result<Fasta>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut seq = String::new();

        for line in self.lines_reader.by_ref() {
            let line = match line {
                Result::Ok(line) => line,
                Err(err) => return Some(Err(anyhow!("Invalid line/file format: {err}"))),
            };

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
            //Checks for valid starting line in fasta
            } else if !self.curr_name.is_empty() {
                seq += &line;
            } else {
                return Some(Err(anyhow!("Invalid fasta format")));
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
    pub fn new(bufreader: BufReader<T>) -> Self {
        Self {
            lines_reader: bufreader.lines(),
            curr_name: String::new()
        }
    }
}

pub fn parse_fasta(args: &PalinArgs) -> Result<FastaIterator<Box<dyn Read>>> {
    let reader = get_reader(args)?;
    Ok(FastaIterator::new(reader))
}

pub fn get_reader(args: &PalinArgs) -> Result<BufReader<Box<dyn Read>>> {
    let file = File::open(&args.input_file)?;

    if args.fgz {
        Ok(BufReader::with_capacity(
            BUFF_SIZE,
            Box::new(GzDecoder::new(file)),
        ))
    } else if args.fa {
        return Ok(BufReader::with_capacity(BUFF_SIZE, Box::new(file)));
    } else {
        bail!("Invalid file format input")
    }
}
