//! Evaluate Division
//! You are given an array of variable pairs equations and an array of real numbers values,
//! where equations[i] = [Ai, Bi] and values[i] represent the equation Ai / Bi = values[i].
//! Each Ai or Bi is a string that represents a single variable.
//! You are also given some queries, where queries[j] = [Cj, Dj] represents the jth query where you must find the answer for Cj / Dj = ?.
//! Return the answers to all queries. If a single answer cannot be determined, return -1.0.
//! https://leetcode.com/problems/evaluate-division

use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet},
    rc::Rc,
};

struct Solution;

#[derive(Debug)]
struct DivisorInfo {
    divisor: GraphLink,
    fraction: f64,
}

#[derive(Debug)]
struct GraphNode {
    var: String,
    divisors: Vec<DivisorInfo>,
}

impl GraphNode {
    fn new(s: String) -> Self {
        Self {
            var: s,
            divisors: Vec::new(),
        }
    }

    fn into_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

type GraphLink = Rc<RefCell<GraphNode>>;

impl Solution {
    fn build_graph(equations: &[Vec<String>], values: &[f64]) -> HashMap<String, GraphLink> {
        let mut graph: HashMap<String, GraphLink> = HashMap::new();

        for i in 0..equations.len() {
            let dividend = equations[i][0].clone();
            let divisor = equations[i][1].clone();
            let fraction = values[i];

            // best way to minigate graph multiple borrow issue.
            if (!graph.contains_key(&dividend)) {
                graph.insert(dividend.clone(), GraphNode::new(dividend.clone()).into_rc());
            }
            if (!graph.contains_key(&divisor)) {
                graph.insert(divisor.clone(), GraphNode::new(divisor.clone()).into_rc());
            }
            let dividend_node = graph.get(&dividend).unwrap();
            let divisor_node = graph.get(&divisor).unwrap();

            dividend_node.borrow_mut().divisors.push(DivisorInfo {
                divisor: divisor_node.clone(),
                fraction,
            });

            divisor_node.borrow_mut().divisors.push(DivisorInfo {
                divisor: dividend_node.clone(),
                fraction: 1_f64 / fraction,
            });
        }

        graph
    }

    fn calculate_division(dividend_node: GraphLink, divisor_var: &str) -> Option<f64> {
        fn _calculate_division(
            dividend_node: GraphLink,
            divisor_var: &str,
            visited: &mut HashSet<String>,
            current_division: f64,
        ) -> Option<f64> {
            let bor_div_node = dividend_node.borrow();
            if visited.contains(&bor_div_node.var) {
                return None;
            }
            visited.insert(bor_div_node.var.clone());

            if let Some(division_info) = bor_div_node
                .divisors
                .iter()
                .find(|d| d.divisor.borrow().var == divisor_var)
            {
                return Some(current_division * division_info.fraction);
            } else {
                for div_info in &bor_div_node.divisors {
                    let result = _calculate_division(
                        div_info.divisor.clone(),
                        divisor_var,
                        visited,
                        current_division * div_info.fraction,
                    );

                    if result.is_some() {
                        return result;
                    }
                }
            }

            None
        }

        let mut visited_set: HashSet<String> = HashSet::new();
        let current_fraction = 1_f64;

        _calculate_division(
            dividend_node,
            divisor_var,
            &mut visited_set,
            current_fraction,
        )
    }

    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph = Self::build_graph(&equations, &values);

        let mut result: Vec<f64> = vec![-1_f64; queries.len()];

        for (idx, query) in queries.iter().enumerate() {
            let dividend = &query[0];
            let divisor = &query[1];

            let dividend_node = graph.get(dividend);
            let divisor_node = graph.get(divisor);

            let fraction: f64 = match (dividend_node, divisor_node) {
                (Some(dividend_node), Some(divisor_node)) => {
                    Self::calculate_division(dividend_node.clone(), divisor).unwrap_or(-1_f64)
                }
                _ => -1_f64,
            };

            result[idx] = fraction;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn calc_equation_1() {
        let equations = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
        ];
        let values = vec![2.0, 3.0];
        let queries = vec![
            vec!["a".to_string(), "c".to_string()],
            vec!["b".to_string(), "a".to_string()],
            vec!["a".to_string(), "e".to_string()],
            vec!["a".to_string(), "a".to_string()],
            vec!["x".to_string(), "x".to_string()],
        ];

        // we can represent equations as graph
        // (a) <- connection(fw=2,bw=0.5) -> (b) <- connection =(fw=3,bw=1/3) -> (c)

        let result = Solution::calc_equation(equations, values, queries);

        assert_eq!(result, [6.0, 0.5, -1.0, 1.0, -1.0]);
    }
}
