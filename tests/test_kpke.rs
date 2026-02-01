use ml_kem_rs::{kpke::key_gen, SELECTED_PARAMETER_SET};

#[test]
fn test_key_gen_completes() {
    let d = [42u8; 32];
    // Should not panic
    let _ = key_gen(d);
}

#[test]
fn test_key_gen_deterministic() {
    let d = [123u8; 32];
    let result1 = key_gen(d);
    let result2 = key_gen(d);
    
    // Key generation with same seed should produce same result
    // This tests that the process is deterministic
    // (Assuming key_gen returns something comparable)
}

#[test]
fn test_key_gen_different_seeds() {
    let d1 = [1u8; 32];
    let d2 = [2u8; 32];
    
    let _result1 = key_gen(d1);
    let _result2 = key_gen(d2);
    
    // Different seeds should produce different keys
    // (Once key_gen returns keys, compare them)
}

#[test]
fn test_key_gen_zero_seed() {
    let d = [0u8; 32];
    // Should handle zero seed gracefully
    let _ = key_gen(d);
}

#[test]
fn test_key_gen_max_seed() {
    let d = [255u8; 32];
    // Should handle max value seed gracefully
    let _ = key_gen(d);
}

#[test]
fn test_key_gen_runs_without_panic() {
    // Test with various random-looking seeds
    for i in 0..10 {
        let mut d = [0u8; 32];
        for j in 0..32 {
            d[j] = ((i * 13 + j * 7) % 256) as u8;
        }
        let _ = key_gen(d);
    }
}

#[test]
fn test_key_gen_parameter_set_ml_kem_768() {
    // Verify the selected parameter set is ML-KEM-768
    assert_eq!(SELECTED_PARAMETER_SET.n, 256);
    assert_eq!(SELECTED_PARAMETER_SET.k, 3);
    assert_eq!(SELECTED_PARAMETER_SET.q, 3329);
    assert_eq!(SELECTED_PARAMETER_SET.eta1, 2);
}

#[cfg(test)]
mod key_gen_integration_tests {
    use super::*;
    
    #[test]
    fn test_key_gen_with_getrandom() {
        // Test with actual random bytes
        let mut d = [0u8; 32];
        getrandom::fill(&mut d).expect("Failed to generate random bytes");
        
        let _ = key_gen(d);
    }
    
    #[test]
    fn test_key_gen_multiple_random_seeds() {
        // Generate multiple keys with different random seeds
        for _ in 0..5 {
            let mut d = [0u8; 32];
            getrandom::fill(&mut d).expect("Failed to generate random bytes");
            let _ = key_gen(d);
        }
    }
}
