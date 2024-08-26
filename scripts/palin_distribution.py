import pandas
import sys
import os
import matplotlib.pyplot as plt
import num_palins

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", path_name), sep='\t')

def get_ratio(bin_size):
    end = df["End"]
    length = df["Length"]
    end_length = len(end)
    bin_indices = []
    bin_lengths = []
    for i in range(end[end_length - 1] // bin_size + 1):
        bin_indices.append(i)
        bin_lengths.append(num_palins.num_bps(i*bin_size, (i+1)*bin_size))

    return [bin_indices, bin_lengths]
    plt.plot(bin_indices, bin_lengths)
    plt.xlabel(f"Location on sequence (every {bin_size})")
    plt.ylabel("Num base pairs")
    plt.show()


    