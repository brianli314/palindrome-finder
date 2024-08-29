import pandas
import sys
import os

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", path_name), sep='\t')

def mask_repeats():
    df_filtered = df[df["Sequence"].apply(has_more_uppercase)]
    df_filtered.to_csv(os.path.join("..", "outputs", "filtered_output.tsv"), sep='\t', index=False)

def has_more_uppercase(sequence):
    if not isinstance(sequence, str):
        raise Exception("Invalid sequence")
    capital_count = sum(1 for char in sequence if char.isupper())
    if len(sequence) == 0:
        return True
    return capital_count / len(sequence) >= 0.5

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
    filtered_df.to_csv(os.path.join("..", "outputs", "filtered_output.tsv"), sep='\t', index=False)

filter_lower()