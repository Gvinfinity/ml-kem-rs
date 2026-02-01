use crate::{SELECTED_PARAMETER_SET, algebraic::{ntt, sample_ntt, sample_poly_cbd}, cryptographic::{g, prf}, polynomial::Poly16};

pub fn key_gen(d: [u8;32]){
    let (rho, sigma) = g(&d);
    let n = 0;

    // TODO: Properly initialize as null
    let mut a = [[Poly16::new(&[0;SELECTED_PARAMETER_SET.n as usize]); SELECTED_PARAMETER_SET.k as usize];SELECTED_PARAMETER_SET.k as usize];
    let mut s = [Poly16::new(&[0;SELECTED_PARAMETER_SET.n as usize]);SELECTED_PARAMETER_SET.k as usize];
    let mut e = [Poly16::new(&[0;SELECTED_PARAMETER_SET.n as usize]);SELECTED_PARAMETER_SET.k as usize];

    for i in 0..SELECTED_PARAMETER_SET.k as usize{
        for j in 0..SELECTED_PARAMETER_SET.k as usize {
            a[i][j] = sample_ntt(rho, j as u8, i as u8);
        }
    }

    for i in 0..SELECTED_PARAMETER_SET.k as usize {
        let v = prf::<{ SELECTED_PARAMETER_SET.eta1 }>(sigma, n);
        s[i] = sample_poly_cbd::<{ SELECTED_PARAMETER_SET.eta1 }>(v);
    }

    for i in 0..SELECTED_PARAMETER_SET.k as usize {
        let v = prf::<{ SELECTED_PARAMETER_SET.eta1 }>(sigma, n);
        e[i] = sample_poly_cbd::<{ SELECTED_PARAMETER_SET.eta1 }>(v);
    }

    let s_ntt= s.map(|c| ntt(c));
    let e_ntt = s.map(|c| ntt(c));

    println!("{:?}", s_ntt);
    println!("{:?}", e_ntt);
}