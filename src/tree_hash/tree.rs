use rand::{
    distr::{Alphanumeric, Uniform},
    Rng,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct RawNode {
    pub name: String,
    pub data: Arc<String>,
    pub children: Vec<Arc<RawNode>>,
}

fn generate_large_string(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

fn generate_name(level: usize) -> String {
    let id = rand::thread_rng().gen_range(0..1000000);

    format!("child_{level}_{id}")
}

fn generate_wide_tree(
    current_level: usize,
    levels: usize,
    width: usize,
    data_size: usize,
) -> Arc<RawNode> {
    let data = Arc::new(generate_large_string(data_size));
    let children = if current_level < levels {
        (0..width)
            .map(|_| generate_wide_tree(current_level + 1, levels, width, data_size))
            .collect()
    } else {
        Vec::new()
    };
    Arc::new(RawNode {
        name: generate_name(current_level),
        data,
        children,
    })
}

pub fn calculate_path(to_node: &str, node_name: &str) -> String {
    format!("{to_node}/{node_name}")
}

fn generate_deep_tree(current_depth: usize, depth: usize, data_size: usize) -> Arc<RawNode> {
    let data = Arc::new(generate_large_string(data_size));
    let children = if current_depth < depth {
        vec![generate_deep_tree(current_depth + 1, depth, data_size)]
    } else {
        Vec::new()
    };
    Arc::new(RawNode {
        name: generate_name(current_depth),
        data,
        children,
    })
}

pub fn gen_deep() -> Arc<RawNode> {
    generate_deep_tree(0, 100, 100000)
}

pub fn gen_wide() -> Arc<RawNode> {
    generate_wide_tree(0, 3, 4, 102000)
}

pub fn traverse_tree<T>(node: Arc<RawNode>, f: &mut T)
where
    T: FnMut(Arc<RawNode>, usize, &str),
{
    traverse_tree_inner(node, f, 0, "");
}

fn traverse_tree_inner<T>(node: Arc<RawNode>, f: &mut T, current_depth: usize, current_path: &str)
where
    T: FnMut(Arc<RawNode>, usize, &str),
{
    let current_path = calculate_path(current_path, &node.name);

    f(node.clone(), current_depth, &current_path);
    for child in &node.children {
        traverse_tree_inner(child.clone(), f, current_depth + 1, &current_path);
    }
}
