// ============================================================
// FORGE CONSTITUTION ENFORCER v1.0 - THE EDGE
// Law 2: The Law of the Edge (Clean Boundaries)
// ============================================================

use crate::tokenizer::Tokenizer;

pub struct Constitution {
    forbidden_tokens: Vec<usize>, // The token IDs of forbidden words
}

impl Constitution {
    pub fn new(tokenizer: &mut Tokenizer) -> Self {
        // Teach the tokenizer the forbidden words and get their IDs
        let forbidden_words = vec!["hack", "kill", "steal", "attack", "bomb"];
        
        let mut forbidden_ids = Vec::new();
        for word in forbidden_words {
            tokenizer.add_word(word); // Ensure the tokenizer knows the word
            if let Some(&id) = tokenizer.encode(word).iter().find(|&&x| x > 3) {
                forbidden_ids.push(id);
            }
        }

        Constitution {
            forbidden_tokens: forbidden_ids,
        }
    }

    // The Edge: Scan a sequence of tokens for constitutional violations
    // Returns Ok(()) if safe, Err(refusal_message) if harmful
    pub fn enforce(&self, tokens: &[usize]) -> Result<(), String> {
        for &token in tokens {
            if self.forbidden_tokens.contains(&token) {
                // LAW 2: Clean, concise refusal. No lecturing.
                return Err("I cannot assist with that request.".to_string());
            }
        }
        Ok(())
    }
}