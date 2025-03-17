use rotary_permutator::CharRotor;

fn main() {

    let mut char_rotor = CharRotor::init(vec!['x', 'y', 'z'], 3);
    while let Some(permutations) = char_rotor.next() {
        println!("{:?}", permutations);
    }
}
