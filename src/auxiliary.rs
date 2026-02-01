use crate::SELECTED_PARAMETER_SET;

pub const fn get_bit_reversal_lookup() -> [u8;128] {
    let mut table = [0u8;128];
    let mut i = 0;

    while i < 128 {
        let mut reversing = 0u8;
        reversing |= (i & 1) << 6;
        reversing |= ((i >> 1) & 1) << 5;
        reversing |= ((i >> 2) & 1) << 4;
        reversing |= ((i >> 3) & 1) << 3;
        reversing |= ((i >> 4) & 1) << 2;
        reversing |= ((i >> 5) & 1 ) << 1;
        reversing |= i >> 6;

        table[i as usize] = reversing;

        i += 1;
    }

    return table;
}

const fn modpow(base: u8, exponent: u8, modulus: u32) -> u16 {
    let mut result = 1;
    let mut b: u32 = base as u32 % modulus;
    let mut e = exponent;
    
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % modulus;
        }
        e = e >> 1;
        b = (b * b) % modulus;
    }

    result as u16
}

pub fn get_zeta_from_index(i: usize) -> u16 {
    const ZETA_LUT: [u16; 128] = get_zeta_lut();

    ZETA_LUT[i]
}

const fn get_zeta_lut() -> [u16; 128] {
    let mut zeta_list = [0u16;128];
    
    let mut i = 0;

    while i < 128 {
        zeta_list[i] = modpow(17u8, bit_rev(i as u8), SELECTED_PARAMETER_SET.q as u32);
        i += 1;
    }

    zeta_list
}

pub const fn bit_rev(n: u8) -> u8 {
    const LUT: [u8;128] = get_bit_reversal_lookup();
    
    LUT[n as usize]
}
