#![forbid(unsafe_code)]

use std::borrow::Borrow;
use std::mem::replace;

pub struct Node<K, V> {
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    balance_factor: Factor,
    key: K,
    value: Option<V>,
    pub left_count: usize,
    pub right_count: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Factor {
    LeftHeavy,
    Balanced,
    RightHeavy,
}

impl Factor {
    pub fn as_int(&self) -> i8 {
        match *self {
            Factor::LeftHeavy => 1,
            Factor::Balanced => 0,
            Factor::RightHeavy => -1,
        }
    }
}

impl<K: Ord, V> From<Node<K, V>> for Option<Box<Node<K, V>>> {
    fn from(node: Node<K, V>) -> Self {
        Some(Box::new(node))
    }
}

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            left: None,
            right: None,
            balance_factor: Factor::Balanced,
            key,
            value: Some(value),
            left_count: 0,
            right_count: 0,
        }
    }

    pub fn balance(&self) -> &Factor {
        &self.balance_factor
    }

    pub fn get_left(&self) -> &Option<Box<Node<K, V>>> {
        &self.left
    }

    pub fn get_right(&self) -> &Option<Box<Node<K, V>>> {
        &self.right
    }

    pub fn get_key(&self) -> &K {
        &self.key
    }

    pub fn get_key_borrowed<Q: ?Sized>(&self) -> &Q
        where
            K: Borrow<Q>,
    {
        self.key.borrow()
    }

    pub fn get_key_value_tuple(self) -> (K, V) {
        (self.key, self.value.unwrap())
    }

    pub fn get_value(&self) -> Option<&V> {
        self.value.as_ref()
    }

    pub fn get_key_value(&self) -> (&K, &V) {
        (&self.key, &self.value.as_ref().unwrap())
    }

    pub fn set_right(&mut self, node: Option<Box<Node<K, V>>>) {
        self.right = node;
    }

    pub fn set_left(&mut self, node: Option<Box<Node<K, V>>>) {
        self.left = node;
    }

    pub fn replace_value(&mut self, value: V) -> V {
        self.value.replace(value).unwrap()
    }

    pub fn take_left(&mut self) -> Option<Box<Node<K, V>>> {
        self.left.take()
    }

    pub fn take_right(&mut self) -> Option<Box<Node<K, V>>> {
        self.right.take()
    }

    pub fn set_balance(&mut self, balance: Factor) {
        self.balance_factor = balance;
    }

    pub fn is_leaf(&self) -> bool {
        self.get_left().is_none() && self.get_right().is_none()
    }

    pub fn replace_key_value(&mut self, tuple: (K, V)) -> (K, V) {
        let old_key = replace(&mut self.key, tuple.0);
        let old_value = self.value.replace(tuple.1);
        (old_key, old_value.unwrap())
    }

    pub fn left_count(&self) -> usize {
        self.left_count
    }

    pub fn right_count(&self) -> usize {
        self.right_count
    }
}
