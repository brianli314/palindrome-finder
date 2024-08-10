import os
import pandas
import sys
import matplotlib.pyplot as plt
large_path = ".."
script, path_name, length = sys.argv

df = pandas.read_csv(os.path.join(large_path, path_name), sep='\t')

def get_num_palins(length):
    column = df["Length"]
    counter = 0
    x_axis = []
    y_axis = []
    for i in range(int(length), 51):
        for line in column:
            if line == i:
                counter += 1
            if i == 50 and line > 50:
                counter += 1

        if i != 50:
            x_axis.append(str(i))
        y_axis.append(counter)
        counter = 0

    x_axis.append("50+")

    plt.bar(x_axis, y_axis, width=0.4)
    plt.xticks([0, 5, 10, 15, 20, 25, 30,35, 40])
    plt.yscale('log')
    plt.xlabel("Palindrome length")
    plt.ylabel("Num palindromes")
    plt.show()

get_num_palins(length)

