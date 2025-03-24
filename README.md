# Rust FastCluster (for Python)

A lightweight Rust-backed hierarchical clustering implementation, currently supporting the **Ward method** for precomputed condensed distance matrices.

## 🔧 Features

- ✅ **Ward linkage** (only)
- 🐍 **Python-compatible** – designed to plug into existing sequence analysis workflows
- 🪟 **Optimized for Windows** – especially useful when C++-based alternatives fail

## 📦 Why Rust?

While the C++-based [`fastcluster`](https://danifold.net/fastcluster.html) library is highly efficient and performs well on **macOS/Linux** and some **Windows (Intel)** machines, it can **fail to run or compile** on certain Windows environments—particularly on **non-Intel chipsets**, such as those found in recent **CMD (Microsoft custom)** hardware.

This Rust implementation offers:

- ✅ Better **compatibility** on Windows machines where `fastcluster` (C++) may fail
- ⚡ **Native performance**, comparable in many cases
- 🔧 **Ease of packaging** and cross-platform distribution
- 🛡️ **Memory safety** and modern tooling

> This module has been tested on multiple Windows systems:
> - On an **Intel Windows** machine: `fastcluster (C++)` generally performs faster.
> - On a **AMD Windows** machine: the C++ version **fails to run**, but this Rust version **works smoothly**.

Hence, this module serves as a **fallback option** and **alternative backend** for users encountering system compatibility issues.

## 🚧 Limitations

- Only supports the **Ward** method (more methods may be added in the future).
- Input must be a **condensed 1D distance matrix**, e.g., from `scipy.spatial.distance.squareform`.
- This is an **experimental backend**, not a replacement for general-purpose clustering libraries.

## 💡 Purpose

I include this Rust implementation to:

- Offer a **robust fallback** when fastcluster (C++) does not work
- Allow users to **compare performance** across backends and systems
- Promote **accessibility** for diverse Python environments

> I recommend **additional testing** before adopting it in production pipelines, especially if raw speed is critical.

## 📁 Example Usage

```python
from sequenzo._rust_fast_cluster import linkage

linkage_matrix = linkage(condensed_matrix, method="ward")
```

## 📬 Questions or Feedback

If you have any questions, suggestions, or run into issues, feel free to:

- Open an [issue]([https://github.com/your-repo-link/issues](https://github.com/yuqi-liang-qiqi/rust-fast-cluster/issues))
- Or contact me directly at **yuqi.liang.1900@gmail.com**

Always happy to hear from you!

