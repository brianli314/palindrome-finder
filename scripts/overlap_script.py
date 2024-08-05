import pandas
import sys
import os

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", path_name), sep='\t')

def get_overlap():
    start = df["Start"]
    end = df["End"]
    counter = 0
    for i in range(len(start)):
        j = i+1
        while j < len(end) and end[i] > start[j]:
            counter += end[i] - start[j] + 1
            j += 1
    print(counter)
    
get_overlap()