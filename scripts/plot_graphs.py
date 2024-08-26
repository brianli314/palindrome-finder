import matplotlib.pyplot as plt

import length_script, palin_distribution

def plot_graphs(distribution, length):
    num = 2
    idx = 1

    if distribution and length:
        fig, (axs1, axs2) = plt.subplots(2)
    elif distribution:
        fig, axs1 = plt.subplots(1)
    elif length:
        fig, axs2 = plt.subplots(1)
    else: 
        return
    
    if distribution:
        data = palin_distribution.get_ratio(100000)
        bin_indices = data[0]
        bin_lengths = data[1]
        axs1.plot(bin_indices, bin_lengths)
        axs1.set_xlabel(f"Location on sequence (every {100000})")
        axs1.set_ylabel("Num base pairs")
    
    if length:
        length_data = length_script.get_num_palins()
        x_axis = length_data[0]
        y_axis = length_data[1]
        axs2.bar(x_axis, y_axis, width=0.4)
        axs2.set_xticks([0, 5, 10, 15, 20, 25, 30, 35, 40])
        axs2.set_yscale('log')
        axs2.set_xlabel("Palindrome length")
        axs2.set_ylabel("Num palindromes")
    

    plt.show()

plot_graphs(True, True)