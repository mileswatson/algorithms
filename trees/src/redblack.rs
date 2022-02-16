use std::{mem::take, ops::Not};

use Color::*;
use RBTree::*;
use Side::*;

use crate::SearchTree;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub enum RBTree<T> {
    Node(Color, T, Box<RBTree<T>>, Box<RBTree<T>>),
    Leaf,
}

impl<T> Default for RBTree<T> {
    fn default() -> RBTree<T> {
        Leaf
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

enum Fixup {
    Fixed,
    CheckRed,
    Fix(Side),
}

impl<T: Ord> RBTree<T> {
    fn red(value: T) -> RBTree<T> {
        Node(Red, value, Box::new(Leaf), Box::new(Leaf))
    }

    fn color(&self) -> Color {
        match self {
            Node(color, ..) => *color,
            Leaf => Black,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        match self {
            Node(col, ..) => *col = color,
            Leaf => panic!(),
        }
    }

    fn check_invariant(&self) -> Option<u32> {
        match self {
            Node(Black, _, left, right) => {
                let d = left.check_invariant()?;
                if d == right.check_invariant()? {
                    Some(d + 1)
                } else {
                    None
                }
            }
            Node(Red, _, left, right) => match (left.color(), right.color()) {
                (Black, Black) => {
                    let d = left.check_invariant()?;
                    if d == right.check_invariant()? {
                        Some(d)
                    } else {
                        None
                    }
                }
                (_, _) => None,
            },
            Leaf => Some(0),
        }
    }

    fn invariant_holds(&self) -> bool {
        self.check_invariant().is_some()
    }

    fn get_mut(&mut self, side: Side) -> &mut RBTree<T> {
        match self {
            Node(_, _, left, right) => match side {
                Left => left.as_mut(),
                Right => right.as_mut(),
            },
            Leaf => panic!(),
        }
    }

    fn rotate(&mut self, direction: Side) -> Option<()> {
        let mut old = take(self);
        let mut new = take(old.get_mut(!direction));
        let mid = take(new.get_mut(direction));

        *old.get_mut(!direction) = mid;
        *new.get_mut(direction) = old;
        *self = new;
        Some(())
    }

    fn _insert(&mut self, other: T) -> Fixup {
        let pside = match self {
            Leaf => {
                *self = RBTree::red(other);
                return Fixup::CheckRed;
            }
            Node(_, val, _, _) => {
                if &other < val {
                    Left
                } else {
                    Right
                }
            }
        };
        let fixup = self.get_mut(pside)._insert(other);

        let zside = match fixup {
            Fixup::Fixed => return Fixup::Fixed,
            Fixup::CheckRed => {
                return if self.color() == Black {
                    Fixup::Fixed
                } else {
                    Fixup::Fix(pside)
                }
            }
            Fixup::Fix(zside) => zside,
        };

        while self.get_mut(pside).color() == Red {
            let u = self.get_mut(!pside);
            if u.color() == Red {
                u.set_color(Black);
                self.get_mut(pside).set_color(Black);
                self.set_color(Red);
                debug_assert!(self.invariant_holds());
                return Fixup::CheckRed;
            } else {
                let p = self.get_mut(pside);
                if zside != pside {
                    p.rotate(pside);
                }
                p.set_color(Black);
                self.set_color(Red);
                self.rotate(!pside);
            }
        }
        debug_assert!(self.invariant_holds());
        Fixup::Fixed
    }

    fn fill_vec(self, v: &mut Vec<T>) {
        if let Node(_, val, left, right) = self {
            left.fill_vec(v);
            v.push(val);
            right.fill_vec(v);
        }
    }
}

impl<T: Ord> SearchTree<T> for RBTree<T> {
    fn new() -> RBTree<T> {
        Leaf
    }

    fn insert(&mut self, other: T) {
        self._insert(other);
        self.set_color(Black);
        debug_assert!(self.invariant_holds())
    }

    fn delete(&mut self, _: &T) -> Option<T> {
        todo!()
    }

    fn to_vec(self) -> Vec<T> {
        let mut v = Vec::new();
        self.fill_vec(&mut v);
        v
    }
}

#[cfg(test)]
mod test {
    use crate::test::insertion;

    use super::RBTree;

    #[test]
    fn redblack_tree_test() {
        insertion::<RBTree<i32>>();
    }
}
