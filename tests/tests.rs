use ntest::{assert_false, timeout};
use rand::{seq::SliceRandom as _, thread_rng, Rng as _};
use std::collections::{HashMap, HashSet};
use avltree::AVLTreeMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Number(i32);

#[test]
fn empty() {
    let mut map = AVLTreeMap::new();
    assert!(map.is_empty());
    assert_eq!(map.insert(1, 1), None);
    assert_eq!(map.insert(2, 2), None);
    assert_eq!(map.insert(3, 3), None);
    assert!(!map.is_empty());
}
#[test]
fn test_example() {
    let init_numbers = Vec::from([9, 6, 3, 4, 8, 1, 5, 15, 10, 7, 13, 12, 11, 2, 14]);
    let deleted_numbers = [4, 9, 6, 10, 7, 2, 13];

    let mut avl_tree = AVLTreeMap::new();

    for num in &init_numbers {
        avl_tree.insert(*num, *num);
    }

    let set_numbers: HashSet<i32> = init_numbers.into_iter().collect();
    let set_deleted: HashSet<i32> = deleted_numbers.into_iter().collect();

    let mut dif: Vec<i32> = set_numbers.difference(&set_deleted).cloned().collect();

    for num in &deleted_numbers {
        avl_tree.remove(num);
        println!("After del {num}");
        for i in 0..avl_tree.len() {
            let n_th_value = avl_tree.nth_key_value(i).unwrap();
            print!("{}  ", *n_th_value.0)
        }
        println!();
    }

    dif.sort_unstable();
    println!();
    println!("{:?}", dif);
}

#[test]
fn test_avl_tree() {
    for _ in 0..100000 {
        let mut avl_tree = AVLTreeMap::new();
        let mut rng = thread_rng();

        // Generate up to 20 unique i32 numbers
        let mut unique_numbers: Vec<i32> = (1..=15).collect();
        unique_numbers.shuffle(&mut rng);
        // unique_numbers.truncate(rng.gen_range(7..=13));

        // Add unique numbers to AVL tree
        for &num in &unique_numbers {
            avl_tree.insert(num, num);
        }

        let init_numbers = unique_numbers.clone();

        unique_numbers.shuffle(&mut rng);
        let mut deleted_numbers = Vec::new();

        for _ in 0..unique_numbers.len() / 2 {
            let num = unique_numbers.pop().unwrap();
            avl_tree.remove(&num);
            deleted_numbers.push(num);
        }

        // Check the correctness of nth_key_value method
        let mut sorted_numbers = unique_numbers.clone();
        sorted_numbers.sort();

        for (i, num) in sorted_numbers.iter().enumerate() {
            let nth_key_value = avl_tree.nth_key_value(i).unwrap();
            assert_eq!(
                nth_key_value,
                (num, num),
                "Failed at index {i}. Initial numbers {:?}. Deleted numbers: {:?}",
                init_numbers,
                deleted_numbers
            );
        }

        // Verify deleted numbers are not present in the tree
        for num in deleted_numbers {
            assert!(!avl_tree.contains_key(&num));
        }
    }
}

#[test]
fn test_root() {
    let mut map = AVLTreeMap::new();
    assert_eq!(map.insert(1, 1), None);
    assert_eq!(map.insert(4, 4), None);
    assert_eq!(map.insert(2, 2), None);
}

#[test]
fn test_remove() {
    let mut map = AVLTreeMap::new();
    map.insert(30, 30);
    map.insert(15, 15);
    map.insert(45, 45);
    map.insert(7, 7);
    map.remove_entry(&45);
    map.insert(23, 23);
    map.insert(35, 35);
    map.insert(51, 51);
    map.insert(3, 3);
    map.insert(11, 11);
    map.insert(19, 19);
    map.insert(32, 32);
    map.insert(33, 33);
    map.insert(34, 34);
    map.insert(40, 40);
    map.remove_entry(&40);
    map.remove_entry(&30);
    map.remove_entry(&33);
    map.remove_entry(&19);
    map.remove_entry(&7);
}

#[test]
fn should_compile1() {
    let mut map = AVLTreeMap::new();
    assert_eq!(map.insert(Number(1), 1), None);
    assert!(map.contains_key(&Number(1)));
}

#[test]
fn should_compile2() {
    let mut map = AVLTreeMap::new();
    assert_eq!(map.remove("hello"), None);
    assert_eq!(map.insert("hello".to_string(), 1), None);
    assert!(map.contains_key("hello"));
    assert!(!map.contains_key("world"));
    assert_eq!(map.remove_entry("hello"), Some(("hello".to_string(), 1)));
}

