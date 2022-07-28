#![no_std]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

#[cfg(test)]
#[macro_use]
extern crate std;
extern crate self as bitmatch;

pub use macros::bitmatch;

#[inline(always)]
pub fn from_mask<T: Into<u128> + From<u128>>(val: T, mut mask: u128) -> T {
    let mut val: u128 = val.into();
    let mut new_val = 0;
    let mut add_bit = 0;
    while mask != 0 {
        let bit = mask & 0b1;
        if bit != 0 {
            new_val |= ((val & bit != 0) as u128) << add_bit;
            add_bit += 1;
        }
        val >>= 1;
        mask >>= 1;
    }
    new_val.into()
}

#[cfg(test)]
mod tests {
    use macros::bitmatch;

    #[test]
    fn test() {
        // 0000 zzzz zzzz <- nothing,
        // 0001 xxxx zzzz <- x
        // 0010 0x0x 00xx <- x
        let tests_input = vec![
            (0b0000_1100_0010, None),
            (0b0000_0101_0000, None),
            (0b0000_0000_0000, None),
            (0b0001_1111_0000, Some(0b1111)),
            (0b0001_1101_0101, Some(0b1101)),
        ];
        let mut test_case = 0;
        for (i, r) in tests_input {
            let val = #[bitmatch]
            match i {
                "0000_zzzz_zzzz" => None,
                "0001_xxxx_zzzz" => Some(x),
                _ => Some(u128::MAX),
            };
            assert!(
                    r == val,
                    "expected '{r:?}', got '{val:?}'\noriginal input: '{i:012b}'\nat test_case {test_case}"
                );
            test_case += 1;
        }
    }
}
