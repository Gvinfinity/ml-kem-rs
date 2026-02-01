use ml_kem_rs::{polynomial::*, SELECTED_PARAMETER_SET};

#[test]
fn test_polynomial_creation() {
    let coeffs = [1u16; 256];
    let poly = Poly16::new(&coeffs);
    
    for i in 0..256 {
        assert_eq!(poly[i], 1);
    }
}

#[test]
fn test_polynomial_indexing() {
    let mut coeffs = [0u16; 256];
    for i in 0..256 {
        coeffs[i] = i as u16;
    }
    let poly = Poly16::new(&coeffs);
    
    for i in 0..256 {
        assert_eq!(poly[i], i as u16);
    }
}

#[test]
fn test_polynomial_addition() {
    let coeffs1 = [100u16; 256];
    let coeffs2 = [50u16; 256];
    
    let poly1 = Poly16::new(&coeffs1);
    let poly2 = Poly16::new(&coeffs2);
    
    let result = poly1 + poly2;
    
    for i in 0..256 {
        assert_eq!(result[i], 150);
    }
}

#[test]
fn test_polynomial_addition_overflow() {
    let coeffs1 = [65535u16; 256]; // Max u16
    let coeffs2 = [1u16; 256];
    
    let poly1 = Poly16::new(&coeffs1);
    let poly2 = Poly16::new(&coeffs2);
    
    let result = poly1 + poly2;
    
    // Should wrap around
    for i in 0..256 {
        assert_eq!(result[i], 0);
    }
}

#[test]
fn test_polynomial_scalar_multiplication() {
    let coeffs = [10u16; 256];
    let poly = Poly16::new(&coeffs);
    
    let result = poly * 5u16;
    
    for i in 0..256 {
        assert_eq!(result[i], 50);
    }
}

#[test]
fn test_polynomial_scalar_multiplication_zero() {
    let coeffs = [42u16; 256];
    let poly = Poly16::new(&coeffs);
    
    let result = poly * 0u16;
    
    for i in 0..256 {
        assert_eq!(result[i], 0);
    }
}

#[test]
fn test_polynomial_modulo() {
    let mut coeffs = [0u16; 256];
    for i in 0..256 {
        coeffs[i] = (i * 100) as u16;
    }
    let poly = Poly16::new(&coeffs);
    
    let result = poly % 3329u16;
    
    for i in 0..256 {
        assert!(result[i] < 3329, "Coefficient {} should be less than 3329", result[i]);
        assert_eq!(result[i], ((i * 100) % 3329) as u16);
    }
}

#[test]
fn test_polynomial_modulo_q() {
    // Test modulo reduction by q (3329)
    let mut coeffs = [0u16; 256];
    coeffs[0] = 3329;
    coeffs[1] = 3330;
    coeffs[2] = 6658;
    coeffs[3] = 100;
    
    let poly = Poly16::new(&coeffs);
    let result = poly % (SELECTED_PARAMETER_SET.q as u16);
    
    assert_eq!(result[0], 0);
    assert_eq!(result[1], 1);
    assert_eq!(result[2], 0);
    assert_eq!(result[3], 100);
}

#[test]
fn test_polynomial_clone() {
    let coeffs = [42u16; 256];
    let poly1 = Poly16::new(&coeffs);
    let poly2 = poly1.clone();
    
    for i in 0..256 {
        assert_eq!(poly1[i], poly2[i]);
    }
}

#[test]
fn test_polynomial_copy() {
    let coeffs = [42u16; 256];
    let poly1 = Poly16::new(&coeffs);
    let poly2 = poly1; // Should copy, not move
    
    // Both should be accessible
    assert_eq!(poly1[0], 42);
    assert_eq!(poly2[0], 42);
}

#[test]
fn test_polynomial_size() {
    let coeffs = [0u16; 256];
    let poly = Poly16::new(&coeffs);
    
    // Verify we can access all 256 coefficients
    let _ = poly[255]; // Should not panic
}

#[test]
fn test_polynomial_addition_commutativity() {
    let coeffs1 = [123u16; 256];
    let coeffs2 = [456u16; 256];
    
    let poly1 = Poly16::new(&coeffs1);
    let poly2 = Poly16::new(&coeffs2);
    
    let result1 = poly1 + poly2;
    let result2 = poly2 + poly1;
    
    for i in 0..256 {
        assert_eq!(result1[i], result2[i]);
    }
}

#[test]
fn test_polynomial_addition_associativity() {
    let coeffs1 = [100u16; 256];
    let coeffs2 = [200u16; 256];
    let coeffs3 = [300u16; 256];
    
    let poly1 = Poly16::new(&coeffs1);
    let poly2 = Poly16::new(&coeffs2);
    let poly3 = Poly16::new(&coeffs3);
    
    let result1 = (poly1 + poly2) + poly3;
    let result2 = poly1 + (poly2 + poly3);
    
    for i in 0..256 {
        assert_eq!(result1[i], result2[i]);
    }
}

#[test]
fn test_polynomial_zero() {
    let coeffs = [0u16; 256];
    let poly = Poly16::new(&coeffs);
    
    for i in 0..256 {
        assert_eq!(poly[i], 0);
    }
}
