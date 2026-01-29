pub mod cryptographic {
    use std::{array, io::Read};

    use sha3::{Digest, Sha3_256, Sha3_512, Shake256, digest::{ExtendableOutput, Update}};

    enum ReturnVec {
        Vec128([u8; 128]),
        Vec192([u8; 192])
    }

    pub fn prf(eta: usize, s: [u8; 32], b: u8) -> ReturnVec { 
        assert!(eta == 2 || eta == 3, "n should be 2 or 3");

        let mut hasher = Shake256::default();
        hasher.update(&s);
        hasher.update(array::from_ref(&b));
        
        let mut reader = hasher.finalize_xof();

        if eta == 2 {
            let mut output = [0u8; 128];
            reader.read(&mut output).expect("Failed");
            return ReturnVec::Vec128(output);
        } else {
            let mut output = [0u8; 192];
            reader.read(&mut output).expect("Failed");
            return ReturnVec::Vec192(output);
        }
    }

    pub fn h(s: &[u8]) -> [u8;32] {
        let mut hasher = Sha3_256::default();

        Update::update(&mut hasher, &s);
        
        return hasher.finalize().into();
    }

    pub fn j(s: &[u8]) -> [u8;32] {
        let mut hasher = Shake256::default();
        let mut output = [0u8; 32];

        hasher.update(&s);
        hasher.finalize_xof_into(&mut output);

        return output;
    }

    pub fn g(c: &[u8]) -> ([u8;32], [u8;32]) {
        let mut hasher = Sha3_512::default();
        Update::update(&mut hasher, &c);

        let hash: [u8; 64] = hasher.finalize().into();
        let (left, right) = hash.split_at(32);
        let tuple = (left.try_into().unwrap(), right.try_into().unwrap());
        return tuple
    }
}
