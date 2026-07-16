use sha2::{Digest, Sha256};
use std::collections::HashMap;

type Hash = Vec<u8>;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct ProofItem {
    hash: Hash,
    direction: Direction,
}

#[derive(Debug)]
struct MerkleProof {
    items: Vec<ProofItem>,
}

struct MerkleTree {
    levels: Vec<Vec<Hash>>,
    leaf_map: HashMap<String, usize>,
}

impl MerkleTree {
    fn new(data: Vec<&str>) -> Self {
        let mut leaf_map = HashMap::new();

        // leaf hashes
        let mut current_level = Vec::new();

        for (i, item) in data.iter().enumerate() {
            let hash = Self::hash(item.as_bytes());

            leaf_map.insert(item.to_string(), i);

            current_level.push(hash);
        }

        let mut levels = Vec::new();

        levels.push(current_level.clone());

        // build tree
        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            let mut i = 0;

            while i < current_level.len() {
                let left = &current_level[i];

                let right = if i + 1 < current_level.len() {
                    &current_level[i + 1]
                } else {
                    left
                };

                let parent =
                    Self::hash_pair(left, right);

                next_level.push(parent);

                i += 2;
            }

            levels.push(next_level.clone());

            current_level = next_level;
        }

        Self {
            levels,
            leaf_map,
        }
    }

    fn hash(data: &[u8]) -> Hash {
        Sha256::digest(data).to_vec()
    }

    fn hash_pair(left: &[u8], right: &[u8]) -> Hash {
        let mut combined = Vec::new();

        combined.extend_from_slice(left);
        combined.extend_from_slice(right);

        Self::hash(&combined)
    }

    fn root(&self) -> &Hash {
        &self.levels.last().unwrap()[0]
    }

    fn generate_proof(
        &self,
        data: &str,
    ) -> Option<MerkleProof> {
        let mut index =
            *self.leaf_map.get(data)?;

        let mut proof = Vec::new();

        for level in &self.levels {
            if level.len() == 1 {
                break;
            }

            let sibling_index =
                if index % 2 == 0 {
                    index + 1
                } else {
                    index - 1
                };

            let sibling_hash =
                if sibling_index < level.len() {
                    level[sibling_index].clone()
                } else {
                    level[index].clone()
                };

            let direction =
                if index % 2 == 0 {
                    Direction::Right
                } else {
                    Direction::Left
                };

            proof.push(ProofItem {
                hash: sibling_hash,
                direction,
            });

            index /= 2;
        }

        Some(MerkleProof { items: proof })
    }

    fn verify_proof(
        data: &str,
        proof: &MerkleProof,
        root: &[u8],
    ) -> bool {
        let mut current_hash =
            Self::hash(data.as_bytes());

        for item in &proof.items {
            current_hash = match item.direction {
                Direction::Left => {
                    Self::hash_pair(
                        &item.hash,
                        &current_hash,
                    )
                }

                Direction::Right => {
                    Self::hash_pair(
                        &current_hash,
                        &item.hash,
                    )
                }
            };
        }

        current_hash == root
    }
}

fn to_hex(hash: &[u8]) -> String {
    hash
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

fn main() {
    let data = vec!["a", "b", "c", "d"];

    let tree = MerkleTree::new(data);

    println!(
        "Root Hash:\n{}\n",
        to_hex(tree.root())
    );

    let proof =
        tree.generate_proof("c").unwrap();

    println!("Proof for c:");

    for item in &proof.items {
        println!(
            "{:?} -> {}",
            item.direction,
            to_hex(&item.hash)
        );
    }

    let valid = MerkleTree::verify_proof(
        "c",
        &proof,
        tree.root(),
    );

    println!("\nProof Valid: {}", valid);
}