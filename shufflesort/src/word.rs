#[derive(Debug)]
pub struct Word<'a> {
    pub hash: u128,
    pub source: &'a [u8],
}

#[derive(Debug)]
pub struct WordIter<'a> {
    buf: &'a [u8],
}

impl<'a> From<&'a [u8]> for WordIter<'a> {
    fn from(buf: &'a [u8]) -> Self {
        Self { buf }
    }
}

impl<'a> Iterator for WordIter<'a> {
    type Item = Word<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            return None;
        }

        const RADIX: u128 = 26;

        let mut iter = self.buf.iter();
        let mut hash: u128 = 0;
        let len = iter
            .by_ref()
            .take_while(|b| **b != b'\n')
            .inspect(|b| hash = hash * RADIX + **b as u128) // FIX: use sort_key(*b)
            .count();
        hash *= RADIX.pow(20 - len as u32); // PERF: lookup table for pow()

        let (source, remaining) = self.buf.split_at(len + 1);
        self.buf = remaining;
        Some(Word { hash, source })
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    const DATA: &[u8] = b"foo\nbar\nbaz\n";

    #[test]
    fn worditer() {}
}
