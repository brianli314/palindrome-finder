import pandas
import os
import matplotlib.pyplot as plt

#script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", "outputs","gap_data"), sep=',', skiprows=9)

def gap_data():
    bps = df["bps"]
    gap = df["Gap"]
    x_axis = []
    y_axis = []
    for i in range(len(bps)):
        x_axis.append(gap[i])
        y_axis.append(bps[i])
    
    plt.scatter(x_axis, y_axis)
    plt.xlabel("Gap length")
    plt.ylabel("Number of bps (million)")
    plt.xticks(x_axis)
    plt.yticks(y_axis)
    plt.show()

        

gap_data()