#[derive(Debug)]
//gerir allar tegundir með enum very basic. Very Nice
pub enum Tegund {
    Kennslustofa,
    Skrifstofa,
    Annad,
}

impl Tegund {
    pub fn from(s: &str) -> Self {
        match s {
            "Kennslustofa" => Tegund::Kennslustofa,
            "Skrifstofa" => Tegund::Skrifstofa,
            _ => Tegund::Annad,
        }
    }
}
