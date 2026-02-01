use ml_kem_rs::auxiliary::*;

#[test]
fn test_bit_reversal_lookup_table() {
    let lut = get_bit_reversal_lookup();
    
    // Test that table has correct size
    assert_eq!(lut.len(), 128);
    
    // Test specific known values
    assert_eq!(lut[0], 0);     // 0b0000000 -> 0b0000000
    assert_eq!(lut[1], 64);    // 0b0000001 -> 0b1000000
    assert_eq!(lut[2], 32);    // 0b0000010 -> 0b0100000
    assert_eq!(lut[127], 127); // 0b1111111 -> 0b1111111
    
    // Test that reversing twice gives original
    for i in 0..128 {
        let reversed = lut[i as usize];
        let double_reversed = lut[reversed as usize];
        assert_eq!(i, double_reversed, "Double reversal of {} failed", i);
    }
}

#[test]
fn test_bit_rev() {
    // Test boundary cases
    assert_eq!(bit_rev(0), 0);
    assert_eq!(bit_rev(127), 127);
    
    // Test specific values
    assert_eq!(bit_rev(1), 64);
    assert_eq!(bit_rev(64), 1);
    assert_eq!(bit_rev(2), 32);
    assert_eq!(bit_rev(32), 2);
    
    // Test that bit_rev is self-inverse
    for i in 0..128u8 {
        assert_eq!(bit_rev(bit_rev(i)), i, "bit_rev is not self-inverse for {}", i);
    }
}

#[test]
fn test_bytes_to_bits() {
    // Test single byte
    let bytes = vec![0b10110100];
    let bits = bytes_to_bits(&bytes);
    assert_eq!(bits.len(), 8);
    assert_eq!(bits, vec![0, 0, 1, 0, 1, 0, 1, 1]); // LSB first
    
    // Test zero byte
    let bytes = vec![0];
    let bits = bytes_to_bits(&bytes);
    assert_eq!(bits, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    
    // Test all ones
    let bytes = vec![255];
    let bits = bytes_to_bits(&bytes);
    assert_eq!(bits, vec![1, 1, 1, 1, 1, 1, 1, 1]);
    
    // Test multiple bytes
    let bytes = vec![0b10101010, 0b11001100];
    let bits = bytes_to_bits(&bytes);
    assert_eq!(bits.len(), 16);
    assert_eq!(&bits[0..8], &[0, 1, 0, 1, 0, 1, 0, 1]);
    assert_eq!(&bits[8..16], &[0, 0, 1, 1, 0, 0, 1, 1]);
}

#[test]
fn test_bits_to_bytes() {
    // Test single byte
    let bits = vec![0, 0, 1, 0, 1, 0, 1, 1];
    let bytes = bits_to_bytes(&bits);
    assert_eq!(bytes.len(), 1);
    assert_eq!(bytes[0], 0b10110100);
    
    // Test all zeros
    let bits = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let bytes = bits_to_bytes(&bits);
    assert_eq!(bytes[0], 0);
    
    // Test all ones
    let bits = vec![1, 1, 1, 1, 1, 1, 1, 1];
    let bytes = bits_to_bytes(&bits);
    assert_eq!(bytes[0], 255);
}

#[test]
fn test_bytes_bits_roundtrip() {
    // Test that bytes -> bits -> bytes preserves data
    let original_bytes = vec![42, 128, 255, 0, 127, 200];
    let bits = bytes_to_bits(&original_bytes);
    let recovered_bytes = bits_to_bytes(&bits);
    assert_eq!(original_bytes, recovered_bytes);
}

#[test]
fn test_get_zeta_from_index() {
    // Test that function returns valid values in range [0, q)
    for i in 0..128 {
        let zeta = get_zeta_from_index(i);
        assert!(zeta < 3329, "Zeta value {} at index {} is out of range", zeta, i);
    }
    
    // Test first value (should be 1)
    assert_eq!(get_zeta_from_index(0), 1);
    
    // Test that all values are precomputed correctly (non-zero except possibly some)
    let zeta_64 = get_zeta_from_index(64);
    assert!(zeta_64 > 0 && zeta_64 < 3329);
}

#[test]
fn test_zeta_values_consistency() {
    // Test that zetas are computed consistently
    for i in 0..128 {
        let zeta1 = get_zeta_from_index(i);
        let zeta2 = get_zeta_from_index(i);
        assert_eq!(zeta1, zeta2, "Zeta values not consistent for index {}", i);
    }
}

#[test]
fn test_bytes_to_bits_no_alternating_zeros() {
    // Regression test for the bug that caused alternating zeros
    let bytes = vec![255u8; 32]; // All bits should be 1
    let bits = bytes_to_bits(&bytes);
    
    // Count zeros and ones
    let zero_count = bits.iter().filter(|&&b| b == 0).count();
    let one_count = bits.iter().filter(|&&b| b == 1).count();
    
    // All bits should be 1 for input of all 255s
    assert_eq!(zero_count, 0, "Found unexpected zeros in bits");
    assert_eq!(one_count, 256, "Expected 256 ones");
    
    // Test pattern doesn't create alternating zeros
    let bytes = vec![0b10101010u8; 4];
    let bits = bytes_to_bits(&bytes);
    let expected = vec![0, 1, 0, 1, 0, 1, 0, 1];
    for i in 0..4 {
        assert_eq!(&bits[i*8..(i+1)*8], &expected[..], "Pattern mismatch at byte {}", i);
    }
}
