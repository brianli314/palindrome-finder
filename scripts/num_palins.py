import pandas
import sys
import os

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", path_name), sep='\t')

def num_palins():
    print(len(df["Length"]))

def num_bps(start_pt, end_pt):
    start = df["Start"]
    end = df["End"]
    length = df["Length"]
    end_pt = min(end_pt, end[len(end) - 1])
    filtered_start = start[start >= start_pt]
    start_index = filtered_start.idxmin()
    index = 0
    counter = 0
    for i in range(start_index, len(length)):
        if start[i] >= end_pt:
            break
        if start[i] < start_pt:
            continue
        
        _end = min(end[i], end_pt)
        if start[i] > index:
            counter += _end - start[i] + 1
        else:
            counter += _end - index

        index = _end

    return counter


#print(num_bps(0, 500000000))
