pub struct NgramIterator<I: Iterator> {
    iter: I,
    n: usize,
    window: Vec<I::Item>,
}

impl<I: Iterator> NgramIterator<I> {
    fn new(mut iter: I, n: usize) -> Self {
        let mut window = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(item) = iter.next() {
                window.push(item);
            } else {
                break;
            }
        }
        NgramIterator { iter, n, window }
    }
}

impl<I> Iterator for NgramIterator<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.window.len() < self.n {
            return None;
        }
        let result = self.window.clone();
        self.window.remove(0);
        if let Some(next_item) = self.iter.next() {
            self.window.push(next_item);
        }
        Some(result)
    }
}

pub trait Ngram: Iterator {
    fn ngrams(self, n: usize) -> NgramIterator<Self>
    where
        Self: Sized,
    {
        NgramIterator::new(self, n)
    }
}

impl<I: Iterator> Ngram for I {}

#[derive(Debug)]
pub struct TokinizedChars<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> TokinizedChars<'a> {
    pub fn new(text: &'a str) -> Self {
        TokinizedChars {
            chars: text.chars(),
        }
    }
}

impl<'a> From<&'a str> for TokinizedChars<'a> {
    fn from(text: &'a str) -> Self {
        TokinizedChars::new(text)
    }
}

impl<'a> Iterator for TokinizedChars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }
}

mod tests {
    use super::*;

    #[allow(dead_code)]
    fn ngrams(n: usize, text: &str) -> Vec<String> {
        TokinizedChars::new(text)
            .ngrams(n)
            .map(|ngram| ngram.iter().collect())
            .collect()
    }

    #[test]
    fn test_ngram_ascii() {
        let text = "hello";
        let n = 2;
        let expected = vec!["he", "el", "ll", "lo"];
        assert_eq!(ngrams(n, text), expected);
    }

    #[test]
    fn test_ngram_japanese() {
        let text = "こんにちは日本語";
        let n = 2;
        let expected = vec!["こん", "んに", "にち", "ちは", "は日", "日本", "本語"];
        assert_eq!(ngrams(n, text), expected);
    }
}
