
pub struct TryReader<'l, T> {
    elements: &'l Vec<T>,
    needle: usize,
}

impl<'l, T> TryReader<'l, T> {
    pub fn new(elements: &'l Vec<T>) -> TryReader<'l, T> {
        TryReader {
            elements,
            needle: 0,
        }
    }
    pub fn next(&mut self) -> Option<&T> {
        if self.has_next() {
            let t = &self.elements[self.needle];
            self.needle += 1;
            Some(t)
        } else {
            None
        }
    }
    pub fn try_<S, F>(&mut self, f: F) -> Result<(usize, S), Option<S>>
    where F: FnOnce(&mut TryReader<T>) -> Result<S, Option<S>> {
        let mut clone = TryReader {
            elements: self.elements,
            needle: self.needle
        };
        match f(&mut clone).map(|result| (clone.needle - self.needle, result)) {
            Ok(result) => {
                self.needle = clone.needle;
                Ok(result)
            },
            Err(Some(replacement)) => Ok((clone.needle - self.needle, replacement)),
            Err(None) => Err(None),
        }
    }

    pub fn has_next(&self) -> bool {
        self.elements.len() != self.needle
    }
}