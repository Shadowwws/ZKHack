use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
use ark_ec::{AffineCurve, PairingEngine};
use std::ops::Neg;

use crate::hash::hash_to_curve;
use ark_ff::One;

pub fn verify(pk: G2Affine, msg: &[u8], sig: G1Affine) {
    let (_, h) = hash_to_curve(msg);
    assert!(Bls12_381::product_of_pairings(&[
        (
            sig.into(),
            G2Affine::prime_subgroup_generator().neg().into()
        ),
        (h.into(), pk.into()),
    ])
    .is_one());
}

// e(s,G2) = e(h,pk) => pk = G2*s so s = h*s
// h = sum(b_i*P_i)
// s = x*h => sum(x*b_i*P_i)