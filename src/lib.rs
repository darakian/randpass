use rand::prelude::*;

struct Alphabet<T>{
    chars: Vec<T>,
}

impl<T> Alphabet<T>{
    pub fn new(characters: Vec<T>) -> Self{
        Alphabet{chars: characters}
    }

    pub fn rand_elem(&self, rng: &mut rand::rngs::ThreadRng) -> Option<&T> {
        if self.chars.len() == 0{
            return None
        }
        Some(&self.chars[rng.gen_range(0, self.chars.len())])
    }
}

pub fn test() -> String {
    let mut byte = [0u8; 1];
    rand::thread_rng().fill_bytes(&mut byte);
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    rand_n_elems(10, Alphabet::new(chars))
}

fn rand_n_elems(n: u32, alph: Alphabet<char>) -> String{
    let mut rng = thread_rng();
    let mut chars = Vec::new();
    for _i in 0..=n {
        chars.push(alph.rand_elem(&mut rng).unwrap());
    }
    chars.into_iter().collect()
}