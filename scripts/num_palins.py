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

    end_pt = min(end_pt, end[len(end)-1])

    filtered_start = start[start >= start_pt]

    index = start_pt
    counter = 0 

        
    for i in range(max(0, filtered_start.idxmin() - 1), len(length)):
        if start[i] >= end_pt:
            break
        
        effective_start = max(start[i],start_pt)
        effective_end = min(end[i], end_pt)
        if effective_start > index:
            counter += effective_end - effective_start + 1
        else:
            counter += max(effective_end - index, 0)

        index = effective_end

    return counter

def num_bps_overlap():
    length = df["Length"]
    gap = df["Gap(Approx)"]
    counter = 0
    for i in range(len(length)):
        counter += length[i] - gap[i]

    print(counter)

if __name__ == "__main__":
    print(num_bps(0, 500000000000))

    
