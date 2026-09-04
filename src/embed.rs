use crate::lexicon::{self, FEATURES};

pub const HASH_DIMS: usize = 32;
pub const DIMS: usize = FEATURES.len() + HASH_DIMS;

pub fn embed_text(text: &str) -> Vec<f32> {
    let mut vector = vec![0.0f32; DIMS];
    let tokens = tokenize(text);
    if tokens.is_empty() {
        return vector;
    }

    for token in &tokens {
        let stemmed = lexicon::stem(token);
        if let Some(features) = lexicon::lookup(&stemmed) {
            for &index in features {
                vector[index as usize] += 1.0;
            }
        } else if let Some(features) = lexicon::lookup(token) {
            for &index in features {
                vector[index as usize] += 1.0;
            }
        }
        accumulate_ngrams(&mut vector, token);
    }

    for window in tokens.windows(2) {
        let bigram = format!("{} {}", window[0], window[1]);
        accumulate_hashed(&mut vector, &bigram, 0.35);
    }

    l2_normalize(&mut vector);
    vector
}

pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut dot = 0.0;
    let mut na = 0.0;
    let mut nb = 0.0;
    for i in 0..a.len() {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na.sqrt() * nb.sqrt())
    }
}

pub fn lerp_vec(a: &[f32], b: &[f32], t: f32) -> Vec<f32> {
    let t = t.clamp(0.0, 1.0);
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x * (1.0 - t) + y * t)
        .collect()
}

pub fn sub_vec(a: &[f32], b: &[f32], amount: f32) -> Vec<f32> {
    let mut out: Vec<f32> = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| x - y * amount)
        .collect();
    l2_normalize(&mut out);
    out
}

pub fn add_vec(a: &[f32], b: &[f32], amount: f32) -> Vec<f32> {
    let mut out: Vec<f32> = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| x + y * amount)
        .collect();
    l2_normalize(&mut out);
    out
}

pub fn l2_normalize(vector: &mut [f32]) {
    let norm: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 1e-8 {
        for value in vector.iter_mut() {
            *value /= norm;
        }
    }
}

pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            current.push(c.to_ascii_lowercase());
        } else if !current.is_empty() {
            tokens.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

pub fn clauses(text: &str) -> Vec<String> {
    let mut clauses = Vec::new();
    let mut current = String::new();
    for c in text.chars() {
        current.push(c);
        if matches!(c, '.' | '!' | '?' | ';' | '\n') {
            let trimmed = current.trim().to_string();
            if !trimmed.is_empty() {
                clauses.push(trimmed);
            }
            current.clear();
        }
    }
    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        clauses.push(trimmed);
    }
    if clauses.is_empty() && !text.trim().is_empty() {
        clauses.push(text.trim().to_string());
    }
    clauses
}

fn accumulate_ngrams(vector: &mut [f32], token: &str) {
    let padded = format!("_{token}_");
    let chars: Vec<char> = padded.chars().collect();
    if chars.len() >= 3 {
        for window in chars.windows(3) {
            let gram: String = window.iter().collect();
            accumulate_hashed(vector, &gram, 0.12);
        }
    }
}

fn accumulate_hashed(vector: &mut [f32], text: &str, weight: f32) {
    let index = FEATURES.len() + (fnv1a(text) % HASH_DIMS as u64) as usize;
    vector[index] += weight;
}

fn fnv1a(text: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in text.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn similar_phrases_are_closer_than_opposites() {
        let warm = embed_text("ember hands held darling tender");
        let also_warm = embed_text("warm close held care");
        let cold = embed_text("ice chart fluorescent instruments");
        assert!(cosine(&warm, &also_warm) > cosine(&warm, &cold));
    }
}
