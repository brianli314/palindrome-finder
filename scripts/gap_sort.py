import os
import pandas
import sys
import matplotlib.pyplot as plt


script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", "outputs",path_name), sep='\t')

def gap_sort(bin_size):
    gap = df["Gap(Approx)"]
    output = {}
    for line in gap:
        index = line // bin_size
        if index in output:
            output[index] += 1
        else:
            output[index] = 1
    write_to_file(output)
    plt.scatter(list(output.keys()), list(output.values()))
    plt.xticks(list(output.keys()))
    plt.xlabel(f"Sorted every {bin_size} bps")
    plt.show()

    
def write_to_file(output):
    with open(os.path.join("..", "outputs", "gap_output.txt"), 'w') as file:
        for key, value in sorted(output.items()):
            file.write(f'{key*10} - {(key+1)*10}: {value}\n')
gap_sort(10)
