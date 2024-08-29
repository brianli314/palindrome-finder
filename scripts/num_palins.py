import pandas
import sys
import argparse
parser = argparse.ArgumentParser(
    description="Prints the total number of palindromes and the total number of base pairs involved in palindromes"
)
parser.add_argument('-i', '--input', required=True, help="File input path")

args = parser.parse_args()
input_name = args.input

df = pandas.read_csv(input_name, sep='\t')

def num_palins():
    return len(df["Length"])

def num_bps(start_pt=0, end_pt=sys.maxsize):
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

if __name__ == "__main__":
    print(f"{num_palins()} palindromes found")
    print(f"{num_bps()} base pairs that are in palindromes")

    
