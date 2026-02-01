use ml_kem_rs::{algebraic::*, polynomial::Poly16, SELECTED_PARAMETER_SET};

#[test]
fn test_ntt_output_size() {
    let coeffs = [1u16; 256];
    let poly = Poly16::new(&coeffs);
    
    let result = ntt(poly);
    
    // NTT should preserve size
    let _ = result[255]; // Should not panic
}

#[test]
fn test_ntt_deterministic() {
    let coeffs = [42u16; 256];
    let poly = Poly16::new(&coeffs);
    
    let result1 = ntt(poly);
    let result2 = ntt(poly);
    
    for i in 0..256 {
        assert_eq!(result1[i], result2[i], "NTT should be deterministic");
    }
}

#[test]
fn test_ntt_zero_polynomial() {
    let coeffs = [0u16; 256];
    let poly = Poly16::new(&coeffs);
    
    let result = ntt(poly);
    
    // NTT of zero should be zero
    for i in 0..256 {
        assert_eq!(result[i], 0, "NTT of zero polynomial should be zero");
    }
}

#[test]
fn test_ntt_output_in_range() {
    let mut coeffs = [0u16; 256];
    for i in 0..256 {
        coeffs[i] = (i % 3329) as u16;
    }
    let poly = Poly16::new(&coeffs);
    
    let result = ntt(poly);
    
    // All coefficients should be in valid range [0, q)
    for i in 0..256 {
        assert!(result[i] < SELECTED_PARAMETER_SET.q as u16, 
                "NTT output coefficient {} = {} is out of range", i, result[i]);
    }
}

#[test]
fn test_ntt_inv_output_size() {
    let coeffs = [1u16; 256];
    let poly = Poly16::new(&coeffs);
    
    let result = ntt_inv(poly);
    
    // NTT inverse should preserve size
    let _ = result[255]; // Should not panic
}

#[test]
fn test_ntt_inv_output_in_range() {
    let mut coeffs = [0u16; 256];
    for i in 0..256 {
        coeffs[i] = (i % 3329) as u16;
    }
    let poly = Poly16::new(&coeffs);
    
    let result = ntt_inv(poly);
    
    // All coefficients should be in valid range [0, q)
    for i in 0..256 {
        assert!(result[i] < SELECTED_PARAMETER_SET.q as u16, 
                "NTT inverse output coefficient {} = {} is out of range", i, result[i]);
    }
}

#[test]
fn test_ntt_roundtrip() {
    let mut coeffs = [0u16; 256];
    for i in 0..256 {
        coeffs[i] = ((i * 13) % 3329) as u16;
    }
    let original = Poly16::new(&coeffs);
    
    let transformed = ntt(original);
    let recovered = ntt_inv(transformed);
    
    // Should recover original polynomial (approximately, due to modular arithmetic)
    for i in 0..256 {
        assert_eq!(recovered[i], original[i], 
                   "NTT roundtrip failed at index {}", i);
    }
}

#[test]
fn test_sample_ntt_output_size() {
    let rho = [1u8; 32];
    let result = sample_ntt(rho, 0, 0);
    
    // Should produce 256 coefficients
    let _ = result[255]; // Should not panic
}

#[test]
fn test_sample_ntt_output_in_range() {
    let rho = [42u8; 32];
    let result = sample_ntt(rho, 0, 0);
    
    // All coefficients should be in valid range [0, q)
    for i in 0..256 {
        assert!(result[i] < SELECTED_PARAMETER_SET.q as u16,
                "Sampled coefficient {} = {} is out of range", i, result[i]);
    }
}

#[test]
fn test_sample_ntt_deterministic() {
    let rho = [123u8; 32];
    let result1 = sample_ntt(rho, 5, 7);
    let result2 = sample_ntt(rho, 5, 7);
    
    for i in 0..256 {
        assert_eq!(result1[i], result2[i], "sample_ntt should be deterministic");
    }
}

