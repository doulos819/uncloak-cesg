// Thor's Code for week 1 cipher question. We shall bench!

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use anyhow::Result;

pub mod vignere {

    #[derive(Clone, Debug)]
    pub struct Vignere<'a> {
    key: &'a [u8],
    }

    fn check_alphabetic(s: &str) -> anyhow::Result<()> {
    for c in s.chars() {
        match c {
        'a'..='z' => (),
        _ => return Err(anyhow::anyhow!("Invalid character in key, must be lowercase a-z: {c}")),
        }
    }
    Ok(())
    }

    impl<'a> Vignere<'a> {
    pub fn new(key: &'a str) -> anyhow::Result<Self> {
        check_alphabetic(key)?;
        Ok(Self { key: key.as_bytes() })
    }

    pub fn encrypt(&self, plaintext: &str) -> anyhow::Result<Vec<u8>> {
        check_alphabetic(plaintext)?;
        let key_it = self.key.iter().cycle();
        let out = std::iter::zip(plaintext.bytes(), key_it)
        .map(|(p, k)| {
            let p = p - b'a';
            let k = k - b'a';
            let c = (p + k) % 26;
            c + b'a'
        })
        .collect::<Vec<_>>();
        Ok(out)
    }

    pub fn decrypt(&self, ciphertext: &str) -> anyhow::Result<String> {
        check_alphabetic(ciphertext)?;
        let key_it = self.key.iter().cycle();
        Ok(
        std::iter::zip(ciphertext.bytes(), key_it)
            .map(|(p, k)| {
            let p = p - b'a';
            let k = k - b'a';
            let c = (26 + p - k) % 26;
            c + b'a'
            })
            .map(|c| c as char)
            .collect(),
        )
    }
    }

    #[cfg(test)]
    mod test {
    use super::*;
    #[test]
    fn test_vig() {
        let msg = "aoeuidhtnsqjkxbmwvzpyfgcrl";
        let key = "averygoodkey";
        let v = Vignere::new(key).unwrap();
        let ciphertext = v.encrypt(msg).unwrap();
        let plaintext = v.decrypt(&String::from_utf8(ciphertext).unwrap()).unwrap();
        assert_eq!(msg, plaintext);
    }
    }
}

