import pandas as pd
import numpy as np

# Load the data
data = pd.read_csv('TIMES.CSV')

# Compute statistics
mean_time = data['elapsed_time_us'].mean()
median_time = data['elapsed_time_us'].median()
std_dev_time = data['elapsed_time_us'].std()
min_time = data['elapsed_time_us'].min()
max_time = data['elapsed_time_us'].max()

# Print statistics
print(f"Mean: {mean_time} us")
print(f"Median: {median_time} us")
print(f"Standard Deviation: {std_dev_time} us")
print(f"Min: {min_time} us")
print(f"Max: {max_time} us")

