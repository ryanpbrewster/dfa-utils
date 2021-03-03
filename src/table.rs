use std::{collections::HashMap, hash::Hash, iter::FromIterator};

#[derive(Debug, Clone)]
pub struct Table<A, B, C> {
    tuples: Vec<(A, B, C)>,
}
impl<A, B, C> From<Vec<(A, B, C)>> for Table<A, B, C> {
    fn from(tuples: Vec<(A, B, C)>) -> Self {
        Table { tuples }
    }
}
impl<A, B, C> IntoIterator for Table<A, B, C> {
    type Item = (A, B, C);

    type IntoIter = std::vec::IntoIter<(A, B, C)>;

    fn into_iter(self) -> Self::IntoIter {
        self.tuples.into_iter()
    }
}
impl<'a, A, B, C> IntoIterator for &'a Table<A, B, C> {
    type Item = &'a (A, B, C);

    type IntoIter = std::slice::Iter<'a, (A, B, C)>;

    fn into_iter(self) -> Self::IntoIter {
        self.tuples.iter()
    }
}
impl<A, B, C> FromIterator<(A, B, C)> for Table<A, B, C> {
    fn from_iter<T: IntoIterator<Item = (A, B, C)>>(iter: T) -> Self {
        Table {
            tuples: iter.into_iter().collect(),
        }
    }
}

impl<A, B, C> Table<A, B, C>
where
    A: Eq + Hash + Copy,
    B: Eq + Hash + Copy,
    C: Eq + Hash + Copy,
{
    pub fn len(&self) -> usize {
        self.tuples.len()
    }
    pub fn by_a(&self) -> HashMap<A, Vec<(B, C)>> {
        group_by_to(&self.tuples, |&(a, _, _)| a, |&(_, b, c)| (b, c))
    }
    pub fn by_b(&self) -> HashMap<B, Vec<(A, C)>> {
        group_by_to(&self.tuples, |&(_, b, _)| b, |&(a, _, c)| (a, c))
    }
    pub fn by_c(&self) -> HashMap<C, Vec<(A, B)>> {
        group_by_to(&self.tuples, |&(_, _, c)| c, |&(a, b, _)| (a, b))
    }
}

fn group_by_to<T, K: Eq + Hash, V>(
    input: &[T],
    key_fn: impl Fn(&T) -> K,
    value_fn: impl Fn(&T) -> V,
) -> HashMap<K, Vec<V>> {
    let mut output: HashMap<K, Vec<V>> = HashMap::new();
    for t in input {
        let k = key_fn(t);
        let v = value_fn(t);
        output.entry(k).or_default().push(v);
    }
    output
}
