mod cryptographic;
mod kpke;

struct ParameterSet {
    n: u32,
    k: u32,
    q: u32,
    eta1: u32,
    eta2: u32,
    du: u32,
    dv: u32
}

const ML_KEM_512: ParameterSet = ParameterSet { n: 256, k: 2, q: 3329, eta1: 3, eta2: 2, du: 10, dv: 4 };
const ML_KEM_768: ParameterSet = ParameterSet { n: 256, k: 3, q: 3329, eta1: 2, eta2: 2, du: 10, dv: 4 };
const ML_KEM_1024: ParameterSet = ParameterSet { n: 256, k: 4, q: 3329, eta1: 2, eta2: 2, du: 11, dv: 5 };

const SELECTED_PARAMETER_SET: &ParameterSet = &ML_KEM_768;
fn main() {


}
