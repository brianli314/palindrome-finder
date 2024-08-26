import os
import pandas
import sys
import matplotlib.pyplot as plt


script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..",path_name), sep='\t')

def gap_sort(bin_size):
    gap = df["Gap(Approx)"]
    output = {}
    for line in gap:
        index = line // bin_size
        if index in output:
            output[index] += 1
        else:
            output[index] = 1
    #write_to_file(output)
    plt.bar(list(output.keys()), list(output.values()))
    x_ticks = list(range(0, len(output.keys()), max(1, max(output.keys())//bin_size)))
    x_labels = [f"{i*bin_size} - {(i+1)*bin_size}" for i in x_ticks]
    plt.xticks(x_ticks, x_labels)
    plt.title("Gap length to number of palindromes")
    plt.xlabel("Gap length")
    plt.ylabel("Number palindromes in this range")
    plt.show()

def gap_sort_length(bin_size):
    gap = df["Gap(Approx)"]
    length = df["Length"]
    output = {}
    counts = {}
    for i in range(len(length)):
        index = gap[i] // bin_size
        if index in output:
            output[index] += length[i] - gap[i]
            counts[index] += 1
        else:
            output[index] = length[i] - gap[i]
            counts[index] = 1
    averages = {key: output[key] / counts[key] for key in output}
    plt.bar(list(averages.keys()), list(averages.values()))
    x_ticks = list(range(0, len(averages.keys()), max(1, max(averages.keys())//bin_size)))
    x_labels = [f"{i*bin_size} - {(i+1)*bin_size}" for i in x_ticks]
    plt.xticks(x_ticks, x_labels)
    plt.title("Gap length to average length")
    plt.xlabel("Gap length")
    plt.ylabel("Average length of palindrome")
    plt.show()


    
def write_to_file(output):
    with open(os.path.join("..", "outputs", "gap_output.txt"), 'w') as file:
        for key, value in sorted(output.items()):
            file.write(f'{key*10} - {(key+1)*10}: {value}\n')

gap_sort_length(10)