import os
import pandas
import sys

large_path = "C:/Users/brian/CS projects/VScode Projects/Rust/Palindromes"
script, path_name, length = sys.argv

df = pandas.read_csv(os.path.join(large_path, path_name), sep='\t')

def get_num_palins(length):
    column = df["Length"]
    counter = 0
    for i in range(int(length), 50):
        for line in column:
            if line == i:
                counter += 1


        print(counter)
        counter = 0

    for line in column:
        if line >= 50:
            counter += 1
    print(counter)

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

def get_ratio(bin_size):
    start = df["Start"]
    end = df["End"]
    length = df["Length"]
    bins = {}
    for i in range(len(start)):
        bin_index = start[i] // bin_size
        bins[bin_index] = bins.get(bin_index, 0) + length[i]

    for bin_index, total_length in bins.items():
        print(f"{bin_index}\t{total_length}")
    

get_overlap()
