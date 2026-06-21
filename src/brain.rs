// ============================================================
// FORGE TENSOR ENGINE v1.0
// The fundamental math layer for the Forge Brain.
// No external ML libraries. Pure Rust.
// ============================================================

#[derive(Debug)]
pub struct Tensor {
    pub data: Vec<f32>,
    pub rows: usize,
    pub cols: usize,
}

impl Tensor {
    // Create a new Tensor filled with zeros
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Tensor {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    // Create a new Tensor from a 2D array of data
    pub fn from_data(data: Vec<Vec<f32>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        let flat_data: Vec<f32> = data.into_iter().flatten().collect();
        Tensor { data: flat_data, rows, cols }
    }

    // Create a Tensor from a flat vector of data
    pub fn from_flat_data(data: Vec<f32>, rows: usize, cols: usize) -> Self {
        Tensor { data, rows, cols }
    }

    // Flip the matrix rows and columns (Required for Attention)
    pub fn transpose(&self) -> Tensor {
        let mut transposed_data = vec![0.0; self.rows * self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                transposed_data[j * self.rows + i] = self.data[i * self.cols + j];
            }
        }
        Tensor {
            data: transposed_data,
            rows: self.cols,
            cols: self.rows,
        }
    }

    // The core of AI: Matrix Multiplication
    // This is the exact operation that happens inside Transformers
    pub fn matmul(&self, other: &Tensor) -> Result<Tensor, &str> {
        if self.cols != other.rows {
            return Err("Matrix dimensions do not match for multiplication!");
        }

        let mut result = Tensor::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    // A[i][k] * B[k][j]
                    let a_val = self.data[i * self.cols + k];
                    let b_val = other.data[k * other.cols + j];
                    sum += a_val * b_val;
                }
                // C[i][j] = sum
                result.data[i * result.cols + j] = sum;
            }
        }

        Ok(result)
    }

    // A simple activation function (ReLU) to add non-linearity
    // AI cannot learn complex things without activation functions
    #[allow(dead_code)] // Added to suppress the warning since we aren't using it yet
    pub fn relu(&self) -> Tensor {
        let mut result = Tensor::zeros(self.rows, self.cols);
        for i in 0..self.data.len() {
            result.data[i] = if self.data[i] > 0.0 { self.data[i] } else { 0.0 };
        }
        result
    }
}