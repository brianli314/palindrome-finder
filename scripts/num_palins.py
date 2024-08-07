import pandas
import sys
import os

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", path_name), sep='\t')

def num_palins():
    print(len(df["Length"]))

def num_bps():
    print(sum(df["Length"]))

num_bps()