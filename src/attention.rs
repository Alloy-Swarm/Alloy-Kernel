// ============================================================
// FORGE SELF-ATTENTION v1.0 - THE CONTEXT ENGINE
// This is the exact mathematical mechanism that powers 
// Claude, GPT-4, and all frontier AI models.
// ============================================================

use crate::brain::Tensor;

pub struct Attention;

impl Attention {
    /// Scaled Dot-Product Attention
    /// Q (Query): What I'm looking for
    /// K (Key): What I contain
    /// V (Value): The actual information I give you
    pub fn scaled_dot_product_attention(
        query: &Tensor, 
        key: &Tensor, 
        value: &Tensor
    ) -> Result<Tensor, String> {
        // 1. Calculate Attention Scores (How much focus to give each word)
        // Scores = Query * Key^T
        let key_transposed = key.transpose();
        let scores = query.matmul(&key_transposed)
            .map_err(|e| e.to_string())?;

        // 2. Scale the scores (Prevents numbers from getting too big)
        let d_k = query.cols as f32;
        let scale_factor = 1.0 / d_k.sqrt();
        let mut scaled_scores_data = Vec::with_capacity(scores.data.len());
        for &val in &scores.data {
            scaled_scores_data.push(val * scale_factor);
        }
        let scaled_scores = Tensor::from_flat_data(
            scaled_scores_data, 
            scores.rows, 
            scores.cols
        );

        // 3. Softmax (Turn scores into probabilities that sum to 1.0)
        let attention_weights = Self::softmax(&scaled_scores)?;

        // 4. Multiply by Value (Extract the actual information based on focus)
        let output = attention_weights.matmul(value)
            .map_err(|e| e.to_string())?;

        Ok(output)
    }

    /// Softmax: Turns raw numbers into probabilities (0.0 to 1.0)
    fn softmax(tensor: &Tensor) -> Result<Tensor, String> {
        let mut output_data = Vec::with_capacity(tensor.data.len());
        
        for i in 0..tensor.rows {
            // Find the max value in the row for numerical stability
            let row_start = i * tensor.cols;
            let row_end = row_start + tensor.cols;
            let row = &tensor.data[row_start..row_end];
            let max_val = row.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

            // Calculate exponentials
            let exps: Vec<f32> = row.iter().map(|&x| (x - max_val).exp()).collect();
            let sum_exps: f32 = exps.iter().sum();

            // Normalize
            for exp in exps {
                output_data.push(exp / sum_exps);
            }
        }

        Ok(Tensor::from_flat_data(output_data, tensor.rows, tensor.cols))
    }
}