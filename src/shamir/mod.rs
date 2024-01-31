//! This code is inspired from https://github.com/unusualbob/shamirJS

mod constants;
mod helpers;

use crate::shamir::{
    constants::{FIELD_BITS, MAX_SHARES},
    helpers::{
        crypto::lagrange,
        share::{calculate_randomized_shares, extract_share_component},
        string::{binary_to_hex, hex_to_binary, pad_left, split_binary},
    },
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Shamir {}

#[wasm_bindgen]
impl Shamir {
    #[wasm_bindgen(js_name = generateShares)]
    pub fn generate_shares(
        secret: &[u8],
        total_shares: u8,
        required_shares: u8,
    ) -> Result<Vec<String>, String> {
        if total_shares < 2 {
            return Err(format!(
                "Number of shares must be an integer between 2 and 2^bits-1 ({}), inclusive.",
                MAX_SHARES
            ));
        }

        if required_shares < 2 {
            return Err(format!(
                "Threshold number of shares must be an integer between 2 and 2^bits-1 ({}), inclusive.",
                MAX_SHARES
            ));
        }

        if required_shares > total_shares {
            return Err(format!(
                "Threshold number of shares was {} but must be less than or equal to the {} shares specified as the total to generate.", required_shares, 
                total_shares
            ));
        }

        let secrets = split_binary(
            &format!("1{}", hex_to_binary(&hex::encode(secret))?),
            Some(128),
        )?;
        let mut x = vec!["".to_string(); total_shares as usize];
        let mut y = vec!["".to_string(); total_shares as usize];

        // For each character in the secret integer array, generate `total_shares` sub-shares,
        // concatenating each sub-share `i` to create a total of `total_shares` outputs
        for secret in &secrets {
            let sub_shares = calculate_randomized_shares(*secret, total_shares, required_shares)?;
            for i in 0..(total_shares as usize) {
                if x[i].is_empty() {
                    x[i] = format!("{:x}", sub_shares[i].x);
                }
                y[i] = format!(
                    "{}{}",
                    pad_left(&format!("{:b}", sub_shares[i].y), FIELD_BITS as usize),
                    y[i]
                );
            }
        }
        // Creates the final share strings which contain the share's id and the data allocated to the share
        for i in 0..(total_shares as usize) {
            let share_id = pad_left(&x[i], 2);
            x[i] = format!("{}{}", share_id, binary_to_hex(&y[i])?);
        }
        Ok(x)
    }

    #[wasm_bindgen(js_name = recoverSecret)]
    pub fn recover_secret(shares: Vec<String>) -> Result<Vec<u8>, String> {
        // Here we split each share's hexadecimal data into an array of integers. We then copy each item at position `j` for each share into
        // its own array. This ultimately 'rotates' the arrays so that the output changes from something like this:
        //
        //   Share 1 [1, 2, 3, 4, 5]
        //   Share 2 [6, 7, 8, 9, 10]
        //   Share 3 [11, 12, 13, 14, 15]
        //
        // Into something like this:
        //
        // [
        //   [ 1, 6, 11 ],
        //   [ 2, 7, 12 ],
        //   [ 3, 8, 13 ],
        //   [ 4, 9, 14 ],
        //   [ 5, 10, 15 ]
        // ]
        let mut x: Vec<u8> = vec![];
        let mut split_shares: Vec<Vec<u8>> = vec![];
        for share in &shares {
            let share_component = extract_share_component(share)?;
            if !x.contains(&share_component.id) {
                x.push(share_component.id);
                split_shares.push(split_binary(&hex_to_binary(&share_component.data)?, None)?);
            }
        }

        let mut y: Vec<Vec<u8>> = vec![];
        for i in 0..split_shares[0].len() {
            let mut data: Vec<u8> = vec![];
            for j in &split_shares {
                data.push(j[i]);
            }
            y.push(data);
        }

        let mut secret = String::from("");
        // We then extract the secret from each array by calculating the lagrange point using each array as a set of coordinates.
        // These secrets are concatenated together to make the binary string version of the original secret.
        for i in &y {
            secret = pad_left(&format!("{:b}", lagrange(&x, i)), FIELD_BITS as usize) + &secret;
        }

        // Search the string for the first '1' and disregard all 0s before that as these were added via a left-pad.
        // We then convert the remaining binary string back into hexadecimal to get the original secret data
        if let Some(i) = secret.find('1') {
            let secret = binary_to_hex(&secret[i + 1..])?;
            match hex::decode(secret) {
                Ok(v) => Ok(v),
                Err(error) => Err(format!(
                    "Can't convert secret to bytes, the error is: {:}",
                    error
                )),
            }
        } else {
            Err("Can't recover secret.".to_string())
        }
    }
}
