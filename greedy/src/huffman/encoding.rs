use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use bit_vec::BitVec;

use super::decoding::{Decoder, Encoded};

#[derive(Debug)]
pub enum EncodeTree<T> {
    Parent(Box<EncodeTree<T>>, Box<EncodeTree<T>>, u64),
    Leaf(T, u64),
}

impl<T> EncodeTree<T> {
    pub fn new(left: EncodeTree<T>, right: EncodeTree<T>) -> EncodeTree<T> {
        let cost = left.frequency() + right.frequency();
        EncodeTree::Parent(Box::new(left), Box::new(right), cost)
    }

    fn frequency(&self) -> u64 {
        let (EncodeTree::Parent(_, _, cost) | EncodeTree::Leaf(_, cost)) = self;
        *cost
    }
}

impl<T: Eq + Hash + Clone> EncodeTree<T> {
    fn generate_codebook(&self, codebook: &mut HashMap<T, BitVec>, prefix: &mut BitVec) {
        match self {
            EncodeTree::Parent(l, r, _) => {
                prefix.push(false);
                l.generate_codebook(codebook, prefix);
                prefix.pop();
                prefix.push(true);
                r.generate_codebook(codebook, prefix);
                prefix.pop();
            }
            EncodeTree::Leaf(v, _) => {
                codebook.insert(v.clone(), prefix.clone());
            }
        }
    }

    pub fn codebook(&self) -> HashMap<T, BitVec> {
        let mut codebook = HashMap::<T, BitVec>::new();
        let mut prefix = BitVec::new();
        self.generate_codebook(&mut codebook, &mut prefix);
        codebook
    }
}

impl<T> PartialEq for EncodeTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.frequency() == other.frequency()
    }
}

impl<T> Eq for EncodeTree<T> {}

impl<T> PartialOrd for EncodeTree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.frequency().partial_cmp(&self.frequency())
    }
}

impl<T: Eq> Ord for EncodeTree<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency().cmp(&self.frequency())
    }
}

fn create_tree<T: Hash + Eq + Clone>(items: &[T]) -> Option<EncodeTree<T>> {
    let mut counts: HashMap<T, u64> = HashMap::new();
    for item in items {
        if let Some(c) = counts.get_mut(item) {
            *c += 1
        } else {
            counts.insert(item.clone(), 1);
        }
    }
    let mut heap: BinaryHeap<EncodeTree<T>> = counts
        .into_iter()
        .map(|(t, f)| EncodeTree::Leaf(t, f))
        .collect();
    loop {
        let a = heap.pop()?;
        if let Some(b) = heap.pop() {
            heap.push(EncodeTree::new(a, b));
        } else {
            break Some(a);
        }
    }
}

pub fn encode_with_tree<T: Eq + Hash + Clone>(items: &[T], tree: &EncodeTree<T>) -> Vec<u8> {
    let codebook = tree.codebook();
    items
        .iter()
        .map(|item| codebook.get(item))
        .fold(BitVec::new(), |mut x, y| {
            x.extend(y.unwrap());
            x
        })
        .to_bytes()
}

pub fn encode<T: Hash + Eq + Clone>(items: &[T]) -> Encoded<T> {
    match create_tree(items) {
        None => Encoded::Empty,
        Some(t) => {
            let bytes = if let EncodeTree::Leaf(..) = t {
                Vec::new()
            } else {
                encode_with_tree(items, &t)
            };
            Encoded::Huffman(bytes, items.len() as u64, Decoder::from(t))
        }
    }
}
