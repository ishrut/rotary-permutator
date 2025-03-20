/// This struct is initialised and tracks states for the iterator.
#[derive(Debug, Clone)]
pub struct CharRotor {
    /// Current state of the machine
    blocks: Vec<char>,
    /// List of provided chars
    chars: Vec<char>,
    /// Tracks end of cycleto change states
    trigger: Option<usize>,
    /// Tracks when to yield None
    final_trigger: bool,
}

impl CharRotor {
    /// Creates an initial state for the machine from a given size.
    /// The size is the length of the permutation.
    /// # Example
    /// ```
    /// let mut char_rotor = CharRotor::init(vec!['a', 'b', 'c']);
    /// ```
    pub fn init(chars: Vec<char>, size: usize) -> Self {
        let mut blocks = Vec::new();
        for _i in 0..size {
            blocks.push(chars[0].clone());
        }
        Self {
            blocks,
            trigger: None,
            final_trigger: false,
            chars,
        }
    }

    /// Gets a capture of the current state of the machine.
    pub fn current_blocks(&self) -> Vec<char> {
        self.blocks.clone()
    }

    // Finds the index of a given char in self.chars
    fn get_char_idx(&self, ch: &char) -> Option<usize> {
        for (count, symbol) in self.chars.iter().enumerate() {
            if symbol == ch {
                return Some(count);
            }
        }
        return None;
    }

    // Rotates the wheel at index to the next state and sets triggers
    fn rot_at(&mut self, index: usize) {
        let symbol = self.blocks[index];
        let symb_idx = self.get_char_idx(&symbol).unwrap();

        if symb_idx < self.chars.len() - 1 {
            self.blocks[index] = self.chars[symb_idx + 1];
        } else {
            if index > 0 {
                self.trigger = Some(index - 1);
                self.blocks[index] = self.chars[0].clone();
            } else {
                self.final_trigger = true;
            }
        }
    }
}

impl std::iter::Iterator for CharRotor {
    type Item = Vec<char>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(idx) = self.trigger {
            self.trigger = None;
            self.rot_at(idx);
        }
        let cap = self.blocks.clone();
        self.rot_at(self.blocks.len() - 1);

        if self.final_trigger {
            return None;
        }

        Some(cap)
    }
}
