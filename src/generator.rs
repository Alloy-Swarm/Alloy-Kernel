// ============================================================
// FORGE GENERATOR v1.0 - THE MIND
// Takes tokens, processes them through the Brain, and predicts the next token.
// (Currently uses random weights, but the architecture is real).
// ============================================================

use crate::brain::Tensor;
use crate::tokenizer::Tokenizer;
use rand::Rng;

pub struct Generator {
    weights: Tensor, // The neural network layer
}

impl Generator {
    pub fn new(tokenizer: &Tokenizer) -> Self {
        // In a real AI, these weights are trained on billions of words.
        // For now, we initialize them randomly to prove the architecture works.
        let vocab_size = tokenizer.vocab_size();
        let mut rng = rand::thread_rng();
        
        let mut random_data = Vec::with_capacity(vocab_size * vocab_size);
        for _ in 0..(vocab_size * vocab_size) {
            random_data.push(rng.gen_range(-0.1..0.1)); // Small random numbers
        }

        let weights = Tensor::from_flat_data(random_data, vocab_size, vocab_size);

        Generator { weights }
    }

    // The "Forward Pass" of the AI
    // Takes the current tokens, does the math, and guesses the next token.
    pub fn generate(&self, input_tokens: &[usize], max_len: usize) -> Vec<usize> {
        let mut output_tokens = input_tokens.to_vec();
        let mut rng = rand::thread_rng();

        for _ in 0..max_len {
            // 1. Take the last token as the context
            let last_token = *output_tokens.last().unwrap_or(&0);
            
            // 2. Create a One-Hot vector (1.0 at the token's index, 0.0 everywhere else)
            let vocab_size = self.weights.rows;
            let mut input_data = vec![0.0; vocab_size];
            if last_token < vocab_size {
                input_data[last_token] = 1.0;
            }
            let input_tensor = Tensor::from_data(vec![input_data]);

            // 3. Do the Matrix Multiplication (The core of AI)
            match input_tensor.matmul(&self.weights) {
                Ok(logits) => {
                    // 4. Find the highest number (Greedy Decoding)
                    // In a real AI, we use Softmax and sampling, but this proves the math works.
                    let mut best_token = 0;
                    let mut best_value = f32::NEG_INFINITY;
                    
                    for (i, &val) in logits.data.iter().enumerate() {
                        if val > best_value {
                            best_value = val;
                            best_token = i;
                        }
                    }

                    // 5. Stop if we hit the End of Sentence token
                    if best_token == 3 { // <EOS>
                        break;
                    }
                    
                    // Add some randomness so it doesn't repeat the same word forever
                    if rng.gen_bool(0.3) {
                        best_token = rng.gen_range(4..vocab_size.max(5));
                    }

                    output_tokens.push(best_token);
                }
                Err(_) => break,
            }
        }

        output_tokens
    }
}