import os
import pandas
import sys
import matplotlib.pyplot as plt
import argparse

parser = argparse.ArgumentParser(
    description="Plots gap length to frequency and gap length to average palindrome length"
)
parser.add_argument('-i', '--input', required=True, help="File input path")

args = parser.parse_args()
input_name = args.input

df = pandas.read_csv(input_name, sep='\t')

#Shows frequency of gap sizes
def gap_plot():
    gap = df["Gap(Approx)"]
    output = {}
    for line in gap:
        if line in output:
            output[line] += 1
        else:
            output[line] = 1

    plt.bar(list(output.keys()), list(output.values()))
    plt.title("Gap length to frequency")
    plt.xlabel("Gap length")
    plt.ylabel("Frequency of palindrome with this gap")
    plt.show()
    return output

#Shows average length of gap sizes
def gap_plot_length():
    gap = df["Gap(Approx)"]
    length = df["Length"]
    output = {}
    counts = {}
    for i in range(len(length)):
        index = gap[i]
        if index in output:
            output[index] += length[i] - gap[i]
            counts[index] += 1
        else:
            output[index] = length[i] - gap[i]
            counts[index] = 1
    averages = {key: output[key] / counts[key] for key in output}
    plt.bar(list(averages.keys()), list(averages.values()))
    plt.title("Gap length to average length")
    plt.xlabel("Gap length")
    plt.ylabel("Average length of palindrome")
    plt.show()
    return averages

if __name__ == '__main__':
    gap_plot()
    gap_plot_length()