use std::{collections::HashMap, process, sync::Arc};

use rayon::{ThreadPool, ThreadPoolBuilder};

use crate::tree_hash::tree::{self, traverse_tree, RawNode};

fn t_pool() -> ThreadPool {
    ThreadPoolBuilder::new()
        .num_threads(20)
        .build()
        .expect("Failed to build thread pool")
}

struct NodeWithHash {
    node: Arc<RawNode>,
    hash: Option<String>,
}

struct Layer {
    nodes: HashMap<String, NodeWithHash>,
}

fn calculate_layer_hash(pool: &ThreadPool, current_layer: &mut Layer, next_layer: Option<&Layer>) {
    pool.scope(|s| {
        current_layer
            .nodes
            .iter_mut()
            .for_each(|(node_path, node)| {
                s.spawn(move |b| match next_layer {
                    Some(next_layer) => {
                        let children_hashes: Vec<String> = node
                            .node
                            .children
                            .iter()
                            .map(|c| {
                                let child_path = tree::calculate_path(node_path, &c.name);

                                let child = next_layer.nodes.get(&child_path).unwrap();
                                child.hash.clone().unwrap()
                            })
                            .collect();
                        let hash =
                            super::hash::calculate_hash(&node.node, node_path, &children_hashes);
                        node.hash = Some(hash);
                    }
                    None => {
                        let hash = super::hash::calculate_hash(&node.node, node_path, &[]);
                        node.hash = Some(hash);
                    }
                });
            });
    });
}

pub fn calculate_hash(tree: Arc<RawNode>) -> String {
    let pool = t_pool();
    let mut layers: Vec<Layer> = vec![];

    traverse_tree(tree, &mut |node, node_level, path| {
        if layers.get(node_level).is_none() {
            layers.insert(
                node_level,
                Layer {
                    nodes: HashMap::new(),
                },
            )
        }

        unsafe {
            layers
                .get_unchecked_mut(node_level)
                .nodes
                .insert(path.into(), NodeWithHash { node, hash: None })
        };
    });

    for i in (0..layers.len()).rev() {
        if i == layers.len() - 1 {
            let current_layer = layers.get_mut(i).unwrap();
            calculate_layer_hash(&pool, current_layer, None);
            continue;
        }

        let (left_mut, right_mut) = layers.split_at_mut(i + 1);

        let right_immut: &[Layer] = &right_mut;

        let next_layer = right_immut.first().unwrap();

        let (current_layer, _) = left_mut.split_last_mut().unwrap();

        calculate_layer_hash(&pool, current_layer, Some(&next_layer));
    }

    layers[0]
        .nodes
        .values()
        .next()
        .unwrap()
        .hash
        .clone()
        .unwrap()
}
