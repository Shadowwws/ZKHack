#![allow(unused, unreachable_code)]
use ark_ed_on_bls12_381::{Fr, EdwardsAffine as GAffine};
use ark_ff::Field;
use strong_adaptivity::{Witness, Instance, Proof, data::puzzle_data, ProofCommitment, ProofResponse};
use strong_adaptivity::utils::b2s_hash_to_field;
use strong_adaptivity::verify;
use strong_adaptivity::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let ck = puzzle_data();

    let comm_rho = ck.message_generator;
    let comm_tau = ck.hiding_generator;

    let commitment = ProofCommitment {
        comm_rho,
        comm_tau,
    };

    let challenge = b2s_hash_to_field(&(ck, commitment));

    let s = Fr::from(1);
    let u = Fr::from(0);
    let t = Fr::from(1);

    let a_1 = Fr::from(0);
    let a_2 = challenge.inverse().unwrap();

    let r_1 = Fr::from(0);
    let r_2 = Fr::from(0);

    let comm_1 = ck.commit_with_explicit_randomness(a_1, r_1); // C_1 = 0
    let comm_2 = ck.commit_with_explicit_randomness(a_2, r_2); // C_2 = (1/challenge)*G

    let instance = Instance { comm_1, comm_2 };

    let response = ProofResponse { s, u, t };

    let proof = Proof {
        commitment,
        response,
    };

    assert!(verify(&ck, &instance, &proof));
    // Check that commitments are correct
    assert_eq!(ck.commit_with_explicit_randomness(a_1, r_1), instance.comm_1);
    assert_eq!(ck.commit_with_explicit_randomness(a_2, r_2), instance.comm_2);
    // Check that messages are unequal
    assert_ne!(a_1, a_2);
}