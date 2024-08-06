import pandas
import sys
import os

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", path_name), sep='\t')

def get_max():
    length = df["Length"]
    start = df["Start"]
    max_index = length.idxmax()
    print(f"Longest palin: {length[max_index]} at pos {start[max_index]}")

get_max()
