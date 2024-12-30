
# GPU Proof Project

A Rust-based project for generating and verifying zkSNARK proofs using GPU acceleration. This project leverages [bellperson](https://github.com/filecoin-project/bellperson) for efficient zero-knowledge proof generation and CUDA for hardware-accelerated performance.

---

## Features

- **Randomized Circuit Generation**: Supports dynamic, complex circuit generation for zkSNARKs.
- **GPU Acceleration**: Uses NVIDIA CUDA to speed up proof generation.
- **Resource Control**: Dynamically adjusts input size to control memory consumption (default: up to 50GB).

---

## Installation

### Prerequisites

- **Rust (latest stable version)**  
  Install Rust:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup update
  ```

- **CUDA Toolkit (v11.0 or higher)**  
  Install CUDA:  
  [Download from NVIDIA](https://developer.nvidia.com/cuda-downloads)

  Verify installation:
  ```bash
  nvcc --version
  ```

### Clone and Build

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/gpu_proof_project.git
   cd gpu_proof_project
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

---

## Usage

### Running the Project

Execute the following command to run the project:
```bash
cargo run --release
```

### Expected Output

The program will:
1. Generate a randomized zkSNARK circuit.
2. Create a zero-knowledge proof using GPU acceleration.
3. Verify the generated proof.
4. Output the validity and execution time.

---

## Configuration

The project supports configurable input size and memory control. Default settings:
- **Maximum Memory Usage**: 50GB.
- **Optimization**: Maximum (Release build).

To modify configurations, edit `config.rs` in the source directory.

---

## Monitoring GPU Usage

Use `nvidia-smi` to monitor GPU load and memory usage during execution:
```bash
nvidia-smi -l 1
```

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch:
   ```bash
   git checkout -b feature-name
   ```
3. Commit your changes:
   ```bash
   git commit -m "Description of changes"
   ```
4. Push to your branch:
   ```bash
   git push origin feature-name
   ```
5. Open a pull request.

---

## Acknowledgments

- [bellperson](https://github.com/filecoin-project/bellperson) for zkSNARK implementation.
- [NVIDIA CUDA](https://developer.nvidia.com/cuda-zone) for GPU acceleration.
