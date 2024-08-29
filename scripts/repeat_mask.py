import pandas
import sys
import os
import argparse 

parser = argparse.ArgumentParser(
    description="Filters out palindromes with more than x%% lowercase letters in arms"
)
parser.add_argument('-i', '--input', required=True, help="File input path")
parser.add_argument('-o', '--output', required=True, help="File output path")
parser.add_argument('-r', "--ratio", type=float, default=0.5, help="Max percentage of lowercase letters")
args = parser.parse_args()

input_name = args.input
output_name = args.output
ratio = args.ratio

df = pandas.read_csv(input_name, sep='\t')

def mask_repeats():
    df_filtered = df[df["Sequence"].apply(has_more_uppercase)]
    df_filtered.to_csv(os.path.join("..", "outputs", "filtered_output.tsv"), sep='\t', index=False)

def has_more_uppercase(sequence):
    if not isinstance(sequence, str):
        raise Exception("Invalid sequence")
    capital_count = sum(1 for char in sequence if char.isupper())
    if len(sequence) == 0:
        return True
    return capital_count / len(sequence) >= ratio

def filter_lower():
    arms = df["Arm-Length(Approx)"]
    seq = df["Sequence"]
    indices = []
    for i in range(len(arms)):
        arm = arms[i]
        first = slice(0, arm)
        second = slice(-1, -arm, -1)
        if has_more_uppercase(seq[i][first]) and has_more_uppercase(seq[i][second]):
            indices.append(i)
    filtered_df = df.iloc[indices]
    filtered_df.to_csv(output_name, sep='\t', index=False)

if __name__ == '__main__':
    filter_lower()