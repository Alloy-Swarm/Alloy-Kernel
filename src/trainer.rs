// ============================================================
// FORGE TRAINER v1.0 - THE NIGHTLY EVOLUTION
// Teaches the AI by adjusting weights based on text data.
// ============================================================

use crate::brain::Tensor;
use crate::tokenizer::Tokenizer;

pub struct Trainer;

impl Trainer {
    // Train a simple statistical model (Bigram) using our custom Tensors.
    // This is the foundation of language learning: "Given word A, what is word B?"
    pub fn train_on_text(text: &str, tokenizer: &mut Tokenizer) -> Tensor {
        // 1. Teach the tokenizer all the words in the text
        for word in text.to_lowercase().split_whitespace() {
            let clean_word: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
            if !clean_word.is_empty() {
                tokenizer.add_word(&clean_word);
            }
        }

        let vocab_size = tokenizer.vocab_size();
        
        // 2. Create a Matrix (Tensor) to store word-relationships
        // Rows = Current Word, Columns = Next Word
        let mut weights_data = vec![0.0; vocab_size * vocab_size];

        // 3. Read the text and count word pairs
        let tokens = tokenizer.encode(text);
        for i in 0..tokens.len().saturating_sub(1) {
            let current_token = tokens[i];
            let next_token = tokens[i + 1];
            
            // Increment the count in the matrix
            let index = current_token * vocab_size + next_token;
            weights_data[index] += 1.0;
        }

        // 4. Apply a "Softmax" like normalization (add smoothing, then normalize)
        // This turns raw counts into probabilities that sum to 1.0
        let mut final_data = vec![0.0; vocab_size * vocab_size];
        for i in 0..vocab_size {
            let mut row_sum = 0.0;
            for j in 0..vocab_size {
                // Add a tiny number (smoothing) so probability is never exactly 0
                let val = weights_data[i * vocab_size + j] + 0.1; 
                final_data[i * vocab_size + j] = val;
                row_sum += val;
            }
            // Normalize the row so it sums to 1.0
            if row_sum > 0.0 {
                for j in 0..vocab_size {
                    final_data[i * vocab_size + j] /= row_sum;
                }
            }
        }

        Tensor::from_flat_data(final_data, vocab_size, vocab_size)
    }
}