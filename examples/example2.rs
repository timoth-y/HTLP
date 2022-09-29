use num_bigint_dig::{BigUint, ToBigUint};
use htlp::lhp;

fn main() {
    // Example: generate puzzles for a witness of ECDSA key size

    let pp = lhp::setup::setup(256, 12.to_biguint().unwrap()); // generate parameters that uses 20 bit representation, time parameter is 10

    let sk = hex::decode("70a5a045d6cb3cf556f515fae057fb58e9d3d478b9afb9eb491d456d8ef91c55").unwrap();
    let sk = BigUint::from_bytes_be(&sk);
    let z1 = lhp::generate::gen(&pp, sk.clone());

    let mut s = lhp::solve::solve(&pp, &z1);

    println!("recovered secret: {}", hex::encode(s.to_bytes_be()));
    assert_eq!(sk, s);
}
