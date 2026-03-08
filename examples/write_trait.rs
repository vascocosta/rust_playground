use std::io::{self, Write};

enum Encryption {
    Caesar(u8),
    Xor(u8),
}

struct Encrypter<W: Write> {
    inner: W,
    encryption: Encryption,
}

impl<W: Write> Encrypter<W> {
    fn new(inner: W, encryption: Encryption) -> Self {
        Self { inner, encryption }
    }

    fn caesar(&self, buf: &[u8], shift: u8) -> Vec<u8> {
        buf.iter().map(|b| b.wrapping_add(shift)).collect()
    }

    fn xor(&self, buf: &[u8], key: u8) -> Vec<u8> {
        buf.iter().map(|b| b ^ key).collect()
    }
}

impl<W: Write> Write for Encrypter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let encrypted = match self.encryption {
            Encryption::Xor(key) => self.xor(buf, key),
            Encryption::Caesar(shift) => self.caesar(buf, shift),
            _ => todo!(),
        };

        self.inner.write(&encrypted)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

fn main() -> io::Result<()> {
    let data = "Hello this is a secret message!";
    let mut encrypted = Vec::new();
    let mut encrypter = Encrypter::new(&mut encrypted, Encryption::Caesar(10));

    write!(&mut encrypter, "{}", data)?;
    encrypter.flush()?;

    println!("Encrypted data: {:?}", encrypted);

    Ok(())
}
