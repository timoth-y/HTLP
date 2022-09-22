use num_bigint_dig::ToBigUint;
use htlp::lhp;

fn main() {
    // Example: generate two puzzles and homomorphicly add them

    let pp = lhp::setup::setup(20, 18.to_biguint().unwrap()); // generate parameters that uses 20 bit representation, time parameter is 10

    //First Puzzle

    let z1 = lhp::generate::gen(&pp, 434.to_biguint().unwrap()); // secret for the first puzzle is 434
    let mut s = lhp::solve::solve(&pp, &z1);
    println!("First puzzle solved. Secret: {}",s);

    //Second Puzzle

    let z2 = lhp::generate::gen(&pp, 10.to_biguint().unwrap()); // secret for the first puzzle 10
    s=lhp::solve::solve(&pp,&z2);
    println!("Second puzzle solved. Secret: {}",s);

    //Third Puzzle (sum of the previous ones)

    let z3 = lhp::lin_eval::add(&pp, &z1, &z2); // secret for the third puzzle 444
    s=lhp::solve::solve(&pp,&z3);
    println!("Third (homomorphic evaluation of the previous puzzles) puzzle solved. Secret: {}",s);
}
