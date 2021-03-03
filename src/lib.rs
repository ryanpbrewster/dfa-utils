use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

use partition::Partition;
use table::Table;

mod partition;
mod table;

#[derive(Debug)]
pub struct DFA<S, E> {
    initial_state: S,
    final_states: HashSet<S>,
    transitions: Table<S, E, S>,
}

impl<S, E> DFA<S, E>
where
    S: Eq + Hash + Copy + Debug,
    E: Eq + Hash + Copy + Debug,
{
    pub fn prune_unreachable(self) -> DFA<S, E> {
        let outflows = self.transitions.by_a();
        let inflows = self.transitions.by_c();

        let mut reachable = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(self.initial_state);
        while let Some(src) = queue.pop_front() {
            if !reachable.insert(src) {
                continue;
            }
            if let Some(neighbors) = outflows.get(&src) {
                for &(_, dst) in neighbors {
                    queue.push_back(dst);
                }
            }
        }

        let mut relevant = HashSet::new();
        for &q in &self.final_states {
            queue.push_back(q);
        }
        while let Some(dst) = queue.pop_front() {
            if !relevant.insert(dst) {
                continue;
            }
            if let Some(neighbors) = inflows.get(&dst) {
                for &(src, _) in neighbors {
                    queue.push_back(src);
                }
            }
        }

        let allowed: HashSet<S> = reachable.intersection(&relevant).copied().collect();
        assert!(allowed.contains(&self.initial_state));

        DFA {
            initial_state: self.initial_state,
            final_states: self.final_states.intersection(&allowed).copied().collect(),
            transitions: self
                .transitions
                .into_iter()
                .filter(|(src, _, dst)| allowed.contains(src) && allowed.contains(dst))
                .collect(),
        }
    }

    pub fn minimize(&self) -> DFA<S, E> {
        let by_src = self.transitions.by_a();
        let by_dst = self.transitions.by_c();
        let by_label = self.transitions.by_b();

        let mut blocks = {
            let states: HashSet<S> = by_src.keys().chain(by_dst.keys()).copied().collect();
            Partition::new(states.into_iter().collect())
        };

        // Start an initial partition by separating out the accepting states.
        for &q in &self.final_states {
            blocks.mark(q);
        }
        blocks.split();

        let mut cords = Partition::new(self.transitions.clone().into_iter().collect());
        // Start the initial partition by separating out every edge label.
        for (label, es) in by_label {
            for (src, dst) in es {
                cords.mark((src, label, dst));
            }
            cords.split();
        }
        println!("initial cord partition has {} sets", cords.len());

        // Repeatedly refine partitions.
        let mut b = 1;
        let mut c = 0;
        while c < cords.len() {
            for &(src, _, _) in cords.owned(c) {
                blocks.mark(src);
            }
            blocks.split();
            c += 1;
            while b < blocks.len() {
                for &dst in blocks.owned(b) {
                    if let Some(edges) = by_dst.get(&dst) {
                        for &(src, label) in edges {
                            cords.mark((src, label, dst));
                        }
                    }
                }
                cords.split();
                b += 1;
            }
        }

        let mut canonical_tuples = Vec::new();
        for i in 0..blocks.len() {
            let src = blocks.canonical(i);
            for &(label, dst) in &by_src[&src] {
                canonical_tuples.push((src, label, blocks.canonical(blocks.owner(dst))));
            }
        }
        DFA {
            initial_state: blocks.canonical(blocks.owner(self.initial_state)),
            final_states: self
                .final_states
                .iter()
                .map(|&q| blocks.canonical(blocks.owner(q)))
                .collect(),
            transitions: Table::from(canonical_tuples),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prune_wikipedia() {
        let transitions: Vec<(u32, u8, u32)> = vec![
            (0, 0, 1),
            (0, 1, 2),
            (1, 0, 0),
            (1, 1, 3),
            (2, 0, 4),
            (2, 1, 5),
            (3, 0, 4),
            (3, 1, 5),
            (4, 0, 4),
            (4, 1, 5),
            (5, 0, 5),
            (5, 1, 5),
        ];
        let input: DFA<u32, u8> = DFA {
            initial_state: 0,
            final_states: vec![2, 3, 4].into_iter().collect(),
            transitions: Table::from(transitions),
        };
        assert_eq!(input.transitions.len(), 12);
        assert_eq!(input.transitions.by_a().len(), 6);
        let pruned = input.prune_unreachable();
        // We pruned out 5 transitions and 1 state.
        assert_eq!(pruned.transitions.len(), 7);
        assert_eq!(pruned.transitions.by_a().len(), 5);
    }

    #[test]
    fn minimize_wikipedia() {
        let transitions: Vec<(u32, u8, u32)> = vec![
            (0, 0, 1),
            (0, 1, 2),
            (1, 0, 0),
            (1, 1, 3),
            (2, 0, 4),
            (2, 1, 5),
            (3, 0, 4),
            (3, 1, 5),
            (4, 0, 4),
            (4, 1, 5),
            (5, 0, 5),
            (5, 1, 5),
        ];
        let input: DFA<u32, u8> = DFA {
            initial_state: 0,
            final_states: vec![2, 3, 4].into_iter().collect(),
            transitions: Table::from(transitions),
        };
        assert_eq!(input.transitions.len(), 12);
        assert_eq!(input.transitions.by_a().len(), 6);
        let pruned = input.prune_unreachable();
        let minified = pruned.minimize();
        assert_eq!(minified.transitions.len(), 3);
        assert_eq!(minified.transitions.by_a().len(), 2);
    }

    #[test]
    fn minimize_example1() {
        let transitions: Vec<(u32, u8, u32)> = vec![
            (0, 0, 1),
            (1, 0, 2),
            (1, 1, 2),
            (2, 0, 2),
            (2, 1, 2),
            (0, 1, 3),
            (3, 0, 4),
            (3, 1, 4),
            (4, 0, 4),
            (4, 1, 4),
        ];
        let input: DFA<u32, u8> = DFA {
            initial_state: 0,
            final_states: vec![2, 4].into_iter().collect(),
            transitions: Table::from(transitions),
        };
        assert_eq!(input.transitions.len(), 10);
        assert_eq!(input.transitions.by_a().len(), 5);
        let pruned = input.prune_unreachable();
        println!("{:?}", pruned);
        let minified = pruned.minimize();
        println!("{:?}", minified);
        assert_eq!(minified.transitions.len(), 6);
        assert_eq!(minified.transitions.by_a().len(), 3);
        assert_eq!(minified.final_states.len(), 1);
    }
}
