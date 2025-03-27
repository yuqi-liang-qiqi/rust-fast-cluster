"""
@Author  : Yuqi Liang 梁彧祺
@File    : main.py
@Time    : 27/03/2025 10:28
@Desc    :
    How to run this example on your Python?
    Make sure you’ve built/installed the Python module with maturin:
        maturin develop  # or pip install .
        Then run:
        python examples/main.py
"""
from rust_fast_cluster import linkage
import numpy as np

# Condensed distance matrix for 4 points
# Corresponds to distances: (0,1), (0,2), (0,3), (1,2), (1,3), (2,3)
condensed_dists = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6]

# Call the Rust-accelerated linkage function
z = linkage(condensed_dists, method="ward")

print("Linkage matrix (Z):")
print(np.array(z))  # Convert to regular NumPy array if needed
