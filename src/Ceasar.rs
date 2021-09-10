
pub struct Ceasar {
    shift: usize,
    alphabet: Vec<char>
}

impl Ceasar {
    pub fn standard(shift: usize) -> Self {
        Self { 
            shift, 
            alphabet: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'] 
        }
    }

    pub fn new(shift: usize, alphabet: Vec<char>) -> Self {
        Self {
            shift,
            alphabet
        }
    }

    pub fn encrypt(&self, text: &str) -> Option<String> {
        text
            .chars()
            .map(|c| {
                self.alphabet
                    .iter()
                    .position(|ac| *ac == c)
                    .map(|i| {
                        let alphabet_len = self.alphabet.len();
                        let index_of_char = (i + self.shift) % alphabet_len;
                        self.alphabet[index_of_char]
                    })
            })
            .collect()
    }

    // pub fn decrypt(text: &str) -> String {

    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_text_is_encrpyted_with_a_shift_of_3() {
        // letters should transform into a letter 3 characters down the alphabet
        let text = "aaa";
        let ceasar = Ceasar::standard(3);

        let result = ceasar.encrypt(text).unwrap();

        assert_eq!("ddd".to_string() , result);
    }

    #[test]
    fn when_text_is_encrpyted_with_a_shift_of_3_and_there_is_only_1_letter_left_in_the_alphabet() {
        // letters should transform into a letter 3 characters down the alphabet
        let text = "yyy";
        let ceasar = Ceasar::standard(3);

        let result = ceasar.encrypt(text).unwrap();

        // then should wrap to start of alphabet
        assert_eq!("bbb".to_string() , result);
    }

    #[test]
    fn just_for_fun() {
        // letters should transform into a letter 3 characters down the alphabet
        let text = "helloworld";
        let ceasar = Ceasar::standard(3);

        let result = ceasar.encrypt(text).unwrap();

        // then should wrap to start of alphabet
        assert_eq!("khoorzruog".to_string() , result);
    }
}

