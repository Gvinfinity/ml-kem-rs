use crate::kpke::key_gen;

mod cryptographic;
mod kpke;
mod algebraic;
mod auxiliary;
mod polynomial;
mod bytevec;

struct ParameterSet {
    n: usize,
    k: u32,
    q: u32,
    eta1: usize,
    eta2: usize,
    du: u32,
    dv: u32
}

const ML_KEM_512: ParameterSet = ParameterSet { n: 256, k: 2, q: 3329, eta1: 3, eta2: 2, du: 10, dv: 4 };
const ML_KEM_768: ParameterSet = ParameterSet { n: 256, k: 3, q: 3329, eta1: 2, eta2: 2, du: 10, dv: 4 };
const ML_KEM_1024: ParameterSet = ParameterSet { n: 256, k: 4, q: 3329, eta1: 2, eta2: 2, du: 11, dv: 5 };

const SELECTED_PARAMETER_SET: &ParameterSet = &ML_KEM_768;
fn main() {
    let mut buf = [0u8;32];
    let _ = getrandom::fill(&mut buf);
    
    let k = key_gen(buf);

    println!("{:?}", k);
}
