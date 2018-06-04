use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use std::iter::repeat;

pub fn natural_xor_alt_1(x: &Natural, y: &Natural) -> Natural {
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if x.significant_bits() >= y.significant_bits() {
            Box::new(x.bits().zip(y.bits().chain(repeat(false))))
        } else {
            Box::new(x.bits().chain(repeat(false)).zip(y.bits()))
        };
    let mut or_bits = Vec::new();
    for (b, c) in bit_zip {
        or_bits.push(b ^ c);
    }
    Natural::from_bits_asc(&or_bits)
}

pub fn natural_xor_alt_2(x: &Natural, y: &Natural) -> Natural {
    let limb_zip: Box<Iterator<Item = (u32, u32)>> = if x.limb_count() >= y.limb_count() {
        Box::new(x.limbs().zip(y.limbs().chain(repeat(0))))
    } else {
        Box::new(x.limbs().chain(repeat(0)).zip(y.limbs()))
    };
    let mut or_limbs = Vec::new();
    for (x, y) in limb_zip {
        or_limbs.push(x ^ y);
    }
    Natural::from_owned_limbs_asc(or_limbs)
}