// ============================================================
// FORGE TOKENIZER v1.0
// Translates human text into AI numbers (and back).
// ============================================================

use std::collections::HashMap;

pub struct Tokenizer {
    vocab: HashMap<String, usize>, // Word -> ID
    decode_vocab: HashMap<usize, String>, // ID -> Word
}

impl Tokenizer {
    // Create a new blank Tokenizer
    pub fn new() -> Self {
        let mut vocab = HashMap::new();
        let mut decode_vocab = HashMap::new();

        // Special tokens every AI needs
        vocab.insert("<PAD>".to_string(), 0);
        vocab.insert("<UNK>".to_string(), 1); // Unknown word
        vocab.insert("<BOS>".to_string(), 2); // Beginning of sentence
        vocab.insert("<EOS>".to_string(), 3); // End of sentence

        // Build the reverse lookup
        for (word, &id) in &vocab {
            decode_vocab.insert(id, word.clone());
        }

        Tokenizer { vocab, decode_vocab }
    }

    // Teach the tokenizer a new word (like building a vocabulary)
    pub fn add_word(&mut self, word: &str) {
        if !self.vocab.contains_key(word) {
            let next_id = self.vocab.len();
            self.vocab.insert(word.to_string(), next_id);
            self.decode_vocab.insert(next_id, word.to_string());
        }
    }

    // How many words the tokenizer knows
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    // Encode: Turn a sentence into numbers
    pub fn encode(&self, text: &str) -> Vec<usize> {
        let mut tokens = vec![2]; // Start with <BOS>

        for word in text.to_lowercase().split_whitespace() {
            // Clean the word of basic punctuation for this simple version
            let clean_word: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
            
            if let Some(&id) = self.vocab.get(&clean_word) {
                tokens.push(id);
            } else {
                tokens.push(1); // <UNK> for words we don't know yet
            }
        }

        tokens.push(3); // End with <EOS>
        tokens
    }

    // Decode: Turn numbers back into a sentence
    pub fn decode(&self, tokens: &[usize]) -> String {
        let words: Vec<String> = tokens
            .iter()
            .filter_map(|&id| self.decode_vocab.get(&id).cloned())
            .filter(|word| !word.starts_with('<')) // Hide special tokens
            .collect();
        
        words.join(" ")
    }
}