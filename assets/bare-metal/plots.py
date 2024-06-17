import pandas as pd
import matplotlib.pyplot as plt

# Load the data
data = pd.read_csv('TIMES.CSV')

# Histogram
plt.figure(figsize=(10, 6))
plt.hist(data['elapsed_time_us'], bins=50, alpha=0.75, color='blue')
plt.title('Histogram of Execution Times')
plt.xlabel('Elapsed Time (us)')
plt.ylabel('Frequency')
plt.grid(True)
plt.show()

# Box plot
plt.figure(figsize=(10, 6))
plt.boxplot(data['elapsed_time_us'], vert=False)
plt.title('Box Plot of Execution Times')
plt.xlabel('Elapsed Time (us)')
plt.grid(True)
plt.show()

