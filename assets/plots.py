import pandas as pd
import matplotlib.pyplot as plt
import sys

dir = sys.argv[1]

# Load the data
data = pd.read_csv(dir + '/times.csv')

# Histogram
plt.figure()
plt.hist(data['elapsed_time_us'])
plt.title('Histogram of Execution Times')
plt.xlabel('Elapsed Time (us)')
plt.ylabel('iterations')
plt.yscale('log')
plt.savefig(dir + '/hist.svg')

plt.figure()
plt.plot(data['elapsed_time_us'], '.')
plt.title('iteration times')
plt.xlabel('iteration')
plt.ylabel('Elapsed Time (us)')
plt.savefig(dir + '/times.png', dpi=300)
