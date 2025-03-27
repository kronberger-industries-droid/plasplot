import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

e   = 1.602e-19   # elementary charge [C]
kB  = 1.381e-23   # Boltzmann constant [J/K]

def read_dataset(file_path):
    """
    Reads a tab-separated CSV file and returns a pandas DataFrame.
    
    Parameters:
        file_path (str): Path to the CSV file.
    
    Returns:
        pd.DataFrame: DataFrame containing the dataset.
    """
    df = pd.read_csv(file_path, delimiter="\t", decimal=",", engine="python")
    return df

def plot_fg(df):
    """
    Plots the column 'Ig' (interpreted as fg) versus 'U' with markers only on every 10th data point.
    
    Parameters:
        df (pd.DataFrame): DataFrame containing the data.
    """
    # Using tail(100) to get the last 100 rows
    last_100 = df.tail(100)

    # Fit a linear function (degree 1) for Ig over U
    slope, intercept = np.polyfit(last_100["U"], last_100["Ig"], 1)

    # Calculate the fitted values
    fitted_values_tail = slope * df["U"] + intercept

    print("Interception with U=0 at: ", intercept)

    first_100 = df.head(100)

    slope, intercept = np.polyfit(first_100["U"], first_100["Ig"], 1)

    # Calculate the fitted values
    fitted_values_head = slope * df["U"] + intercept

    print("Interception with U=0 at: ", intercept, "\n")
    
    # Create a line plot for Ig and I'g versus U
    plt.figure(figsize=(10, 6))
    plt.plot(df["U"], df["Ig"], label="Ig", marker="o")
    plt.plot(df["U"], fitted_values_head, '-', label='Linear Fit Head')
    plt.plot(df["U"], fitted_values_tail, '-', label='Linear Fit Tail')
    plt.xlabel("U")
    plt.ylabel("Current")
    plt.legend()

    # Adjust the axes to cross at (0,0)
    ax = plt.gca()
    ax.spines['left'].set_position('zero')
    ax.spines['bottom'].set_position('zero')
    ax.spines['right'].set_color('none')
    ax.spines['top'].set_color('none')
    ax.xaxis.set_label_coords(1.05, 0.5)  # x-label slightly to the right of the plot center
    ax.yaxis.set_label_coords(0.5, 1.05)  # y-label slightly above the plot center

    # Limit the number of tick marks on the x-axis (e.g., to 10 ticks)
    plt.locator_params(axis='x', nbins=10)
    plt.show()

def electron_temp(I_sp, I_sm, dI):
    return ((I_sp * I_sm) / (I_sp + I_sm)) / dI 

def electron_density(I_sp, I_sn, dI):
    return 3e21 * (I_sp + I_sn) / np.sqrt(electron_temp(I_sp, I_sm, dI))

def eq_64_d2IdU2(p, I_sp, I_sn, dI):
    return electron_density(I_sp, I_sn, dI) / (2.5e22 * p)

if __name__ == '__main__':
    file_path = 'Messung3.csv'  # Replace with your file path
    data = read_dataset(file_path)
    print(data.head())
    # row = data.loc[(data["U"] - 0).abs().argmin()]
    # print("U = 0: I'g", row["I'g"])
    # Generate plots
    plot_fg(data)
