mod merkle;
pub mod sha_hasher;

use merkle::MerkleTree;

fn main() {
    let vec = vec!["t1", "t2", "t3", "t4"];

    let merkle = MerkleTree::new(vec);

    println!("{:#?}", merkle);
}
