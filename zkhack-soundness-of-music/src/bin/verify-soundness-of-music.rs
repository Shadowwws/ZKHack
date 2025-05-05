#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use ark_ff::One;
use ark_ec::{AffineCurve, ProjectiveCurve};
use prompt::{puzzle, welcome};
use soundness_of_music::data::puzzle_data;
use soundness_of_music::prover;
use soundness_of_music::circuit;
use soundness_of_music::setup;
use soundness_of_music::verifier;
use soundness_of_music::PUZZLE_DESCRIPTION;

type Fr = <ark_bls12_381::Bls12_381 as ark_ec::PairingEngine>::Fr;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    
    let (circuit , setup) = puzzle_data();

    let two = Fr::one() + Fr::one();
    let four = two + two;
    let mut public_inputs = [Fr::one(), four];
    let private_inputs = [two];
    let mut proof = prover::prove(&public_inputs, &private_inputs, &circuit, &setup);

    proof.pi_input = proof.pi_input + setup.inputs[1].mul(Fr::from(3)).into_affine();

    proof.pi_input_prime = proof.pi_input_prime + setup.inputs[1].mul(Fr::from(3)).into_affine();

    public_inputs = [Fr::one(), Fr::one()];

    assert!(verifier::verify(&public_inputs, &setup, &proof));
}
