import pandas
import sys
import os
import matplotlib.pyplot as plt

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", path_name), sep='\t')

def get_ratio(bin_size):
    start = df["Start"]
    length = df["Length"]
    bin_indices = []
    bin_lengths = []
    for i in range(len(start)):
        bin_index = start[i] // bin_size
        if bin_index in bin_indices:
            index = bin_indices.index(bin_index)
            bin_lengths[index] += length[i]
        else:
            bin_indices.append(bin_index)
            bin_lengths.append(length[i])

    plt.plot(bin_indices, bin_lengths)
    plt.xlabel(f"Location on sequence (every {bin_size})")
    plt.ylabel("Num base pairs")
    plt.show()

get_ratio(10)
    