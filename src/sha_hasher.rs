use sha3::{
    digest::{consts, generic_array::GenericArray}, Digest, Sha3_256
};

// an array of u8 that represents 32 bytes, which represents the hash
pub type Hash = GenericArray<u8, consts::U32>;

pub fn get_hashed_value<T: AsRef<[u8]>>(value: T) -> Hash {
    let mut hasher = Sha3_256::new();

    hasher.update(value);

    hasher.finalize()
}

pub fn concat_hashes(left_tree_root: Hash, right_tree_root: Hash) -> Hash {
    let new_hash = &[left_tree_root, right_tree_root].concat()[..];

    get_hashed_value(new_hash)
}