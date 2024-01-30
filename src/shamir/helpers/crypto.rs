use crate::shamir::constants::{CALCULATED_EXPONENTS, CALCULATED_LOGARITHMS, MAX_SHARES};

pub(crate) fn calculate_fo_fx(x: u8, coefficients: &Vec<u8>) -> u8 {
    let log_x: u16 = CALCULATED_LOGARITHMS[x as usize - 1].into();
    let mut fx = 0;
    for coefficient in coefficients.into_iter().rev() {
        if fx == 0 {
            // if f(0) then we just return the coefficient as it's just equivalent to the Y offset.
            // Using the exponent table would result in an incorrect answer
            fx = *coefficient;
        } else {
            let i = (log_x + (CALCULATED_LOGARITHMS[fx as usize - 1] as u16)) % (MAX_SHARES as u16);
            fx = CALCULATED_EXPONENTS[i as usize] ^ *coefficient;
        }
    }
    fx
}

#[cfg(test)]
mod tests {
    use crate::shamir::helpers::crypto::calculate_fo_fx;

    #[test]
    fn test_calculate_fo_fx() {
        assert_eq!(96, calculate_fo_fx(1, &vec![116, 107, 127]));
        assert_eq!(165, calculate_fo_fx(2, &vec![115, 172, 237]));
        assert_eq!(239, calculate_fo_fx(3, &vec![97, 116, 241]));
        assert_eq!(246, calculate_fo_fx(4, &vec![104, 183, 137]));
        assert_eq!(113, calculate_fo_fx(5, &vec![1, 159, 156]));
    }
}
