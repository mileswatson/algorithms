use bit_vec::{BitVec, Iter};

use super::encoding::EncodeTree;

#[derive(Debug)]
pub enum Decoder<T> {
    Parent(Box<Decoder<T>>, Box<Decoder<T>>),
    Leaf(T),
}

impl<T: Clone> Decoder<T> {
    pub fn from(tree: EncodeTree<T>) -> Decoder<T> {
        match tree {
            EncodeTree::Parent(left, right, _) => Decoder::Parent(
                Box::new(Decoder::from(*left)),
                Box::new(Decoder::from(*right)),
            ),
            EncodeTree::Leaf(value, _) => Decoder::Leaf(value),
        }
    }

    fn decode_item(&self, iter: &mut Iter) -> Option<T> {
        match self {
            Decoder::Parent(left, right) => {
                if iter.next()? {
                    right.decode_item(iter)
                } else {
                    left.decode_item(iter)
                }
            }
            Decoder::Leaf(v) => Some(v.clone()),
        }
    }

    fn decode(&self, bits: &BitVec, count: u64) -> Option<Vec<T>> {
        let mut items = Vec::new();
        let mut iter = bits.iter();
        for _ in 0..count {
            items.push(self.decode_item(&mut iter)?);
        }
        Some(items)
    }
}

#[derive(Debug)]
pub enum Encoded<T> {
    Empty,
    Huffman(Vec<u8>, u64, Decoder<T>),
}

impl<T: Clone> Encoded<T> {
    pub fn decode(&self) -> Option<Vec<T>> {
        match self {
            Encoded::Empty => Some(Vec::new()),
            Encoded::Huffman(bytes, count, decoder) => {
                let bits = BitVec::from_bytes(bytes);
                decoder.decode(&bits, *count)
            }
        }
    }
}
