import argparse
import sys

import pandas

def main():
    args = get_stat_parser()
    df = pandas.read_csv(args.input, sep='\t')

    if args.num:
        print(f"{len(df)} palindromes") #Number of palindromes
    elif args.numbp:
        print(f"{num_bps(df)} base pairs in palindromes")
    elif args.max:
        get_max(df)

def get_stat_parser():
    parser = argparse.ArgumentParser(
        description="Print a statistic"
    )
    parser.add_argument('-i', '--input', required=True, help="File input path")

    statistic_options = parser.add_mutually_exclusive_group(required=True)

    statistic_options.add_argument('--num', help="Prints the total number of palindromes", action='store_true')
    statistic_options.add_argument('--numbp', help="Prints the total number of base pairs involved in palindromes", action='store_true')
    statistic_options.add_argument('--max', help="Prints the location and length of the longest palindrome", action='store_true')
    return parser.parse_args()

#Calculates number of base pairs in a palindrome within a certain range
def num_bps(df, start_pt=0, end_pt=sys.maxsize):
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

#Gets longest palindrome
def get_max(df):
    length = df["Length"]
    start = df["Start"]
    max_index = length.idxmax()
    print(f"Longest palin: {length[max_index]} bp at pos {start[max_index]}")
    return length[max_index]

if __name__ == "__main__":
    main()