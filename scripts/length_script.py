import os
import pandas
import sys
large_path = ".."
script, path_name = sys.argv

df = pandas.read_csv(os.path.join(large_path, path_name), sep='\t')

def get_num_palins():
    column = df["Length"]
    counter = 0
    x_axis = []
    y_axis = []
    for i in range(10, 51):
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

    return [x_axis, y_axis]
    

#get_num_palins(length)

