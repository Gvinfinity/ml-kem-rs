// Integration tests for ML-KEM-RS
// These tests verify the overall correctness of the implementation

use ml_kem_rs::{SELECTED_PARAMETER_SET, ML_KEM_512, ML_KEM_768, ML_KEM_1024};

#[test]
fn test_parameter_sets_defined() {
    // Verify all three parameter sets are properly defined
    assert_eq!(ML_KEM_512.n, 256);
    assert_eq!(ML_KEM_512.k, 2);
    assert_eq!(ML_KEM_512.q, 3329);
    
    assert_eq!(ML_KEM_768.n, 256);
    assert_eq!(ML_KEM_768.k, 3);
    assert_eq!(ML_KEM_768.q, 3329);
    
    assert_eq!(ML_KEM_1024.n, 256);
    assert_eq!(ML_KEM_1024.k, 4);
    assert_eq!(ML_KEM_1024.q, 3329);
}

#[test]
fn test_selected_parameter_set_is_768() {
    assert_eq!(SELECTED_PARAMETER_SET.k, 3);
    assert_eq!(SELECTED_PARAMETER_SET.eta1, 2);
}

#[test]
fn test_fips_203_constants() {
    // Verify FIPS 203 constants are correct
    assert_eq!(SELECTED_PARAMETER_SET.n, 256, "n should be 256 for all ML-KEM variants");
    assert_eq!(SELECTED_PARAMETER_SET.q, 3329, "q should be 3329 for all ML-KEM variants");
}

#[test]
fn test_parameter_set_eta_values() {
    // ML-KEM-512 uses eta1=3, eta2=2
    assert_eq!(ML_KEM_512.eta1, 3);
    assert_eq!(ML_KEM_512.eta2, 2);
    
    // ML-KEM-768 and ML-KEM-1024 use eta1=2, eta2=2
    assert_eq!(ML_KEM_768.eta1, 2);
    assert_eq!(ML_KEM_768.eta2, 2);
    assert_eq!(ML_KEM_1024.eta1, 2);
    assert_eq!(ML_KEM_1024.eta2, 2);
}

#[test]
fn test_parameter_set_compression_values() {
    // Verify du and dv compression parameters
    assert_eq!(ML_KEM_512.du, 10);
    assert_eq!(ML_KEM_512.dv, 4);
    
    assert_eq!(ML_KEM_768.du, 10);
    assert_eq!(ML_KEM_768.dv, 4);
    
    assert_eq!(ML_KEM_1024.du, 11);
    assert_eq!(ML_KEM_1024.dv, 5);
}

#[test]
fn test_modulus_is_prime() {
    // 3329 should be prime (FIPS 203 requirement)
    let q = 3329u32;
    
    // Simple primality check
    for i in 2..((q as f64).sqrt() as u32 + 1) {
        assert_ne!(q % i, 0, "3329 should be prime");
    }
}

#[test]
fn test_polynomial_ring_dimension() {
    // The polynomial ring dimension should be 256 for all variants
    assert_eq!(ML_KEM_512.n, 256);
    assert_eq!(ML_KEM_768.n, 256);
    assert_eq!(ML_KEM_1024.n, 256);
}

#[test]
fn test_security_levels() {
    // ML-KEM-512 targets NIST Level 1 (k=2)
    assert_eq!(ML_KEM_512.k, 2);
    
    // ML-KEM-768 targets NIST Level 3 (k=3)
    assert_eq!(ML_KEM_768.k, 3);
    
    // ML-KEM-1024 targets NIST Level 5 (k=4)
    assert_eq!(ML_KEM_1024.k, 4);
}

#[cfg(test)]
mod full_workflow_tests {
    use ml_kem_rs::{kpke::key_gen, cryptographic::*, algebraic::*};
    
    #[test]
    fn test_complete_key_generation_workflow() {
        // Test the complete key generation workflow
        let mut d = [0u8; 32];
        getrandom::fill(&mut d).expect("Failed to generate random seed");
        
        // This should execute the complete key generation
        let _ = key_gen(d);
    }
    
    #[test]
    fn test_hash_functions_work() {
        let input = b"test data for ML-KEM";
        
        let h_output = h(input);
        assert_eq!(h_output.len(), 32);
        
        let j_output = j(input);
        assert_eq!(j_output.len(), 32);
        
        let (rho, sigma) = g(input);
        assert_eq!(rho.len(), 32);
        assert_eq!(sigma.len(), 32);
    }
    
    #[test]
    fn test_ntt_operations_work() {
        use ml_kem_rs::polynomial::Poly16;
        
        let mut coeffs = [0u16; 256];
        for i in 0..256 {
            coeffs[i] = (i % 100) as u16;
        }
        let poly = Poly16::new(&coeffs);
        
        // Test NTT
        let ntt_result = ntt(poly);
        
        // Test inverse NTT
        let inv_result = ntt_inv(ntt_result);
        
        // Verify roundtrip
        for i in 0..256 {
            assert_eq!(inv_result[i], poly[i]);
        }
    }
    
    #[test]
    fn test_sampling_functions_work() {
        use ml_kem_rs::bytevec::ByteVec;
        
        // Test sample_ntt
        let rho = [1u8; 32];
        let sample = sample_ntt(rho, 0, 0);
        
        // Verify all coefficients are in range
        for i in 0..256 {
            assert!(sample[i] < 3329);
        }
        
        // Test sample_poly_cbd
        let bytes = ByteVec::Vec128([42u8; 128]);
        let cbd_sample = sample_poly_cbd::<2>(bytes);
        
        for i in 0..256 {
            assert!(cbd_sample[i] < 3329);
        }
    }
}
