mod hash;
mod multi_thread_wide_hash;
mod single_thread_hash;
mod tree;

#[cfg(test)]
mod tests {
    use crate::tree_hash::{multi_thread_wide_hash, single_thread_hash, tree};

    #[test]
    fn deep_tree_single_thread_hash() {
        let tree = tree::gen_deep();

        let hash = single_thread_hash::compute_hash_with_path(&tree);

        assert!(!hash.is_empty())
    }

    #[test]
    fn wide_tree_single_thread_hash() {
        let tree = tree::gen_wide();

        let hash = single_thread_hash::compute_hash_with_path(&tree);

        assert!(!hash.is_empty())
    }

    #[test]
    fn wide_tree_multi_thread_hash() {
        let timer = std::time::Instant::now();
        println!("test: tree gen start");
        let tree = tree::gen_wide();
        println!("test: tree gen finish: {}ms", timer.elapsed().as_millis());

        let timer = std::time::Instant::now();
        println!("test: single thread hash started");
        let expected_hash = single_thread_hash::compute_hash_with_path(&tree);
        println!(
            "test: single thread hash finished {}ms",
            timer.elapsed().as_millis()
        );

        let timer = std::time::Instant::now();
        println!("test: multi thread hash started");
        let multi_thread_hash = multi_thread_wide_hash::calculate_hash(tree);
        println!(
            "test: multi thread hash finished {}ms",
            timer.elapsed().as_millis()
        );

        assert_eq!(expected_hash, multi_thread_hash);
    }

    #[test]
    fn deep_tree_multi_thread_hash() {
        let timer = std::time::Instant::now();
        println!("test: tree gen start");
        let tree = tree::gen_deep();
        println!("test: tree gen finish: {}ms", timer.elapsed().as_millis());

        let timer = std::time::Instant::now();
        println!("test: single thread hash started");
        let expected_hash = single_thread_hash::compute_hash_with_path(&tree);
        println!(
            "test: single thread hash finished {}ms",
            timer.elapsed().as_millis()
        );

        let timer = std::time::Instant::now();
        println!("test: multi thread hash started");
        let multi_thread_hash = multi_thread_wide_hash::calculate_hash(tree);
        println!(
            "test: multi thread hash finished {}ms",
            timer.elapsed().as_millis()
        );

        assert_eq!(expected_hash, multi_thread_hash);
    }
}
