import matplotlib.pyplot as plt
import argparse
import numpy as np
import pandas

from print_statistic import num_bps


def main():
    args = get_graph_parser()
    
    df = pandas.read_csv(args.input, sep='\t')

    if args.length:
        plot_length(df)
    elif args.gap:
        plot_gap(df)
    elif args.distr:
        plot_distribution(100000,df)
    elif args.heatmap:
        plot_heatmap(df)


def get_graph_parser():
    parser = argparse.ArgumentParser(
        description="Plot a graph"
    )
    parser.add_argument('-i', '--input', required=True, help="File input path")

    graph_options = parser.add_mutually_exclusive_group(required= True)
    graph_options.add_argument('--length', help="Plots palindrome length to frequency", action='store_true')
    graph_options.add_argument('--gap', help="Plots gap length to frequency", action='store_true')
    graph_options.add_argument('--heatmap', help="Plots a heatmap of gap length and length to frequency", action='store_true')
    graph_options.add_argument('--distr', help="Plots palindrome distribution (number of bps in a palindrome per 100 kb)", action='store_true')

    return parser.parse_args()

#Plots arm length to frequency
def plot_length(df):
    length = df["Arm-Length(Approx)"]
    output = {}
    for line in length:
        if line in output:
            output[line] += 1
        else:
            output[line] = 1

    plt.bar(output.keys(), output.values())
    plt.yscale('log')
    plt.title("Arm length to frequency")
    plt.xlabel("Arm length")
    plt.ylabel("Frequency of palindrome with this arm length (log)")
    plt.show()
    return output

#Plots gap length to frequency
def plot_gap(df):
    gap = df["Gap(Approx)"]
    output = {}
    for line in gap:
        if line in output:
            output[line] += 1
        else:
            output[line] = 1

    plt.bar(output.keys(), output.values())
    plt.title("Gap length to frequency")
    plt.xlabel("Gap length")
    plt.ylabel("Frequency of palindrome with this gap length")
    plt.show()
    return output

#Plots the distribution of palindromes across the genome
def plot_distribution(bin_size, df):
    end = df["End"]
    end_length = len(end)
    bins = {}

    for i in range(end[end_length - 1] // bin_size + 1):
        bins[i] = num_bps(df, i*bin_size, (i+1)*bin_size)

    plt.plot(bins.keys(), bins.values())
    plt.title("Distribution on chromosome")
    plt.xlabel(f"Location on sequence (by {bin_size} bp)")
    plt.ylabel(f"Num base pairs in a palindrome per {bin_size} bp")
    plt.show()
    return bins

#Plots heatmap of arm and gap length to frequency
def plot_heatmap(df):
    length = df["Arm-Length(Approx)"]
    gap = df["Gap(Approx)"]

    matrix = np.zeros((length.max() +1, gap.max() +1))
    for i in range(len(length)):
        matrix[length[i]][gap[i]] += 1

    plt.imshow(matrix)
    plt.gca().invert_yaxis()
    plt.title("Arm Length and gap to frequency")
    plt.ylabel("Arm Length")
    plt.xlabel("Gap length")
    plt.colorbar()
    plt.show()
    return matrix

if __name__ == "__main__":
    main()