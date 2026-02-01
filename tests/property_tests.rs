// Property-based and fuzz-like tests for ML-KEM
// These tests verify mathematical properties and invariants

use ml_kem_rs::{
    polynomial::Poly16,
    algebraic::*,
    auxiliary::*,
    SELECTED_PARAMETER_SET,
};

#[test]
fn property_modulo_reduces_to_range() {
    // Property: a % q should always be in [0, q)
    for i in 0..1000 {
        let mut coeffs = [0u16; 256];
        for j in 0..256 {
            coeffs[j] = ((i * j + 1234) % 65536) as u16;
        }
        let poly = Poly16::new(&coeffs);
        let result = poly % (SELECTED_PARAMETER_SET.q as u16);
        
        for k in 0..256 {
            assert!(result[k] < SELECTED_PARAMETER_SET.q as u16,
                    "Coefficient out of range: {} >= {}", result[k], SELECTED_PARAMETER_SET.q);
        }
    }
}

#[test]
fn property_ntt_preserves_coefficient_count() {
    // Property: NTT should not change the number of coefficients
    for seed in 0..10 {
        let mut coeffs = [0u16; 256];
        for i in 0..256 {
            coeffs[i] = ((seed * i + 42) % 3329) as u16;
        }
        let poly = Poly16::new(&coeffs);
        let transformed = ntt(poly);
        
        // Should still be able to access all 256 coefficients
        for i in 0..256 {
            let _ = transformed[i]; // Should not panic
        }
    }
}

#[test]
fn property_bit_reversal_is_involutive() {
    // Property: bit_rev(bit_rev(x)) == x (involution)
    for i in 0..128u8 {
        let once = bit_rev(i);
        let twice = bit_rev(once);
        assert_eq!(i, twice, "Bit reversal is not involutive for {}", i);
    }
}

#[test]
fn property_sample_ntt_always_in_range() {
    // Property: sample_ntt should always produce coefficients in [0, q)
    for i in 0..5 {
        for j in 0..5 {
            let mut rho = [0u8; 32];
            for k in 0..32 {
                rho[k] = ((i * 31 + j * 17 + k) % 256) as u8;
            }
            
            let sample = sample_ntt(rho, i as u8, j as u8);
            
            for k in 0..256 {
                assert!(sample[k] < SELECTED_PARAMETER_SET.q as u16,
                        "Sample out of range at ({},{},{}): {}", i, j, k, sample[k]);
            }
        }
    }
}

#[test]
fn property_polynomial_addition_commutative() {
    // Property: a + b == b + a
    for i in 0..10 {
        let val_a = (i * 100) as u16;
        let val_b = (i * 200 + 50) as u16;
        
        let poly_a = Poly16::new(&[val_a; 256]);
        let poly_b = Poly16::new(&[val_b; 256]);
        
        let sum1 = poly_a + poly_b;
        let sum2 = poly_b + poly_a;
        
        for j in 0..256 {
            assert_eq!(sum1[j], sum2[j], "Addition not commutative");
        }
    }
}

#[test]
fn property_polynomial_addition_associative() {
    // Property: (a + b) + c == a + (b + c)
    let poly_a = Poly16::new(&[100u16; 256]);
    let poly_b = Poly16::new(&[200u16; 256]);
    let poly_c = Poly16::new(&[300u16; 256]);
    
    let sum1 = (poly_a + poly_b) + poly_c;
    let sum2 = poly_a + (poly_b + poly_c);
    
    for i in 0..256 {
        assert_eq!(sum1[i], sum2[i], "Addition not associative");
    }
}

#[test]
fn property_polynomial_scalar_distributive() {
    // Property: k * (a + b) == k*a + k*b (when no overflow)
    let poly_a = Poly16::new(&[10u16; 256]);
    let poly_b = Poly16::new(&[20u16; 256]);
    let k = 5u16;
    
    let sum = poly_a + poly_b;
    let scaled_sum = sum * k;
    
    let scaled_a = poly_a * k;
    let scaled_b = poly_b * k;
    let sum_scaled = scaled_a + scaled_b;
    
    for i in 0..256 {
        assert_eq!(scaled_sum[i], sum_scaled[i], "Scalar multiplication not distributive");
    }
}

