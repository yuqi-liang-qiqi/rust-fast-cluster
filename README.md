# Rust FastCluster (Native Rust & Python Support)

A lightweight hierarchical clustering implementation using the **Ward method**, written in Rust with support for both native Rust and Python workflows.

## 🔧 Features

- ✅ **Ward linkage** method (only, for now)
- 🦀 **Native Rust** – Use `linkage(...)` for pure Rust applications
- 🐍 **Python Support** – Use `linkage_py(...)` via PyO3 and NumPy integration
- ⚡ **High Performance** – Rust-powered speed with memory safety
- 🪟 **Cross-Platform** – Works smoothly on Windows, macOS, and Linux, including AMD chipsets where C++ alternatives may fail

## 📦 Why Rust FastCluster?

This project was created to address compatibility issues with C++-based clustering libraries like [`fastcluster`](https://danifold.net/fastcluster.html), which can fail on certain Windows systems (e.g., AMD chipsets). Rust FastCluster provides:

- **Better Compatibility** – Reliable on diverse hardware, especially Windows AMD systems
- **Comparable Performance** – Native Rust speed, often close to C++ implementations
- **Easy Integration** – Seamless Python support via PyO3 and NumPy
- **Memory Safety** – Rust's guarantees reduce bugs and crashes

### Tested Scenarios:
- **Intel Windows**: C++ `fastcluster` is faster, but Rust FastCluster works reliably.
- **AMD Windows**: C++ `fastcluster` often fails, while Rust FastCluster runs smoothly.

This makes Rust FastCluster a **robust fallback** or alternative backend for clustering tasks.

## 🚧 Limitations

- Only supports the **Ward** method (more methods may be added later).
- Requires a **condensed 1D distance matrix** as input (e.g., from `scipy.spatial.distance.squareform` in Python).
- Experimental backend – test thoroughly before using in production.

## 📁 Usage

### 🦀 Native Rust Example

Run the example using:


```bash
cargo run --example main
```

### 🐍 Python Example

First, build and install the Python module using `maturin`:

```bash
maturin develop  # or pip install .
```

Then run the example:

```bash
python examples/main.py
```

💡 Purpose

Rust FastCluster aims to:

* Provide a reliable alternative to C++-based clustering libraries.
* Enable seamless integration into both Rust and Python workflows.
* Support diverse environments with a focus on compatibility and performance.

* > Note: For critical applications, compare performance with other backends and test thoroughly.


Here’s the simplified English explanation of the Ward method's input and output as a standalone section, ready to be added to your README:

## 🧠 Understanding the Ward Method: Input and Output

The `linkage` function in Rust FastCluster uses the **Ward method** for hierarchical clustering. Here's a simple explanation of what it takes as input and what it gives as output.

### Input
The function needs two things:

1. **Condensed Distance Matrix (`d`)**  
   - This is a list of numbers (a 1D array) that represents the distances between pairs of points.  
   - Instead of a full square matrix (which would include duplicate distances and zeros on the diagonal), we use a "condensed" format to save space.  
   - For `n` points, the length of this list should be `n * (n - 1) / 2`.  
   - **Example**: For 4 points, the length is `4 * (4 - 1) / 2 = 6`. The list might look like:  
     ```rust
     let d = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6];
     ```
     - `0.1` is the distance between point 0 and point 1.
     - `0.2` is the distance between point 0 and point 2.
     - `0.3` is the distance between point 0 and point 3.
     - `0.4` is the distance between point 1 and point 2.
     - `0.5` is the distance between point 1 and point 3.
     - `0.6` is the distance between point 2 and point 3.  
   - **Note**: In Python, you can create this list using `scipy.spatial.distance.pdist`.

2. **Method (`method`)**  
   - This is a string that tells the function which clustering method to use.  
   - Right now, it only supports `"ward"`.  
   - **Example**:  
     ```rust
     let method = "ward";
     ```

### Output

The function returns a **Linkage Matrix** (often called the `Z` matrix).  

- This is a table (a 2D array) that describes how the points are grouped into clusters step by step.  
- For `n` points, the table has `n - 1` rows (because it takes `n - 1` steps to combine all points into one cluster).  
- Each row has 4 columns:  
  1. The ID of the first cluster being merged.  
  2. The ID of the second cluster being merged.  
  3. The distance between these two clusters.  
  4. The number of points in the new cluster after merging.  

- **Example Output**: For 4 points, the output might look like:  
```
[[  0.   1.   0.1   2. ]
[  2.   4.   0.25  3. ]
[  3.   5.   0.45  4. ]]
```
  - **Row 1**: `[0, 1, 0.1, 2]` means:  
    - Cluster 0 and Cluster 1 are merged.  
    - The distance between them is 0.1.  
    - The new cluster (ID 4) has 2 points.  
  - **Row 2**: `[2, 4, 0.25, 3]` means:  
    - Cluster 2 and Cluster 4 (from the previous step) are merged.  
    - The distance between them is 0.25.  
    - The new cluster (ID 5) has 3 points.  
  - **Row 3**: `[3, 5, 0.45, 4]` means:  
    - Cluster 3 and Cluster 5 (from the previous step) are merged.  
    - The distance between them is 0.45.  
    - The new cluster (ID 6) has 4 points.  

### What Can You Do with the Output?
- The linkage matrix (`Z`) can be used to draw a **dendrogram** (a tree diagram showing how points are grouped).  
- In Python, you can pass it to `scipy.cluster.hierarchy.dendrogram` to visualize the clustering.  
- It also helps you understand the structure of your clusters at each step.

## 📬 Questions or Feedback

If you have any questions, suggestions, or run into issues, feel free to:

- Open an [issue]([https://github.com/your-repo-link/issues](https://github.com/yuqi-liang-qiqi/rust-fast-cluster/issues))
- Or contact me directly at **yuqi.liang.1900@gmail.com**

Always happy to hear from you!

