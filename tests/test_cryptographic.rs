use ml_kem_rs::{cryptographic::*, bytevec::ByteVec};

#[test]
fn test_prf_eta2_output_size() {
    let s = [42u8; 32];
    let b = 0u8;
    let result = prf::<2>(s, b);
    
    match result {
        ByteVec::Vec128(arr) => assert_eq!(arr.len(), 128),
        ByteVec::Vec192(_) => panic!("Expected Vec128 for eta=2"),
    }
}

#[test]
fn test_prf_eta3_output_size() {
    let s = [42u8; 32];
    let b = 0u8;
    let result = prf::<3>(s, b);
    
    match result {
        ByteVec::Vec192(arr) => assert_eq!(arr.len(), 192),
        ByteVec::Vec128(_) => panic!("Expected Vec192 for eta=3"),
    }
}

#[test]
fn test_prf_deterministic() {
    let s = [123u8; 32];
    let b = 5u8;
    
    let result1 = prf::<2>(s, b);
    let result2 = prf::<2>(s, b);
    
    match (result1, result2) {
        (ByteVec::Vec128(arr1), ByteVec::Vec128(arr2)) => {
            assert_eq!(arr1, arr2, "PRF should be deterministic");
        }
        _ => panic!("Unexpected ByteVec variant"),
    }
}

#[test]
fn test_prf_different_inputs_different_outputs() {
    let s = [1u8; 32];
    
    let result1 = prf::<2>(s, 0);
    let result2 = prf::<2>(s, 1);
    
    match (result1, result2) {
        (ByteVec::Vec128(arr1), ByteVec::Vec128(arr2)) => {
            assert_ne!(arr1, arr2, "Different inputs should produce different outputs");
        }
        _ => panic!("Unexpected ByteVec variant"),
    }
}

#[test]
fn test_h_output_size() {
    let input = b"test input";
    let result = h(input);
    assert_eq!(result.len(), 32);
}

#[test]
fn test_h_deterministic() {
    let input = b"deterministic test";
    let result1 = h(input);
    let result2 = h(input);
    assert_eq!(result1, result2, "Hash function h should be deterministic");
}

#[test]
fn test_h_different_inputs() {
    let input1 = b"input one";
    let input2 = b"input two";
    let result1 = h(input1);
    let result2 = h(input2);
    assert_ne!(result1, result2, "Different inputs should produce different hashes");
}

#[test]
fn test_h_empty_input() {
    let input = b"";
    let result = h(input);
    assert_eq!(result.len(), 32);
    // SHA3-256("") is known
}

#[test]
fn test_j_output_size() {
    let input = b"test input";
    let result = j(input);
    assert_eq!(result.len(), 32);
}

#[test]
fn test_j_deterministic() {
    let input = b"deterministic test";
    let result1 = j(input);
    let result2 = j(input);
    assert_eq!(result1, result2, "Hash function j should be deterministic");
}

#[test]
fn test_j_different_inputs() {
    let input1 = b"input one";
    let input2 = b"input two";
    let result1 = j(input1);
    let result2 = j(input2);
    assert_ne!(result1, result2, "Different inputs should produce different outputs");
}

#[test]
fn test_g_output_sizes() {
    let input = b"test";
    let (rho, sigma) = g(input);
    assert_eq!(rho.len(), 32);
    assert_eq!(sigma.len(), 32);
}

#[test]
fn test_g_deterministic() {
    let input = b"deterministic";
    let (rho1, sigma1) = g(input);
    let (rho2, sigma2) = g(input);
    assert_eq!(rho1, rho2, "First output should be deterministic");
    assert_eq!(sigma1, sigma2, "Second output should be deterministic");
}

#[test]
fn test_g_different_inputs() {
    let input1 = b"input 1";
    let input2 = b"input 2";
    let (rho1, sigma1) = g(input1);
    let (rho2, sigma2) = g(input2);
    assert_ne!(rho1, rho2, "Different inputs should produce different rho");
    assert_ne!(sigma1, sigma2, "Different inputs should produce different sigma");
}

#[test]
fn test_g_outputs_different() {
    let input = b"test";
    let (rho, sigma) = g(input);
    // rho and sigma should be different (they're from different halves of the hash)
    assert_ne!(rho, sigma, "The two outputs of g should be different");
}

#[test]
fn test_prf_all_bytes_nonzero_probability() {
    // Test that PRF doesn't produce all zeros (extremely unlikely)
    let s = [0u8; 32];
    let result = prf::<2>(s, 0);
    
    match result {
        ByteVec::Vec128(arr) => {
            let all_zero = arr.iter().all(|&x| x == 0);
            assert!(!all_zero, "PRF output should not be all zeros");
        }
        _ => panic!("Unexpected ByteVec variant"),
    }
}
