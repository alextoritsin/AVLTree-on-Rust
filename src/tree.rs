#![forbid(unsafe_code)]

use crate::node::Factor::{Balanced, LeftHeavy, RightHeavy};
use crate::node::Node;
use crate::tree::Direction::{Left, Right};
use std::borrow::Borrow;
use std::cmp::Ordering::{Equal, Greater, Less};

pub struct AVLTreeMap<K: Ord, V> {
    root: Option<Box<Node<K, V>>>,
    size: usize,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

impl<K: Ord, V> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AVLTreeMap<K, V> {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub(self) fn set_root(&mut self, node: Option<Box<Node<K, V>>>) {
        self.root = node
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(node) = search(&self.root, key) {
            node.get_value()
        } else {
            None
        }
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized,
    {
        search(&self.root, key).is_some()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut need_balance = true;
        let opt_v = if let Some(node) = self.root.take() {
            let (opt_v, new_root) = insert(node, key, value, &mut need_balance);
            self.set_root(new_root.into());
            opt_v
        } else {
            self.set_root(Node::new(key, value).into());
            None
        };

        if opt_v.is_none() {
            self.size += 1;
            None
        } else {
            opt_v
        }
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized,
    {
        let mut need_balance = true;
        let opt_k_v = if let Some(node) = self.root.take() {
            let (new_root, opt_k_v) = delete_node(node.into(), key, &mut need_balance);
            self.set_root(new_root);
            opt_k_v
        } else {
            None
        };

        if opt_k_v.is_some() {
            self.size -= 1;
            opt_k_v
        } else {
            None
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized,
    {
        match self.remove_entry(key) {
            None => None,
            Some(tuple) => Some(tuple.1),
        }
    }

    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        search(&self.root, key)
            .as_ref()
            .map(|node| node.get_key_value())
    }

    pub fn nth_key_value(&self, mut index: usize) -> Option<(&K, &V)> {
        let mut node = &self.root;
        loop {
            if let Some(node_ref) = node {
                match index.cmp(&node_ref.left_count) {
                    Less => node = node_ref.get_left(),
                    Greater => {
                        index -= node_ref.left_count + 1;
                        node = node_ref.get_right();
                    }
                    Equal => break node.as_ref().map(|node| node.get_key_value()),
                }
            } else {
                break None;
            }
        }
    }
}

type TupleOption<K, V> = (Option<Box<Node<K, V>>>, Option<(K, V)>);

fn delete_node<K: Ord, V, Q>(
    node: Option<Box<Node<K, V>>>,
    key: &Q,
    need_balance: &mut bool,
) -> TupleOption<K, V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
{
    if let Some(mut current_node) = node {
        let (opt, child);
        match key.cmp(current_node.get_key_borrowed()) {
            Less => {
                (child, opt) = delete_node(current_node.take_left(), key, need_balance);
                current_node.set_left(child);
                if *need_balance {
                    match current_node.balance() {
                        Balanced => current_node.set_balance(RightHeavy),
                        LeftHeavy => current_node.set_balance(Balanced),
                        RightHeavy => {
                            let right_child = current_node.take_right().unwrap();
                            current_node = rebalance(current_node, right_child, Right);
                        }
                    }
                    *need_balance = *current_node.balance() == Balanced;
                }
                update_count(&mut current_node);
                (current_node.into(), opt)
            }
            Greater => {
                (child, opt) = delete_node(current_node.take_right(), key, need_balance);
                current_node.set_right(child);
                if *need_balance {
                    match current_node.balance() {
                        Balanced => current_node.set_balance(LeftHeavy),
                        RightHeavy => current_node.set_balance(Balanced),
                        LeftHeavy => {
                            let left_child = current_node.take_left().unwrap();
                            current_node = rebalance(current_node, left_child, Left);
                        }
                    }
                    *need_balance = *current_node.balance() == Balanced;
                }
                update_count(&mut current_node);
                (current_node.into(), opt)
            }
            Equal => remove_node(current_node, need_balance),
        }
    } else {
        *need_balance = false;
        (node, None)
    }
}

fn remove_node<K: Ord, V>(mut node: Box<Node<K, V>>, need_balance: &mut bool) -> TupleOption<K, V> {
    match (node.get_left().is_none(), node.get_right().is_none()) {
        (true, true) => (None, Some(node.get_key_value_tuple())),
        (true, false) => (node.take_right(), Some(node.get_key_value_tuple())),
        (false, true) => (node.take_left(), Some(node.get_key_value_tuple())),
        (_, _) => find_closest(node, need_balance),
    }
}

fn find_closest<K: Ord, V>(
    mut node: Box<Node<K, V>>,
    need_balance: &mut bool,
) -> TupleOption<K, V> {
    let (child, opt_k_v) = match node.balance() {
        LeftHeavy | Balanced => find_predecessor(node.take_left(), need_balance),
        RightHeavy => find_successor(node.take_right(), need_balance),
    };

    if let Some(tuple) = opt_k_v {
        let old_key_value = node.replace_key_value(tuple);
        match node.balance() {
            LeftHeavy | Balanced => node.set_left(child),
            RightHeavy => node.set_right(child),
        };
        if *need_balance {
            match node.balance() {
                LeftHeavy | RightHeavy => node.set_balance(Balanced),
                Balanced => node.set_balance(RightHeavy),
            }
            *need_balance = *node.balance() == Balanced;
        };
        update_count(&mut node);
        (node.into(), Some(old_key_value))
    } else {
        let key_value = node.replace_key_value(child.unwrap().get_key_value_tuple());
        update_count(&mut node);
        (None, Some(key_value))
    }
}

fn find_successor<K: Ord, V>(
    node: Option<Box<Node<K, V>>>,
    need_balance: &mut bool,
) -> TupleOption<K, V> {
    if let Some(mut current_node) = node {
        let (l_child, opt) = find_successor(current_node.take_left(), need_balance);
        match &opt {
            None => {
                if let Some(right_child) = current_node.take_right() {
                    (right_child.into(), Some(current_node.get_key_value_tuple()))
                } else {
                    (None, Some(current_node.get_key_value_tuple()))
                }
            }
            Some(_) => {
                current_node.set_left(l_child);
                if *need_balance {
                    match current_node.balance() {
                        Balanced => current_node.set_balance(RightHeavy),
                        LeftHeavy => current_node.set_balance(Balanced),
                        RightHeavy => {
                            let right_child = current_node.take_right().unwrap();
                            current_node = rebalance(current_node, right_child, Right);
                        }
                    }
                    *need_balance = *current_node.balance() == Balanced;
                }
                update_count(&mut current_node);
                (current_node.into(), opt)
            }
        }
    } else {
        (node, None)
    }
}

fn find_predecessor<K: Ord, V>(
    node: Option<Box<Node<K, V>>>,
    need_balance: &mut bool,
) -> TupleOption<K, V> {
    if let Some(mut current_node) = node {
        let (r_child, opt) = find_predecessor(current_node.take_right(), need_balance);
        match &opt {
            None => {
                if let Some(left_child) = current_node.take_left() {
                    (left_child.into(), Some(current_node.get_key_value_tuple()))
                } else {
                    (None, Some(current_node.get_key_value_tuple()))
                }
            }
            Some(_) => {
                current_node.set_right(r_child);
                if *need_balance {
                    match current_node.balance() {
                        Balanced => current_node.set_balance(LeftHeavy),
                        RightHeavy => current_node.set_balance(Balanced),
                        LeftHeavy => {
                            let left_child = current_node.take_left().unwrap();
                            current_node = rebalance(current_node, left_child, Left);
                        }
                    }
                    *need_balance = *current_node.balance() == Balanced;
                }
                update_count(&mut current_node);
                (current_node.into(), opt)
            }
        }
    } else {
        (node, None)
    }
}

fn insert<K: Ord, V>(
    mut node: Box<Node<K, V>>,
    key: K,
    value: V,
    need_balance: &mut bool,
) -> (Option<V>, Box<Node<K, V>>) {
    match &key.cmp(node.get_key()) {
        Equal => {
            *need_balance = false;
            (Some(node.replace_value(value)), node)
        }
        Less => {
            let (opt_v, mut l_child);
            if let Some(left_child) = node.take_left() {
                (opt_v, l_child) = insert(left_child, key, value, need_balance);
            } else {
                (opt_v, l_child) = (None, Node::new(key, value).into());
            };

            node = if *need_balance {
                l_child = rebalance(node, l_child, Left);
                *need_balance = *l_child.balance() != Balanced;
                l_child
            } else {
                node.set_left(l_child.into());
                node
            };
            update_count(&mut node);
            (opt_v, node)
        }
        Greater => {
            let (opt_v, mut r_child);
            if let Some(right_child) = node.take_right() {
                (opt_v, r_child) = insert(right_child, key, value, need_balance);
            } else {
                (opt_v, r_child) = (None, Node::new(key, value).into());
            };

            node = if *need_balance {
                r_child = rebalance(node, r_child, Right);
                *need_balance = *r_child.balance() != Balanced;
                r_child
            } else {
                node.set_right(r_child.into());
                node
            };
            update_count(&mut node);
            (opt_v, node)
        }
    }
}

fn rebalance<K: Ord, V>(
    mut node_a: Box<Node<K, V>>,
    mut node_b: Box<Node<K, V>>,
    dir: Direction,
) -> Box<Node<K, V>> {
    match (node_a.balance(), &dir) {
        (Balanced, _) => {
            match dir {
                Left => {
                    node_a.set_balance(LeftHeavy);
                    node_a.set_left(node_b.into());
                }
                Right => {
                    node_a.set_balance(RightHeavy);
                    node_a.set_right(node_b.into());
                }
            };
            node_a
        }
        (RightHeavy, Left) | (LeftHeavy, Right) => {
            node_a.set_balance(Balanced);
            match dir {
                Left => node_a.set_left(node_b.into()),
                Right => node_a.set_right(node_b.into()),
            };
            node_a
        }
        (_, _) => {
            if node_a.balance().as_int() + node_b.balance().as_int() == 0 {
                do_big_rotation(node_a, node_b)
            } else {
                do_small_rotation(node_a, &mut node_b);
                node_b
            }
        }
    }
}

fn do_big_rotation<K: Ord, V>(
    mut node_a: Box<Node<K, V>>,
    mut node_b: Box<Node<K, V>>,
) -> Box<Node<K, V>> {
    let (a_bal, b_bal) = (*node_a.balance(), *node_b.balance());
    let mut node_c = match &b_bal {
        LeftHeavy | Balanced => node_b.take_left().unwrap(),
        RightHeavy => node_b.take_right().unwrap(),
    };
    if *node_c.balance() == Balanced {
        node_a.set_balance(Balanced);
        node_b.set_balance(Balanced);
    } else if *node_c.balance() == a_bal {
        node_a.set_balance(*node_b.balance());
        node_b.set_balance(Balanced);
        node_c.set_balance(Balanced);
    } else {
        node_b.set_balance(*node_a.balance());
        node_a.set_balance(Balanced);
        node_c.set_balance(Balanced);
    }
    match &b_bal {
        LeftHeavy | Balanced => rotate_right(node_b, &mut node_c),
        RightHeavy => rotate_left(node_b, &mut node_c),
    };
    match &a_bal {
        LeftHeavy | Balanced => rotate_right(node_a, &mut node_c),
        RightHeavy => rotate_left(node_a, &mut node_c),
    }
    node_c
}

fn do_small_rotation<K: Ord, V>(mut node_a: Box<Node<K, V>>, node_b: &mut Box<Node<K, V>>) {
    if node_a.balance() == node_b.balance() {
        node_a.set_balance(Balanced);
        match node_b.balance() {
            LeftHeavy | Balanced => rotate_right(node_a, node_b),
            RightHeavy => rotate_left(node_a, node_b),
        }
        node_b.set_balance(Balanced);
    } else {
        match node_a.balance() {
            LeftHeavy | Balanced => {
                node_b.set_balance(RightHeavy);
                rotate_right(node_a, node_b);
            }
            RightHeavy => {
                node_b.set_balance(LeftHeavy);
                rotate_left(node_a, node_b);
            }
        }
    }
}

fn rotate_right<K: Ord, V>(mut node_a: Box<Node<K, V>>, node_b: &mut Box<Node<K, V>>) {
    node_a.set_left(node_b.take_right());
    update_count(&mut node_a);
    node_b.set_right(node_a.into());
    update_count(node_b);
}

fn rotate_left<K: Ord, V>(mut node_a: Box<Node<K, V>>, node_b: &mut Box<Node<K, V>>) {
    node_a.set_right(node_b.take_left());
    update_count(&mut node_a);
    node_b.set_left(node_a.into());
    update_count(node_b);
}

fn search<'a, K: Ord, V, Q>(
    mut node: &'a Option<Box<Node<K, V>>>,
    key: &Q,
) -> &'a Option<Box<Node<K, V>>>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
{
    loop {
        if let Some(node_ref) = node {
            match key.cmp(node_ref.get_key_borrowed()) {
                Less => node = node_ref.get_left(),
                Greater => node = node_ref.get_right(),
                Equal => break node,
            }
        } else {
            break &None;
        }
    }
}

fn update_count<K: Ord, V>(node: &mut Box<Node<K, V>>) {
    node.left_count = node
        .get_left()
        .as_ref()
        .map_or(0, |n| n.left_count + n.right_count + 1);
    node.right_count = node
        .get_right()
        .as_ref()
        .map_or(0, |n| n.left_count + n.right_count + 1)
}
