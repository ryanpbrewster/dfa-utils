use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

use table::Table;

mod partition;
mod table;

#[derive(Debug)]
struct DFA<S, E> {
    initial_state: S,
    final_states: HashSet<S>,
    transitions: Table<S, E, S>,
}

impl<S, E> DFA<S, E>
where
    S: Eq + Hash + Copy,
    E: Eq + Hash + Copy,
{
    fn prune_unreachable(self) -> DFA<S, E> {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wikipedia() {
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
}
