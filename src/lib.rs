use std::collections::HashMap;

struct DFA {
    // states[0] is the initial state
    states: Vec<State>,
}
struct State {
    accepting: bool,
    transitions: HashMap<u8, usize>,
}

#[cfg(test)]
mod tests {
    use super::{State, DFA};
    #[test]
    fn minimize_1() {
        let a = State {
            accepting: false,
            transitions: vec![(0, 1), (1, 2)].into_iter().collect(),
        };
        let b = State {
            accepting: false,
            transitions: vec![(0, 0), (1, 3)].into_iter().collect(),
        };
        let c = State {
            accepting: true,
            transitions: vec![(0, 4), (1, 5)].into_iter().collect(),
        };
        let d = State {
            accepting: true,
            transitions: vec![(0, 4), (1, 5)].into_iter().collect(),
        };
        let e = State {
            accepting: true,
            transitions: vec![(0, 4), (1, 5)].into_iter().collect(),
        };
        let f = State {
            accepting: false,
            transitions: vec![(0, 5), (1, 5)].into_iter().collect(),
        };
        let input = DFA {
            states: vec![a, b, c, d, e, f],
        };
        let output = minimize(input);
        assert_eq!(output.states.len(), 3);
    }
}
