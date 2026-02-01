use crate::{SELECTED_PARAMETER_SET, auxiliary::get_zeta_from_index, polynomial::Poly16};
use sha3::{Shake128, digest::{ExtendableOutput, Update, XofReader}};

pub fn ntt(f: Poly16) -> Poly16 {
    let mut transformed = f.clone();

    let mut i = 1;
    let mut len = 128;
    while len >= 2 {
        let mut start = 0;
        while start < 256 {
            let zeta = get_zeta_from_index(i);
            i += 1;

            let mut j = start;
            while j  < start + len {
                let t = (zeta * transformed[j + len]) % SELECTED_PARAMETER_SET.q as u16;
                transformed[j + len] = (transformed[j] - t) % SELECTED_PARAMETER_SET.q as u16;
                transformed[j] = (transformed[j] + t) % SELECTED_PARAMETER_SET.q as u16;

                j += 1;
            }

            start += 2*len;
        }

        len /= 2;
    }
    
    transformed
}

pub fn ntt_inv(ft: Poly16) -> Poly16 {
    let mut inverse = ft.clone();

    let mut i = 127;
    
    let mut len = 2;
    while len <= 128 {
        let mut start = 0;
        while start < 256 {
            let zeta = get_zeta_from_index(i);
            i -= 1;

            let mut j = start;
            while j < start + len {
                let t = inverse[j];
                inverse[j] = (t + inverse[j + len]) % SELECTED_PARAMETER_SET.q as u16;
                inverse[j + len] = (zeta*(inverse[j+len] - t)) % SELECTED_PARAMETER_SET.q as u16;

                j += 1;
            }

            start += 2*len;
        }

        len *= 2;
    }

    inverse = (inverse*3303) % SELECTED_PARAMETER_SET.q as u16;
    inverse
}

pub fn sample_ntt(rho: [u8;32], b1: u8, b2: u8) -> Poly16 {
    let mut out = [0u16; SELECTED_PARAMETER_SET.n];

    let mut ctx = Shake128::default();
    ctx.update(&rho);
    ctx.update(&[b1, b2]);
    let mut reader = ctx.finalize_xof();
    let mut j = 0;

    while j < 256 {
        let c = reader.read_boxed(3);
        let d1 = c[0] + 256*(c[1]%16);
        let d2 = c[1]/16 + 16*c[2];
        if d1 < SELECTED_PARAMETER_SET.q as u8 {
            out[j] = d1 as u16;
            j += 1;
        }
        if d2 < SELECTED_PARAMETER_SET.q as u8 {
            out[j] = d2 as u16;
            j += 1;
        }
    }
    Poly16::new(&out)
}