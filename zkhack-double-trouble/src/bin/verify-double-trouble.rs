#![allow(unused, unreachable_code)]
use ark_ed_on_bls12_381::Fr;
use ark_ff::Field;
use double_trouble::data::puzzle_data;
use double_trouble::inner_product_argument::utils::challenge;
use double_trouble::verify;
use double_trouble::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (ck, [instance_and_proof_1, instance_and_proof_2]) = puzzle_data();
    let (instance1, proof1) = instance_and_proof_1;
    let (instance2, proof2) = instance_and_proof_2;
    assert!(verify(&ck, &instance1, &proof1));
    assert!(verify(&ck, &instance2, &proof2));

    // s2 - s1 = r*(2*y2 - y1) => r = (s2-s1)/(2*y2-y1)
    let y1 = challenge(&ck, &instance1, &proof1.commitment);
    let y2 = challenge(&ck, &instance2, &proof2.commitment);

    let div = (y2.double()-y1).inverse().unwrap();

    let mut r = Vec::with_capacity(proof1.response.s.capacity());
    for (s1, s2) in proof1.response.s.iter().zip(proof2.response.s.iter()) {
        let s_diff = *s2 - *s1;
        let r_i = s_diff * div;
        r.push(r_i);
    }

    let a: Vec<Fr> = proof1.response.s.iter().zip(r).map(|(s, r)| *s - y1 * r).collect();;

    // u2 - u1 = p*(2*y2 - y1) => p = (u2-u1)/(2*y2-y1)
    let u3 = proof2.response.u - proof1.response.u;

    let p = u3 * (y2.double()-y1).inverse().unwrap();

    let comm_a_rand = proof1.response.u - p*y1;

    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance1.comm_a
    );
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance2.comm_a
    );
}