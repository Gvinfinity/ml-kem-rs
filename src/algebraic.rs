use crate::{SELECTED_PARAMETER_SET, auxiliary::{bytes_to_bits, get_zeta_from_index}, bytevec::ByteVec, polynomial::Poly16};
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
                let t = ((zeta as u32 * transformed[j + len] as u32) % SELECTED_PARAMETER_SET.q as u32) as u16;
                transformed[j + len] = (transformed[j] as i32 - t as i32).rem_euclid(SELECTED_PARAMETER_SET.q as i32) as u16;
                transformed[j] = ((transformed[j] as u32 + t as u32) % SELECTED_PARAMETER_SET.q as u32) as u16;

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
                inverse[j + len] = (zeta*(inverse[j+len] - t)).rem_euclid(SELECTED_PARAMETER_SET.q as u16);

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

    while j < 255 {
        let c = reader.read_boxed(3);
        let d1 = c[0] as u16 + 256*(c[1]%16) as u16;
        let d2: u16 = (c[1]/16) as u16 + 16*c[2] as u16;
        if d1 < SELECTED_PARAMETER_SET.q as u16 {
            out[j] = d1 as u16;
            j += 1;
        }
        if d2 < SELECTED_PARAMETER_SET.q as u16 {
            out[j] = d2 as u16;
            j += 1;
        }
    }
    Poly16::new(&out)
}

pub fn sample_poly_cbd<const ETA: usize>(bytes: ByteVec) -> Poly16 {
    assert!(ETA == 2 || ETA == 3, "n should be 2 or 3");

    let bytes_slice = match &bytes {
        ByteVec::Vec128(arr) => &arr[..],
        ByteVec::Vec192(arr) => &arr[..],
    };

    let b: Vec<u8> = bytes_to_bits(bytes_slice);
    let mut f = [0u16;SELECTED_PARAMETER_SET.n];

    let eta = ETA;

    let mut i = 0;
    while i < 256 {
        let mut x = 0i32;
        let mut y = 0i32;
        
        for j in 0..eta {
            x += b[2*i*eta + j] as i32;
            y += b[2*i*eta + eta + j] as i32;
        }

        f[i] = (((x - y) as i16).rem_euclid(SELECTED_PARAMETER_SET.q as i16)) as u16;

        i += 1;
    }

    Poly16::new(&f)
}