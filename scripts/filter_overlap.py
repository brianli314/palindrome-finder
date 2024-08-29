import os
import pandas
import sys
import argparse 

parser = argparse.ArgumentParser(
    description="Filters out overlapping palindromes"
)
parser.add_argument('-i', '--input', required=True, help="File input path")
parser.add_argument('-o', '--output', required=True, help="File output path")

parser.add_argument('-r', "--ratio", type=float, default=0.9, help="The min percentage of overlap with another sequence in order for it to be filtered out")
args = parser.parse_args()

input_name = args.input
output_name = args.output
ratio = args.ratio

df = pandas.read_csv(input_name, sep='\t')

#Computes overlap
def calculate_overlap(start1, end1, start2, end2):
    overlap_start = max(start1, start2)
    overlap_end = min(end1, end2)
    overlap = max(0, overlap_end - overlap_start)
    
    len1 = end1 - start1
    len2 = end2 - start2
    overlap_percent = overlap / min(len1, len2)
    
    return overlap_percent

#Filters out overlapping palindromes (> 90% overlap)
def filter_overlap():
    start = df["Start"]
    end = df["End"]

    filtered_indices = []
    prev_start = start[0]
    prev_end = end[0]
    filtered_indices.append(0)

    for i in range(1, len(start)):
        current_start = start[i]
        current_end = end[i]
        
        overlap = calculate_overlap(prev_start, prev_end, current_start, current_end)
        
        if ratio > 0.9:
            continue
        else:
            filtered_indices.append(i)
            prev_start = current_start
            prev_end = current_end

    filtered_df = df.iloc[filtered_indices]
    filtered_df.to_csv(output_name, sep='\t', index=False)

if __name__ == '__main__':
    filter_overlap()