# MerkelTreeExcercise

## Description

A simple implementation of a **Merkle Tree** data structure.

Merkle Trees are hash trees build out of arrays which lenght is a factor of 2. Each node in the tree holds a hash representantion of the data and two nodes.
These kind of trees are built from the bottom, by concatenating the hashes of the leaves two by two and repeating this procedure with the subtrees' roots of the following levels, until the root is reached.

![image](https://github.com/FrancoGiachetta/MerkleTreeExcercise/assets/170033636/b49f8524-b81b-4516-86ae-61b30906b6c6)

## Features

- [x] A Merkle Tree can be built out of an array.
- [ ] A Merkle Tree can generate a proof that it contains an element.
- [ ] A Merkle Tree can verify that a given hash is contained in it.
- [ ] A Merke Tree can be dynamic, this means that elements can be added once it is built.

## Usage

Use this command to run tests:

```rust
cargo test
```
