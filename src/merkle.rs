
use crate::sha_hasher::{ self, Hash };

type Node = Option<Box<MerkleTree>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleTree {
    root: Hash,
    left_node: Node,
    right_node: Node,
}

impl MerkleTree {
    pub fn new(vec: Vec<&str>) -> Self {
        let mut hash_vec: Vec<MerkleTree> = Self::to_hashed_leafs(vec);

        for _i in 0..hash_vec.len() / 2{
            let mut aux_merkle_vec: Vec<MerkleTree> = Vec::new();
            
            for j in (0..hash_vec.len()).step_by(2) {
                let left_sub_tree = hash_vec.get(j).unwrap();
                let right_sub_tree = hash_vec.get(j+1).unwrap();

                aux_merkle_vec.push(Self::create_branch(left_sub_tree.clone(), right_sub_tree.clone()))
            }

            hash_vec = aux_merkle_vec;
        }
        
        hash_vec.remove(0)
    }

    fn create_node(hash: Hash) -> Self {
        Self {
            root: hash,
            left_node: None,
            right_node: None,
        }
    }

    fn create_branch(left_tree: MerkleTree, right_tree: MerkleTree) -> Self {
        let hash = sha_hasher::concat_hashes(left_tree.root, right_tree.root);

        Self {
            root: hash,
            left_node: Some(Box::new(left_tree)),
            right_node: Some(Box::new(right_tree))
        }
    }

    // takes a vector a string and returns a vector a leafs
    fn to_hashed_leafs(vec: Vec<&str>) -> Vec<Self> {
        let mut leafs_vec: Vec<MerkleTree> = Vec::new();

        vec.into_iter().for_each(|value| {
            let hash = sha_hasher::get_hashed_value(value.as_bytes());

            leafs_vec.push(Self::create_node(hash));
        });

        leafs_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successfully_creates_merkle_tree() {
        let str_vec = vec!["t1", "t2", "t3", "t4"];
        let merkle_tree = MerkleTree::new(str_vec);

        let t1_h = sha_hasher::get_hashed_value("t1");
        let t2_h = sha_hasher::get_hashed_value("t2");
        let t3_h = sha_hasher::get_hashed_value("t3");
        let t4_h = sha_hasher::get_hashed_value("t4");

        let t12_h = sha_hasher::concat_hashes(t1_h, t2_h);
        let t34_h = sha_hasher::concat_hashes(t3_h, t4_h);

        let t1234_h = sha_hasher::concat_hashes(t12_h, t34_h);

        let check_merkle = MerkleTree {
            root: t1234_h,
            left_node: Some(Box::new(MerkleTree {
                root: t12_h,
                left_node: Some(Box::new(MerkleTree {
                    root:t1_h,
                    left_node: None,
                    right_node: None
                })),
                right_node: Some(Box::new(MerkleTree{
                    root:t2_h,
                    left_node: None,
                    right_node: None
                }))
            })),
            right_node: Some(Box::new(MerkleTree{
                root: t34_h,
                left_node: Some(Box::new(MerkleTree {
                    root:t3_h,
                    left_node: None,
                    right_node: None
                })),
                right_node: Some(Box::new(MerkleTree{
                    root:t4_h,
                    left_node: None,
                    right_node: None
                }))
            }))
        };

        assert_eq!(check_merkle, merkle_tree);
    }
}