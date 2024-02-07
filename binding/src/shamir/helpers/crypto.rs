use crate::{
    shamir::constants::{CALCULATED_EXPONENTS, CALCULATED_LOGARITHMS, MAX_SHARES},
    utils::get_random_buf,
};

use super::string::{pad_left, str_to_u8};

fn get_calculated_logarithm(index: u8) -> i32 {
    if index == 0 {
        CALCULATED_LOGARITHMS[0] as i32
    } else {
        CALCULATED_LOGARITHMS[index as usize - 1] as i32
    }
}

pub(crate) fn calculate_fo_fx(x: u8, coefficients: &[u8]) -> u8 {
    let log_x = get_calculated_logarithm(x);
    let mut fx = 0u8;
    for coefficient in coefficients.iter().rev() {
        if fx == 0 {
            // if f(0) then we just return the coefficient as it's just equivalent to the Y offset.
            // Using the exponent table would result in an incorrect answer
            fx = *coefficient;
        } else {
            let i = (log_x + get_calculated_logarithm(fx)) % (MAX_SHARES as i32);
            fx = CALCULATED_EXPONENTS[i as usize] ^ *coefficient;
        }
    }
    fx
}

pub(crate) fn lagrange(x: &[u8], y: &[u8]) -> u8 {
    if y.len() < x.len() {
        return 0;
    }
    let mut sum = 0u8;
    for (i, _) in x.iter().enumerate() {
        if y[i] > 0 {
            let mut product = get_calculated_logarithm(y[i]);
            for (j, _) in x.iter().enumerate() {
                if i != j {
                    product = (product + get_calculated_logarithm(x[j])
                        - get_calculated_logarithm(x[i] ^ x[j])
                        + MAX_SHARES as i32)
                        % MAX_SHARES as i32;
                }
            }
            if product < 255 {
                sum ^= CALCULATED_EXPONENTS[product as usize];
            }
        }
    }
    sum
}

pub(crate) fn get_random_binary(bits: u8) -> Result<String, String> {
    let buf_size = (bits as f32 / 8f32).ceil() as usize;
    let mut result = String::from("");
    loop {
        let binary = hex::encode(get_random_buf(buf_size)?);
        let len = binary.len() - 1;
        let mut i: usize = 0;
        while i < len || result.len() < bits.into() {
            result += &pad_left(&format!("{:b}", str_to_u8(&binary[i..i + 1], 16)?), 4);
            i += 1;
        }
        result = result[(result.len() - bits as usize)..].to_string();
        if result.find('1').is_some() {
            return Ok(result);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shamir::helpers::crypto::{calculate_fo_fx, lagrange};

    #[test]
    fn test_calculate_fo_fx() {
        assert_eq!(96, calculate_fo_fx(1, &vec![116, 107, 127]));
        assert_eq!(165, calculate_fo_fx(2, &vec![115, 172, 237]));
        assert_eq!(239, calculate_fo_fx(3, &vec![97, 116, 241]));
        assert_eq!(246, calculate_fo_fx(4, &vec![104, 183, 137]));
        assert_eq!(113, calculate_fo_fx(5, &vec![1, 159, 156]));
    }

    #[test]
    fn test_lagrange() {
        assert_eq!(
            116,
            lagrange(&vec![1, 2, 3, 4, 5], &vec![108, 49, 41, 55, 47])
        );
        assert_eq!(111, lagrange(&vec![1, 2], &vec![0, 177]));
    }
}
