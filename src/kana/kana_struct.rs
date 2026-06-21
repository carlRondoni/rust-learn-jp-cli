#[derive(Debug, Clone)]
pub struct Kana {
    pub romaji: Vec<&'static str>,
    pub written: &'static str,
    pub character: &'static str,
}

impl Kana {
    pub fn new<I>(romaji: I, written: &'static str, character: &'static str) -> Self
    where
        I: IntoIterator<Item = &'static str>,
    {
        Self {
            romaji: romaji.into_iter().collect(),
            written,
            character,
        }
    }
}
