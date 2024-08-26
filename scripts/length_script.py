import os
import pandas
import sys
import matplotlib.pyplot as plt

large_path = ".."
script, path_name = sys.argv

df = pandas.read_csv(os.path.join(large_path, path_name), sep='\t')

def get_num_palins(start, num):
    column = df["Length"]
    counter = 0
    output = {}

    for i in range(start, num+1):
        for line in column:
            if line == i:
                counter += 1
            if i == num and line > num:
                counter += 1


        output[i] = counter
        counter = 0

    plt.bar(output.keys(), output.values())
    plt.yscale('log')
    x_ticks = set(range(start, num, num//5))
    x_ticks.add(num)
    x_ticks = sorted(list(x_ticks))
    x_label = list(str(x) for x in x_ticks)
    x_label[-1] = f"{num}+"
    plt.xticks(x_ticks, x_label)
    plt.title("Length to number of palindromes")
    plt.xlabel("Palindrome length")
    plt.ylabel("Num palindromes (log)")
    plt.show()
    return output
    

get_num_palins(20, 300)

