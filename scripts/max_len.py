import pandas
import sys
import os
import argparse

parser = argparse.ArgumentParser(
    description="Finds the longest palindrome"
)
parser.add_argument('-i', '--input', required=True, help="File input path")

args = parser.parse_args()
input_name = args.input

df = pandas.read_csv(input_name, sep='\t')

def get_max():
    length = df["Length"]
    start = df["Start"]
    max_index = length.idxmax()
    print(f"Longest palin: {length[max_index]} at pos {start[max_index]}")
    return length[max_index]

if __name__ == '__main__':
    get_max()
