
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
    pub fn next(&mut self) -> Option<&'l T> {
        if self.has_next() {
            let t = &self.elements[self.needle];
            self.needle += 1;
            Some(t)
        } else {
            None
        }
    }

    pub fn try_<R, S, F>(&mut self, f: F) -> Result<(usize, R), S>
    where F: FnOnce(&mut TryReader<T>) -> Result<R, S> {
        let mut clone = TryReader {
            elements: self.elements,
            needle: self.needle
        };
        match f(&mut clone).map(|result| (clone.needle - self.needle, result)) {
            Ok(result) => {
                self.needle = clone.needle;
                Ok(result)
            },
            Err(err) => Err(err),
        }
    }

    pub fn try_next<R, S, F>(&mut self, f: F) -> Result<R, S>
    where F: FnOnce(&T) -> Result<R, S> {
        match f(&self.elements[self.needle]) {
            Ok(r) => {
                self.next();
                Ok(r)
            },
            Err(s) => Err(s),
        }
    }

    pub fn has_next(&self) -> bool {
        self.elements.len() != self.needle
    }
}