use rotary_permutator::EnumRotor;

#[derive(EnumRotor, Clone, Debug)]
pub enum Levels {
    High,
    Normal,
    Low,
}

impl std::default::Default for Levels {
    fn default() -> Self {
        Self::High
    }
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