#[test]
fn property_bytes_to_bits_size() {
    // Property: bytes_to_bits on n bytes produces 8*n bits
    for n in 1..20 {
        let bytes = vec![42u8; n];
        let bits = bytes_to_bits(&bytes);
        assert_eq!(bits.len(), n * 8, "Wrong number of bits for {} bytes", n);
    }
}

#[test]
fn property_zeta_values_in_range() {
    // Property: All zeta values should be in [0, q)
    for i in 0..128 {
        let zeta = get_zeta_from_index(i);
        assert!(zeta < SELECTED_PARAMETER_SET.q as u16,
                "Zeta[{}] = {} is out of range", i, zeta);
        assert!(zeta > 0 || i == 0, "Zeta[{}] = 0 (only allowed for index 0)", i);
    }
}

#[test]
fn property_zeta_computation_deterministic() {
    // Property: Zeta values should be deterministic
    for _ in 0..5 {
        for i in 0..128 {
            let zeta1 = get_zeta_from_index(i);
            let zeta2 = get_zeta_from_index(i);
            assert_eq!(zeta1, zeta2, "Zeta computation not deterministic");
        }
    }
}

#[test]
fn property_sample_poly_cbd_bounded() {
    // Property: CBD samples should be small (bounded by eta)
    use ml_kem_rs::bytevec::ByteVec;
    
    // For eta=2, coefficients should be in {0, 1, 2, q-2, q-1, q} (mod q)
    let bytes = ByteVec::Vec128([123u8; 128]);
    let sample = sample_poly_cbd::<2>(bytes);
    
    for i in 0..256 {
        let c = sample[i];
        // After modular reduction, small values stay small, negative become large
        assert!(c <= 2 || c >= SELECTED_PARAMETER_SET.q as u16 - 2,
                "CBD coefficient {} not bounded: {}", i, c);
    }
}

#[test]
fn property_ntt_output_bounded() {
    // Property: NTT output should be in [0, q)
    for seed in 0..10 {
        let mut coeffs = [0u16; 256];
        for i in 0..256 {
            coeffs[i] = ((seed * i + 7) % 3329) as u16;
        }
        let poly = Poly16::new(&coeffs);
        let transformed = ntt(poly);
        
        for i in 0..256 {
            assert!(transformed[i] < SELECTED_PARAMETER_SET.q as u16,
                    "NTT output {} out of range: {}", i, transformed[i]);
        }
    }
}

#[test]
fn stress_test_multiple_ntt_transforms() {
    // Stress test: Apply NTT multiple times with different inputs
    for seed in 0..20 {
        let mut coeffs = [0u16; 256];
        for i in 0..256 {
            coeffs[i] = ((seed * 13 + i * 7) % 3329) as u16;
        }
        let poly = Poly16::new(&coeffs);
        let _ = ntt(poly);
        // Should not panic or produce invalid values
    }
}

#[test]
fn stress_test_sample_ntt_many_times() {
    // Stress test: Sample many times with different seeds
    for i in 0..10 {
        let mut rho = [0u8; 32];
        for j in 0..32 {
            rho[j] = ((i * j + 42) % 256) as u8;
        }
        
        let sample = sample_ntt(rho, 0, 0);
        
        // Verify validity
        for k in 0..256 {
            assert!(sample[k] < 3329, "Invalid sample");
        }
    }
}

#[test]
fn edge_case_all_zero_polynomial() {
    // Edge case: All zero polynomial
    let poly = Poly16::new(&[0u16; 256]);
    let transformed = ntt(poly);
    
    // NTT of zero should be zero
    for i in 0..256 {
        assert_eq!(transformed[i], 0, "NTT of zero should be zero");
    }
}

#[test]
fn edge_case_all_one_polynomial() {
    // Edge case: All one polynomial
    let poly = Poly16::new(&[1u16; 256]);
    let transformed = ntt(poly);
    
    // Should produce valid output
    for i in 0..256 {
        assert!(transformed[i] < 3329, "NTT output out of range");
    }
}

#[test]
fn edge_case_max_coefficient_polynomial() {
    // Edge case: All coefficients at q-1
    let poly = Poly16::new(&[3328u16; 256]);
    let transformed = ntt(poly);
    
    for i in 0..256 {
        assert!(transformed[i] < 3329, "NTT output out of range");
    }
}
