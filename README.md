# AVLTree on Rust

This library is a Rust implementation of one of self-balancing tree data structures - AVL Tree.
Developed as part of the assignments of great MIPT Rust [course](https://gitlab.com/alex.stanovoy/mipt-rust) by Alexander Stanovoy.

## API

The available methods repeat those of the HashMap data type with some additional functionality.
All operations are performed in `O(logn)` time.

1. Create a variable of type `AVLTreeMap<K: Ord, V>`.

  ```rust
  let mut tree_map = AVLTreeMap::new();
  ```

2. Insert a new node. Returns an `Option<V>`.

  ```rust
tree_map.insert("hello", 41); // -> None
tree_map.insert("hello", 1); // -> Some(41)
  ```

3. Delete node. Returns `Option<(K, V)>` or `Option<V>`

  ```rust
  tree_map.remove_entry("hello"); // -> Some(("hello", 1))
  ```
or

  ```rust
  tree_map.remove("hello"); // -> Some(1)
  ```
4. Check if a tree contains a node with some key.

  ```rust
  tree_map.contains_key("hello"); // -> false
  ```
5. Get key/value pair

  ```rust
tree_map.insert("hello", 42);
tree_map.get_key_value("hello"); // -> Some(("hello", &42)) 
  ```

6. Find n-th node by order, 0-indexed

  ```rust
tree_map.nth_key_value(0); // -> Some(("hello", &42))
  ```

7. Get the node value by key

  ```rust
tree_map.get("hello"); // -> Some(&42)
  ```

## Testing

Run tests with build optimizations.

  ```
cargo build --release
cargo test --release
  ```

