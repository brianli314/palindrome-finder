import pandas
import sys
import os
import matplotlib.pyplot as plt
import num_palins
import argparse

parser = argparse.ArgumentParser(
    description="Plots a graph of the number of base pairs that are involved in a palindrome across the chromosome. Grouped by bins"
)
parser.add_argument('-i', '--input', required=True, help="File input path")
parser.add_argument('-b', "--bin", type=int, default=100000, help="Bin size for grouping.")

args = parser.parse_args()
input_name = args.input
binsize = args.bin

df = pandas.read_csv(input_name, sep='\t')

#Plots the distribution of palindromes across the genome
def get_ratio(bin_size):
    end = df["End"]
    end_length = len(end)
    bins = {}

    for i in range(end[end_length - 1] // bin_size + 1):
        bins[i] = num_palins.num_bps(i*bin_size, (i+1)*bin_size)

    plt.plot(bins.keys(), bins.values())
    plt.title("Distribution on chromosome")
    plt.xlabel(f"Location on sequence (every {bin_size} bp)")
    plt.ylabel("Num base pairs involved in a palindrome ")
    plt.show()
    return bins

if __name__ == '__main__':
    get_ratio(binsize)

    