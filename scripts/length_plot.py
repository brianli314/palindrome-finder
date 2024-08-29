import os
import pandas
import sys
import matplotlib.pyplot as plt
import argparse

parser = argparse.ArgumentParser(
    description="Plots palindrome length to frequency. Specify starting length and ending length"
)
parser.add_argument('-i', '--input', required=True, help="File input path")
parser.add_argument('-s', '--start', required=True, help="Starting/minimum length")
parser.add_argument('-e', '--end', required=True, help="Ending/maximum length")

args = parser.parse_args()
input_name = args.input
min_length = args.start
max_length = args.end

df = pandas.read_csv(input_name, sep='\t')

#Shows frequency to length
def plot_length(start, end):
    column = df["Length"]
    counter = 0
    output = {}

    for i in range(start, end+1):
        for line in column:
            if line == i:
                counter += 1
            if i == end and line > end:
                counter += 1


        output[i] = counter
        counter = 0

    plt.bar(output.keys(), output.values())
    plt.yscale('log')
    x_ticks = set(range(start, end, end//5))
    x_ticks.add(end)
    x_ticks = sorted(list(x_ticks))
    x_label = list(str(x) for x in x_ticks)
    x_label[-1] = f"{end}+"
    plt.xticks(x_ticks, x_label)
    plt.title("Length to number of palindromes")
    plt.xlabel("Palindrome length")
    plt.ylabel("Num palindromes (log)")
    plt.show()
    return output
    
if __name__ == '__main__':
    plot_length(min_length, max_length)