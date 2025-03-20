use rotary_permutator::EnumRotor;

#[derive(EnumRotor, Clone, Debug, Default)]
pub enum Levels {
    #[default]
    High,
    Normal,
    Low,
}

fn main() {
    let mut levels_engine = Levels::init_rotor_engine(2);
    while let Some(val) = levels_engine.next() {
        println!("{:?}", val);
    }
}
