use std::fmt::Debug;
use std::hash::Hash;
use std::{collections::HashMap, ops::Range};

type SetId = usize;
pub struct Partition<T> {
    elements: Vec<T>,
    locations: HashMap<T, usize>,
    owners: HashMap<T, SetId>,
    spans: Vec<Range<usize>>,
    marked: Vec<usize>,
    // List of sets
    touched: Vec<SetId>,
}

impl<T> Partition<T>
where
    T: Eq + Hash + Copy + Debug,
{
    pub fn new(elements: Vec<T>) -> Partition<T> {
        let locations = elements
            .iter()
            .enumerate()
            .map(|(idx, &e)| (e, idx))
            .collect();
        let owners = elements.iter().map(|&e| (e, 0)).collect();
        let spans = vec![0..elements.len()];
        let marked = vec![0];
        let touched = vec![];
        Partition {
            elements,
            locations,
            owners,
            spans,
            marked,
            touched,
        }
    }
    pub fn len(&self) -> usize {
        self.spans.len()
    }
    pub fn owned(&self, set_id: SetId) -> &[T] {
        let span = self.spans[set_id].clone();
        &self.elements[span]
    }
    pub fn owner(&self, item: T) -> SetId {
        self.owners[&item]
    }
    pub fn canonical(&self, set_id: SetId) -> T {
        self.elements[self.spans[set_id].start]
    }
    pub fn mark(&mut self, item: T) {
        let owner = self.owners[&item];
        let i = self.locations[&item];
        let j = self.spans[owner].start + self.marked[owner];
        assert!(i >= j, "{:?} was already marked", item);

        if i > j {
            // Swap this to the contiguous "marked" region of this set.
            let target = self.elements[j];
            self.elements.swap(i, j);
            self.locations.insert(item, j);
            self.locations.insert(target, i);
        }
        if self.marked[owner] == 0 {
            self.touched.push(owner);
        }
        self.marked[owner] += 1;
    }
    pub fn split(&mut self) {
        while let Some(s) = self.touched.pop() {
            let Range { start, end } = self.spans[s];
            let mid = start + self.marked[s];
            self.marked[s] = 0;
            if mid == end {
                // The entire set was marked, this is a noop.
                continue;
            }
            let s1 = self.spans.len();
            self.marked.push(0);
            if mid - start >= end - mid {
                // the unmarked part is smaller, so that's the new set
                self.spans.push(mid..end);
                self.spans[s] = start..mid;
            } else {
                // the marked part is smaller, so that's the new set
                self.spans.push(start..mid);
                self.spans[s] = mid..end;
            }
            for i in self.spans[s1].clone() {
                self.owners.insert(self.elements[i], s1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Partition;

    #[test]
    fn smoke_test() {
        let mut p = Partition::new(vec!['a', 'b', 'c', 'd']);
        assert_eq!(p.len(), 1);

        p.mark('a');
        p.mark('c');
        p.split();
        assert_eq!(p.len(), 2);
        assert_eq!(p.owned(0), vec!['a', 'c']);
        assert_eq!(p.owned(1), vec!['b', 'd']);

        p.mark('a');
        p.mark('d');
        p.split();
        assert_eq!(p.len(), 4);
    }
}
