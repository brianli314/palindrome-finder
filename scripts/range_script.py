import pandas
import sys
import os

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", path_name), sep='\t')

def num_in_range(start, end):
    counter = 0
    for line in df["Length"]:
        if start <= line < end:
            counter += 1
    print(counter)

num_in_range(60, 80)
num_in_range(80, 100)
num_in_range(100, 150)
num_in_range(150, 200)
num_in_range(200, sys.maxsize)