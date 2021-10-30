
// TODO - switch to indexedset? from IndexedMap crate
pub struct Vigenere {
    shift: String,
    alphabet: Vec<char>,
    missing_char: char
}

impl Vigenere {
    pub fn new(shift: String) -> Self {
        Self {
            shift,
            alphabet: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
            missing_char: '_'
        }
    }

    pub fn encrypt(&self, text: &str) -> String {
        text
            .chars()
            .zip(self.shift.chars().cycle())
            .map(|(it, key)| {
                self.alphabet
                    .iter()
                    .enumerate()
                    .scan((usize::MAX, usize::MAX), |result, (i, next)| {
                        if it == *next {
                            result.0 = i;
                            Some(*result)
                        } else if key == *next {
                            result.1 = i;
                            Some(*result)
                        } else {
                            Some(*result)
                        }
                    })
                    .skip_while(|result| result.0 == usize::MAX || result.1 == usize::MAX)
                    .next()
                    .map_or(self.missing_char, |(it_idx, key_idx)| {
                        let index = (it_idx + key_idx) % self.alphabet.len();
                        self.alphabet[index]
                    })
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_text_is_encrpyted_with_a_shift_of_3() {
        // letters should transform into a letter 3 characters down the alphabet
        let text = "ccc";
        let vig = Vigenere::new(String::from("hi"));

        let result = vig.encrypt(text);

        assert_eq!("jkj".to_string() , result);
    }

    #[test]
    fn when_text_is_encrpyted_with_a_shift_of_3_and_there_is_only_1_letter_left_in_the_alphabet() {
        // letters should transform into a letter 3 characters down the alphabet
        let text = "yyy";
        let vig = Vigenere::new(String::from("hi"));

        let result = vig.encrypt(text);

        // then should wrap to start of alphabet
        assert_eq!("fgf".to_string() , result);
    }

    // #[test]
    // fn missing_character() {
    //     // letters should transform into a letter 3 characters down the alphabet
    //     let text = "hello world";
    //     let ceasar = Ceasar::standard(3);

    //     let result = ceasar.encrypt(text);

    //     // then should wrap to start of alphabet
    //     assert_eq!("khoor_zruog".to_string() , result);
    // }
}



