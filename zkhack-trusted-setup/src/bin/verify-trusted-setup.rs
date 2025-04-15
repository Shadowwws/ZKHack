use ark_bls12_381::Fr;
use ark_ec::AffineCurve;
use prompt::{puzzle, welcome};
use std::str::FromStr;
use trusted_setup::data::puzzle_data;
use trusted_setup::PUZZLE_DESCRIPTION;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (_ts1, _ts2) = puzzle_data();

    // Get the G1 and G2 used because smh they're different
    println!("{}", _ts1[0]);
    println!("{}", _ts2[0]);

    // Get sG1 and sG2
    println!("{}", _ts1[1]);
    println!("{}", _ts2[1]);

    let modp = Fr::from_str("38452154918091875653578148163112927").unwrap();
    let mut s = Fr::from_str("5592216610550884993006174526481245").unwrap();

    while _ts1[0].mul(s) != _ts1[1] {
        s = s+modp;
    }

    assert_eq!(_ts1[0].mul(s), _ts1[1]);
    assert_eq!(_ts2[0].mul(s), _ts2[1]);

    println!("ok !");
}
