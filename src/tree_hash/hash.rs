use sha2::{Digest, Sha256};

use crate::tree_hash::tree::RawNode;

pub fn calculate_hash(node: &RawNode, node_path: &str, child_hashes: &[String]) -> String {
    let mut hasher = Sha256::new();

    hasher.update(node_path.as_bytes());
    hasher.update(node.data.as_bytes());

    for ch in child_hashes {
        hasher.update(ch);
    }
    let bytes = hasher.finalize().to_vec();
    hex::encode(bytes)
}
