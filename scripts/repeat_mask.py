import pandas
import sys
import os

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", path_name), sep='\t')

def mask_repeats():
    df_filtered = df[df["Sequence"].apply(has_more_uppercase)]
    df_filtered.to_csv("../data/filtered_path.tsv", sep='\t', index=False)

def has_more_uppercase(sequence):
    if not isinstance(sequence, str):
        raise Exception("Invalid sequence")
    capital_count = sum(1 for char in sequence if char.isupper())
    return capital_count / len(sequence) >= 0.5

print(has_more_uppercase(df["Sequence"][0]))