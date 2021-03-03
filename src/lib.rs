use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct DFA {
    num_states: usize,
    transitions: Vec<Transition>,
    initial_state: u32,
    final_states: HashSet<u32>,
}
#[derive(Debug, Clone, Copy)]
struct Transition {
    from: u32,
    label: u8,
    to: u32,
}

fn prune_unreachable(input: &DFA) -> DFA {
    let mut outflows: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut inflows: HashMap<u32, HashSet<u32>> = HashMap::new();
    for t in &input.transitions {
        outflows.entry(t.from).or_default().insert(t.to);
        inflows.entry(t.to).or_default().insert(t.from);
    }

    let mut reachable = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(input.initial_state);
    while let Some(q) = queue.pop_front() {
        if !reachable.insert(q) {
            continue;
        }
        if let Some(neighbors) = outflows.get(&q) {
            for &qq in neighbors {
                queue.push_back(qq);
            }
        }
    }

    let mut relevant = HashSet::new();
    for &q in &input.final_states {
        queue.push_back(q);
    }
    while let Some(q) = queue.pop_front() {
        if !relevant.insert(q) {
            continue;
        }
        if let Some(neighbors) = inflows.get(&q) {
            for &qq in neighbors {
                queue.push_back(qq);
            }
        }
    }

    let allowed: HashSet<u32> = reachable.intersection(&relevant).copied().collect();
    assert!(allowed.contains(&input.initial_state));

    DFA {
        num_states: allowed.len(),
        initial_state: input.initial_state,
        final_states: input.final_states.intersection(&allowed).copied().collect(),
        transitions: input
            .transitions
            .iter()
            .filter(|t| allowed.contains(&t.from) && allowed.contains(&t.to))
            .copied()
            .collect(),
    }
}

fn minimize(input: DFA) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wikipedia() {
        let input = DFA {
            initial_state: 0,
            final_states: vec![2, 3, 4].into_iter().collect(),
            num_states: 6,
            transitions: vec![
                Transition {
                    from: 0,
                    to: 1,
                    label: 0,
                },
                Transition {
                    from: 0,
                    to: 2,
                    label: 1,
                },
                Transition {
                    from: 1,
                    to: 0,
                    label: 0,
                },
                Transition {
                    from: 1,
                    to: 3,
                    label: 1,
                },
                Transition {
                    from: 2,
                    to: 4,
                    label: 0,
                },
                Transition {
                    from: 2,
                    to: 5,
                    label: 1,
                },
                Transition {
                    from: 3,
                    to: 4,
                    label: 0,
                },
                Transition {
                    from: 3,
                    to: 5,
                    label: 1,
                },
                Transition {
                    from: 4,
                    to: 4,
                    label: 0,
                },
                Transition {
                    from: 4,
                    to: 5,
                    label: 1,
                },
                Transition {
                    from: 5,
                    to: 5,
                    label: 0,
                },
                Transition {
                    from: 5,
                    to: 5,
                    label: 1,
                },
            ],
        };
        println!("{:?}", input);
        println!("{:?}", prune_unreachable(&input));
    }
}