#[test]
fn test_sample_ntt_different_indices() {
    let rho = [1u8; 32];
    let result1 = sample_ntt(rho, 0, 0);
    let result2 = sample_ntt(rho, 0, 1);
    
    // Different indices should produce different results
    let mut different = false;
    for i in 0..256 {
        if result1[i] != result2[i] {
            different = true;
            break;
        }
    }
    assert!(different, "Different indices should produce different samples");
}

#[test]
fn test_sample_poly_cbd_eta2_output_size() {
    use ml_kem_rs::bytevec::ByteVec;
    let bytes = ByteVec::Vec128([42u8; 128]);
    let result = sample_poly_cbd::<2>(bytes);
    
    // Should produce 256 coefficients
    let _ = result[255]; // Should not panic
}

#[test]
fn test_sample_poly_cbd_eta3_output_size() {
    use ml_kem_rs::bytevec::ByteVec;
    let bytes = ByteVec::Vec192([42u8; 192]);
    let result = sample_poly_cbd::<3>(bytes);
    
    // Should produce 256 coefficients
    let _ = result[255]; // Should not panic
}

#[test]
fn test_sample_poly_cbd_output_in_range() {
    use ml_kem_rs::bytevec::ByteVec;
    let bytes = ByteVec::Vec128([123u8; 128]);
    let result = sample_poly_cbd::<2>(bytes);
    
    // All coefficients should be in valid range [0, q)
    for i in 0..256 {
        assert!(result[i] < SELECTED_PARAMETER_SET.q as u16,
                "CBD sampled coefficient {} = {} is out of range", i, result[i]);
    }
}

#[test]
fn test_sample_poly_cbd_deterministic() {
    use ml_kem_rs::bytevec::ByteVec;
    let bytes = ByteVec::Vec128([99u8; 128]);
    let result1 = sample_poly_cbd::<2>(bytes);
    
    let bytes2 = ByteVec::Vec128([99u8; 128]);
    let result2 = sample_poly_cbd::<2>(bytes2);
    
    for i in 0..256 {
        assert_eq!(result1[i], result2[i], "sample_poly_cbd should be deterministic");
    }
}

#[test]
fn test_sample_poly_cbd_no_alternating_zeros() {
    // Regression test for the alternating zeros bug
    use ml_kem_rs::bytevec::ByteVec;
    let bytes = ByteVec::Vec128([255u8; 128]); // Max entropy input
    let result = sample_poly_cbd::<2>(bytes);
    
    // Count how many coefficients are exactly zero
    let zero_count = (0..256).filter(|&i| result[i] == 0).count();
    
    // With max entropy input, we shouldn't have every other coefficient as zero
    // (that would be 128 zeros)
    assert!(zero_count < 128, 
            "Too many zeros ({}), suggests alternating zero pattern bug", zero_count);
    
    // More specifically, check that we don't have the pattern [x, 0, y, 0, z, 0, ...]
    let mut alternating_pattern = true;
    for i in (1..256).step_by(2) {
        if result[i] != 0 {
            alternating_pattern = false;
            break;
        }
    }
    assert!(!alternating_pattern, "Found alternating zero pattern - bytes_to_bits bug likely present");
}

#[test]
fn test_sample_poly_cbd_eta2_bounded_output() {
    // For eta=2, coefficients should be in range [-2, 2] before modular reduction
    // After reduction mod q, they should be in specific ranges
    use ml_kem_rs::bytevec::ByteVec;
    let bytes = ByteVec::Vec128([0u8; 128]);
    let result = sample_poly_cbd::<2>(bytes);
    
    // All coefficients should be small (close to 0 or close to q)
    for i in 0..256 {
        let c = result[i];
        assert!(c <= 2 || c >= 3327, 
                "CBD eta=2 coefficient {} = {} should be in {{0,1,2,3327,3328}}", i, c);
    }
}