#[test]
fn contains() {
    let mut map = AVLTreeMap::new();
    assert_eq!(map.insert(1, 1), None);
    assert_eq!(map.insert(2, 2), None);
    assert_eq!(map.insert(3, 3), None);
    assert!(!map.contains_key(&0));
    assert!(map.contains_key(&1));
    assert!(map.contains_key(&2));
    assert!(map.contains_key(&3));
    assert!(!map.contains_key(&4));
}

#[test]
fn remove() {
    let mut map = AVLTreeMap::new();
    assert_eq!(map.insert(1, 1), None);
    assert_eq!(map.insert(2, 2), None);
    assert_eq!(map.insert(3, 3), None);
    assert_eq!(map.remove(&1), Some(1));
    assert!(!map.contains_key(&1));
    assert!(map.contains_key(&2));
    assert!(map.contains_key(&3));
    assert_eq!(map.remove(&2), Some(2));
    assert!(!map.contains_key(&1));
    assert!(!map.contains_key(&2));
    assert!(map.contains_key(&3));
    assert_eq!(map.remove(&3), Some(3));
    assert!(!map.contains_key(&1));
    assert!(!map.contains_key(&2));
    assert!(!map.contains_key(&3));
    assert!(map.is_empty());
}

#[test]
fn test_nth() {
    let mut map = AVLTreeMap::<u8, u8>::new();
    assert_eq!(map.insert(2, 2), None);
    assert_eq!(map.insert(1, 1), None);
    assert_eq!(map.insert(3, 3), None);

    assert_eq!(map.remove_entry(&2), Some((2, 2)));
    assert_eq!(map.insert(2, 2), None);

    assert_eq!(map.nth_key_value(0), Some((&1, &1)));
    assert_eq!(map.nth_key_value(1), Some((&2, &2)));
    assert_eq!(map.nth_key_value(2), Some((&3, &3)));
}

#[test]
#[timeout(1500)]
fn performance1() {
    let count = 10000000;
    let mut rng = thread_rng();
    let mut map = AVLTreeMap::new();
    let mut hash_map = HashMap::<u8, u8>::new();
    for _ in 0..count {
        let key = rng.gen();
        let value = rng.gen();
        map.insert(key, value);
        hash_map.insert(key, value);
    }
    let mut vec: Vec<_> = hash_map.into_iter().collect();
    vec.sort_unstable();
    let mut vec: Vec<_> = vec
        .into_iter()
        .enumerate()
        .map(|(index, (key, value))| (key, value, index))
        .collect();
    vec.shuffle(&mut rng);
    for (key, value, index) in &vec {
        assert!(map.contains_key(key));
        assert_eq!(map.nth_key_value(*index), Some((key, value)));
        assert_eq!(map.get_key_value(key), Some((key, value)));
    }
    for (key, value, _) in &vec {
        assert_eq!(map.remove_entry(key), Some((*key, *value)));
        assert!(!map.contains_key(key));
    }
}

#[test]
#[timeout(2500)]
fn performance2() {
    let count = 8000000;
    let mut rng = thread_rng();
    let mut map = AVLTreeMap::new();
    let mut hash_map = HashMap::<u8, u8>::new();
    for _ in 0..count {
        let key = rng.gen();
        let value = rng.gen();
        match rng.gen_range(0usize..10) {
            0..=7 => {
                assert_eq!(map.insert(key, value), hash_map.insert(key, value));
            }
            8 => {
                assert_eq!(map.remove(&key), hash_map.remove(&key));
            }
            9 => {
                assert_eq!(map.remove_entry(&key), hash_map.remove_entry(&key));
            }
            _ => unreachable!(),
        }
        assert_eq!(map.is_empty(), hash_map.is_empty());
        assert_eq!(map.len(), hash_map.len());
        assert_eq!(map.contains_key(&key), hash_map.contains_key(&key));
        assert_eq!(map.get(&key), hash_map.get(&key));
        assert_eq!(map.get_key_value(&key), hash_map.get_key_value(&key));
    }
}

#[test]
#[timeout(1500)]
fn performance3() {
    let count = 1000000;
    let mut rng = thread_rng();
    let mut map = AVLTreeMap::<i32, i32>::new();
    for i in 0..count {
        let value = rng.gen();
        map.insert(i, value);
    }
    for _ in 0..count {
        assert_false!(map.contains_key(&count));
    }
    for i in 1000..count {
        map.remove(&i);
    }
    for i in 0..count {
        let value = rng.gen();
        map.insert(-i, value);
    }
    for _ in 0..count {
        assert_false!(map.contains_key(&(-count)));
    }
}
