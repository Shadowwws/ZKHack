use bls_pedersen::bls::verify;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};
use ark_bls12_381::{G1Affine};
use ark_serialize::CanonicalDeserialize;
use std::io::Cursor;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (pk, ms, sigs) = puzzle_data();
    for (m, sig) in ms.iter().zip(sigs.iter()) {
        verify(pk, m, *sig);
    }

    /* Your solution here! */
    
    let sig = G1Affine::deserialize(&mut Cursor::new(hex::decode("0c307dda209b8ba2a975990c626e1e4376a8d07f93b9524cfba066cdb72e5801bef08b3428659e920b5417c85f4aeb44").unwrap())).unwrap();
    let m = hex::decode("536861646f77777773").unwrap();
    verify(pk, &m, sig);
    
}
