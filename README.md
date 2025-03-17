# rotary-permutator - A library to generate permutations.

The algorithm is inspired from mechanical rotary permutation generators like the cryptex.
It generates permutations of a given length with repetition from a list of chars or variants of an enum.
There are functions to generate permutations from chars and enums.
It implements the Iterator trait for usability.

## Examples

To generate permutations from a list of chars and a given size of permutations:
```rust
use rotary_permutator::CharRotor;

fn main() {
    let mut char_rotor = CharRotor::init(vec!['x', 'y', 'z'], 3);
    while let Some(permutations) = char_rotor.next() {
        println!("{:?}", permutations);
    }
}
```

output:
```
['x', 'x', 'x']
['x', 'x', 'y']
['x', 'x', 'z']
['x', 'y', 'x']
['x', 'y', 'y']
['x', 'y', 'z']
['x', 'z', 'x']
['x', 'z', 'y']
['x', 'z', 'z']
['y', 'x', 'x']
['y', 'x', 'y']
['y', 'x', 'z']
['y', 'y', 'x']
['y', 'y', 'y']
['y', 'y', 'z']
['y', 'z', 'x']
['y', 'z', 'y']
['y', 'z', 'z']
['z', 'x', 'x']
['z', 'x', 'y']
['z', 'x', 'z']
['z', 'y', 'x']
['z', 'y', 'y']
['z', 'y', 'z']
['z', 'z', 'x']
['z', 'z', 'y']
['z', 'z', 'z']
```

Permutations from variants of an enum.
```rust
use rotary_permutator::EnumRotor;

#[derive(EnumRotor, Debug, Clone)]
pub enum Levels {
    High,
    Normal,
    Low,
}

fn main() {
    let mut rotor_engine = RotorEngine::init(3);
    let mut count = 0;
    while let Some(perm) = rotor_engine.next() {
        println!("{:?}", perm);
        count += 1;
    }
    println!("total: {}", count);
}
```

output:
```
[High, High, High]
[High, High, Normal]
[High, High, Low]
[High, Normal, High]
[High, Normal, Normal]
[High, Normal, Low]
[High, Low, High]
[High, Low, Normal]
[High, Low, Low]
[Normal, High, High]
[Normal, High, Normal]
[Normal, High, Low]
[Normal, Normal, High]
[Normal, Normal, Normal]
[Normal, Normal, Low]
[Normal, Low, High]
[Normal, Low, Normal]
[Normal, Low, Low]
[Low, High, High]
[Low, High, Normal]
[Low, High, Low]
[Low, Normal, High]
[Low, Normal, Normal]
[Low, Normal, Low]
[Low, Low, High]
[Low, Low, Normal]
[Low, Low, Low]
total: 27
```
