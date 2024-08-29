import os
import pandas
import sys
import matplotlib.pyplot as plt
import numpy as np
import argparse

parser = argparse.ArgumentParser(
    description="Plots a contour plot of gap and length to frequency. Can be grouped by bin size"
)
parser.add_argument('-i', '--input', required=True, help="File input path")
parser.add_argument('-b', '--bin', type=int, default=1, help="Bin size")

args = parser.parse_args()
input_name = args.input
binsize = args.bin

df = pandas.read_csv(input_name, sep='\t')

def gap_to_length(bin_size):
    length = df["Arm-Length(Approx)"]
    gap = df["Gap(Approx)"]
    matrix = np.zeros((length.max()//bin_size +1, gap.max()//bin_size +1))
    for i in range(len(length)):
        matrix[length[i]//bin_size][gap[i]//bin_size] += 1

    plt.contourf(matrix)
    plt.title(f"Arm Length and gap to frequency (grouped every {bin_size} bases)")
    plt.ylabel(f"Arm Length")
    plt.xlabel(f"Gap length")
    plt.colorbar()
    plt.show()
    return matrix

if __name__ == '__main__':
    gap_to_length(binsize)