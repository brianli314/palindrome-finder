import os
import pandas
import sys
import matplotlib.pyplot as plt
import numpy as np



script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..",path_name), sep='\t')

def gap_to_length(bin_size):
    length = df["Length"]
    gap = df["Gap(Approx)"]
    matrix = np.zeros((length.max()//bin_size +1, gap.max()//bin_size +1))
    for i in range(len(length)):
        matrix[length[i]//bin_size][gap[i]//bin_size] += 1

    #plt.imshow(matrix, cmap='coolwarm', interpolation='nearest')
    plt.contourf(matrix)
    plt.title(f"Length-gap palindrome quantity contour plot (grouped by every {bin_size} bases)")
    plt.ylabel(f"Length of palindrome")
    plt.xlabel(f"Gap length")
    plt.colorbar()
    plt.show()

gap_to_length(1)