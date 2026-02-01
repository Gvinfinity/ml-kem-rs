use ml_kem_rs::bytevec::ByteVec;

#[test]
fn test_bytevec_vec128_creation() {
    let arr = [42u8; 128];
    let vec = ByteVec::Vec128(arr);
    
    match vec {
        ByteVec::Vec128(data) => assert_eq!(data.len(), 128),
        _ => panic!("Expected Vec128 variant"),
    }
}

#[test]
fn test_bytevec_vec192_creation() {
    let arr = [99u8; 192];
    let vec = ByteVec::Vec192(arr);
    
    match vec {
        ByteVec::Vec192(data) => assert_eq!(data.len(), 192),
        _ => panic!("Expected Vec192 variant"),
    }
}

#[test]
fn test_bytevec_vec128_pattern_matching() {
    let arr = [1u8; 128];
    let vec = ByteVec::Vec128(arr);
    
    if let ByteVec::Vec128(data) = vec {
        assert_eq!(data[0], 1);
        assert_eq!(data[127], 1);
    } else {
        panic!("Failed to match Vec128");
    }
}

#[test]
fn test_bytevec_vec192_pattern_matching() {
    let arr = [2u8; 192];
    let vec = ByteVec::Vec192(arr);
    
    if let ByteVec::Vec192(data) = vec {
        assert_eq!(data[0], 2);
        assert_eq!(data[191], 2);
    } else {
        panic!("Failed to match Vec192");
    }
}

#[test]
fn test_bytevec_enum_size() {
    use std::mem::size_of;
    
    // ByteVec should be able to hold either variant
    let size = size_of::<ByteVec>();
    
    // Should be at least as large as the largest variant (192 bytes)
    assert!(size >= 192, "ByteVec size is too small: {}", size);
}

#[test]
fn test_bytevec_different_values() {
    let mut arr128 = [0u8; 128];
    let mut arr192 = [0u8; 192];
    
    for i in 0..128 {
        arr128[i] = i as u8;
    }
    for i in 0..192 {
        arr192[i] = i as u8;
    }
    
    let vec128 = ByteVec::Vec128(arr128);
    let vec192 = ByteVec::Vec192(arr192);
    
    match vec128 {
        ByteVec::Vec128(data) => {
            for i in 0..128 {
                assert_eq!(data[i], i as u8);
            }
        }
        _ => panic!("Wrong variant"),
    }
    
    match vec192 {
        ByteVec::Vec192(data) => {
            for i in 0..192 {
                assert_eq!(data[i], i as u8);
            }
        }
        _ => panic!("Wrong variant"),
    }
}
