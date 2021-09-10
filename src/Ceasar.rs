
pub struct Ceasar {
    shift: usize,
    alphabet: String
}

impl Ceasar {
    pub fn standard(shift: i32) -> Self {
        Self { 
            shift, 
            alphabet: String::from("abcdefghijklmnopqrstuvwxyz ") 
        }
    }

    pub fn new(shift: i32, alphabet: String) -> Self {
        Self {
            shift,
            alphabet
        }
    }

    pub fn encrypt(&self, text: &str) -> Option<String> {
        text.chars()
            .map(|c| {
                let index =
                    self.alphabet
                        .find(c)
                        .map(|i| {
                            let text_len = text.len();
                            let index_of_char = (i + self.shift) % text_len;
                            text.chars().nth(index_of_char)
                        });
            })
            .collect()
    }

    pub fn decrypt(text: &str) -> String {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_text_is_encrpyted_with_a_shift_of_3() {
        // letters should transform into a letter 3 characters down the alphabet
        let text = "aaa";
        let ceasar = Ceasar::standard(3);

        assert_eq!()
    }
}

