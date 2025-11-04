use super::hash;
use sha2::{Digest, Sha256};
use std::sync::Arc;

use crate::tree_hash::tree::RawNode;

fn _compute_hash_with_path(node: &RawNode, path_prefix: &str) -> String {
    let full_path = format!("{}/{}", path_prefix, node.name);

    let mut child_hashes: Vec<String> = vec![];
    for child in &node.children {
        let child_hash = _compute_hash_with_path(child, &full_path);
        child_hashes.push(child_hash);
    }

    hash::calculate_hash(node, &full_path, &child_hashes)
}

pub fn compute_hash_with_path(node: &RawNode) -> String {
    _compute_hash_with_path(node, "")
}
