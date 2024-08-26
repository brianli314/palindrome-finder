import pandas
import sys
import os
import matplotlib.pyplot as plt
import num_palins

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", path_name), sep='\t')

def get_ratio(bin_size):
    end = df["End"]
    end_length = len(end)
    bins = {}

    for i in range(end[end_length - 1] // bin_size + 1):
        bins[i] = num_palins.num_bps(i*bin_size, (i+1)*bin_size)

    plt.plot(bins.keys(), bins.values())
    plt.title("Location of sequence to number of base pairs")
    plt.xlabel(f"Location on sequence (every {bin_size} bp)")
    plt.ylabel("Num base pairs involved in a palindrome ")
    plt.show()
    return bins

get_ratio(100000)

    