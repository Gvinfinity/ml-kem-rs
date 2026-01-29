pub mod cryptographic {
    use std::{array, io::Read};

    use sha3::{Shake256, digest::{ExtendableOutput, Update}};

    enum ReturnVec {
        Vec128([u8; 128]),
        Vec192([u8; 192])
    }

    fn prf<const N: usize>(s: [u8; 4], b: u8) -> ReturnVec { 
        assert!(N == 2 || N == 3, "n should be 2 or 3");

        let mut hasher = Shake256::default();
        hasher.update(&s);
        hasher.update(array::from_ref(&b));
        
        let mut reader = hasher.finalize_xof();

        if N == 2 {
            let mut output = [0u8; 128];
            reader.read(&mut output).expect("Failed");
            return ReturnVec::Vec128(output);
        } else {
            let mut output = [0u8; 192];
            reader.read(&mut output).expect("Failed");
            return ReturnVec::Vec192(output);
        }
    }
}
