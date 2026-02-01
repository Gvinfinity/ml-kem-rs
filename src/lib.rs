// Library interface for ml-kem-rs to enable testing

pub mod cryptographic;
pub mod kpke;
pub mod algebraic;
pub mod auxiliary;
pub mod polynomial;
pub mod bytevec;

pub struct ParameterSet {
    pub n: usize,
    pub k: u32,
    pub q: u32,
    pub eta1: usize,
    pub eta2: usize,
    pub du: u32,
    pub dv: u32
}

pub const ML_KEM_512: ParameterSet = ParameterSet { n: 256, k: 2, q: 3329, eta1: 3, eta2: 2, du: 10, dv: 4 };
pub const ML_KEM_768: ParameterSet = ParameterSet { n: 256, k: 3, q: 3329, eta1: 2, eta2: 2, du: 10, dv: 4 };
pub const ML_KEM_1024: ParameterSet = ParameterSet { n: 256, k: 4, q: 3329, eta1: 2, eta2: 2, du: 11, dv: 5 };

pub const SELECTED_PARAMETER_SET: &ParameterSet = &ML_KEM_768;
