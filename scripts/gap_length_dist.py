import os
import pandas
import sys

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..",path_name), sep='\t')

def gap_length():
    gap = df["Gap(Approx)"]
    output = {}
    for line in gap:
        if line in output:
            output[line] += 1