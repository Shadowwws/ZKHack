#![allow(unused, unreachable_code, dead_code)]

use ark_bls12_381::{Fr, G1Affine};
use ark_ff::*;
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, GeneralEvaluationDomain, Polynomial,
    UVPolynomial,
};
use ark_serialize::CanonicalDeserialize;
use hidden_in_plain_sight::{generate::kzg_commit, PUZZLE_DESCRIPTION};
use prompt::{puzzle, welcome};

fn read_cha_from_file() -> (Vec<G1Affine>, Vec<Vec<Fr>>, Fr, Fr, G1Affine, Fr, Fr) {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open("challenge_data").unwrap();
    let mut bytes: Vec<u8> = vec![];
    file.read_to_end(&mut bytes).unwrap();

    let setup_bytes: Vec<u8> = bytes[0..98312].to_vec();
    let accts_bytes: Vec<u8> = bytes[98312..1130320].to_vec();
    let cha_1_bytes: Vec<u8> = bytes[1130320..1130352].to_vec();
    let cha_2_bytes: Vec<u8> = bytes[1130352..1130384].to_vec();
    let commt_bytes: Vec<u8> = bytes[1130384..1130480].to_vec();
    let opn_1_bytes: Vec<u8> = bytes[1130480..1130512].to_vec();
    let opn_2_bytes: Vec<u8> = bytes[1130512..1130544].to_vec();

    let setup = Vec::<G1Affine>::deserialize_unchecked(&setup_bytes[..]).unwrap();
    let accts = Vec::<Vec<Fr>>::deserialize_unchecked(&accts_bytes[..]).unwrap();
    let cha_1 = Fr::deserialize_unchecked(&cha_1_bytes[..]).unwrap();
    let cha_2 = Fr::deserialize_unchecked(&cha_2_bytes[..]).unwrap();
    let commt = G1Affine::deserialize_unchecked(&commt_bytes[..]).unwrap();
    let opn_1 = Fr::deserialize_unchecked(&opn_1_bytes[..]).unwrap();
    let opn_2 = Fr::deserialize_unchecked(&opn_2_bytes[..]).unwrap();

    (setup, accts, cha_1, cha_2, commt, opn_1, opn_2)
}

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);

    let (setup, accts, cha_1, cha_2, commt, opn_1, opn_2) = read_cha_from_file();

    let domain: GeneralEvaluationDomain<Fr> =
        GeneralEvaluationDomain::new(1000 + 2).unwrap();

    let mut solution_commitment = G1Affine::zero();

    let mut k: u64 = 1;

    for acct in accts.to_vec() {
        println!("k = {}", k);
        k = k+1;

        let target_acct_poly = DensePolynomial::from_coefficients_vec(domain.ifft(&acct));
    
        let mut poly1 = target_acct_poly.evaluate(&cha_1);
        let mut poly2 = target_acct_poly.evaluate(&cha_2);

        const N: u64 = 1024u64;

        poly1 = (opn_1 - poly1)/domain.vanishing_polynomial().evaluate(&cha_1); // b0 + b_1 * cha_1
        poly2 = (opn_2 - poly2)/domain.vanishing_polynomial().evaluate(&cha_2); // b0 + b_1 * cha_2

        let b_1 = (poly2-poly1)/(cha_2-cha_1);
        let b_0 = poly1 - b_1*cha_1;

        let blinding_poly =
            DensePolynomial::from_coefficients_vec(vec![b_0, b_1]);
    
        let blinded_acct_poly = target_acct_poly + blinding_poly.mul_by_vanishing_poly(domain);

        solution_commitment = kzg_commit(&blinded_acct_poly, &setup);

        if solution_commitment == commt{
            println!("Found, account num : {}",k);
            break;
        }
    }

    assert_eq!(solution_commitment, commt);
    
}
