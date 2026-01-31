use crate::{SELECTED_PARAMETER_SET, cryptographic::{g, prf}};

fn KeyGen(d: [u8;32]){
    let (rho, sigma) = g(&d);
    let n = 0;
    let mut a: Vec<Vec<[u8;SELECTED_PARAMETER_SET.n as usize]>> = vec![vec![]; SELECTED_PARAMETER_SET.k as usize];
    let mut s: Vec<[u8;SELECTED_PARAMETER_SET.n as usize]>;

    for i in 0..SELECTED_PARAMETER_SET.k as usize{
        for j in 0..SELECTED_PARAMETER_SET.k as usize {
            a[i][j] = sampleNTT(rho, j as u8, i as u8);
        }
    }

    for i in 0..SELECTED_PARAMETER_SET.k as usize {
        s[i] = samplePolyCBD(SELECTED_PARAMETER_SET.eta1, prf(SELECTED_PARAMETER_SET.eta1 as usize, sigma, n))
    }

}