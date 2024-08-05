import pandas
import sys
import os

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", path_name), sep='\t')

def get_max():
    max_val = 0
    max_index = 31
    length = df["Length"]
    for i in range(len(length)):
        if length[i] > max_val:
            max_index = i
            max_val = length[i]

    print(max_index, max_val)

get_max()
