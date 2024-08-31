# DNA palindrome finder
This tool is designed to identify and extract palindromic sequences and data from a DNA sequence. 

A palindrome or inverted repeat in DNA is a sequence of nucleotides that is followed by its complement sequence.
This command-line tool uses fast sequence alignment algorithms coded in rust to locate such palindromes within a sequence.

It also contains a few scripts that are useful for analyzing the output data.

## Getting started
### Installation
Download the binary in the releases that match your OS and architecture.

This tool only contains binaries for windows and linux. To generate a binary for another system,
install [Rust](https://www.rust-lang.org/tools/install) and clone the repository.
From the terminal, run 
```
cargo build --release
```
Inside the 'target/release' folder, there will be an executable which you can run.

### Scripts
To run the scripts, the repository must be cloned

Make sure you have a python interpreter installed and a virtual enviornment added.

Then, install the libraries via
```
pip install -r requirements.txt
```
The scripts can now be run.

## Algorithm
This tool has two algorithms
### Fixed mismatch
This is the most common type of algorithm found on the internet for palindrome search.
It scans the sequence base by base, and upon finding a complement it extends outwards in both directions, sometimes allowing a fixed number of mismatches.
This does not account for indels, and is also slower. However, it is useful for comparing this program with existing tools

### WFA
The better alternative utilizes the wave-finding algorithm. 
This is a very fast sequence alignment algorithm to locate palindromes.
It accounts for indels and mismatches, and uses multiple pruning methods (rather than a fixed number of mismatches)

### Example usage
```
./palindrome-finder --input input.fa --fa --output results.tsv --length 10 --gap 5 fixed-mismatch --mismatch 4
./palindrome-finder --input input.fa.gz --fgz --output results.tsv --length 10 --gap 5 wfa --mismatch-proportion 0.05
```
This will find palindromes with a minimum arm length of 10, a max gap length of 5.

## Output
The output is a TSV file containing the palindromes found, containing the following 8 statistics
```
Start  End  Approx Arm-Length  Approx Gap  Length  Mismatches  Seq-name  Sequence
...
```

## Scripts
Three utility scripts have been provided for ease of use:

1. plot_graph -- Plots a graph with statistics from the output file
2. print_statistic -- Prints a desired statistic
3. filter_output -- Filters the output to only include certain palindromes

### Example usage
```
python plot_graph.py --input results.tsv --length
```
This will plot a graph of length to frequency using the output from the algorithm

## Acknowledgements
This project would not have been possible without the support of many people. I would like to thank
- Prof. Heng Li, my mentor who guided and helped with this project throughout.
- Ying Zhou, a member of the Li lab who contributed greatly to data analysis.
- Aaron Li, my brother who gave extremely helpful advice throughout.

