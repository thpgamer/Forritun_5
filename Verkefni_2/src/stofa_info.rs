use crate::tegund::Tegund;
use crate::tolva::{Tolva, CPU};
use crate::lengt::Length;
#[derive(Debug)]
//kemur með öll variabuls sem við munum nota og setur á það public
pub struct Herbergi {
    pub stofa_length: Length,
    pub tegund: Tegund,
    pub talva: Option<Tolva>,
}
//definar value til self svo hægt er að nota þau
impl Herbergi {
    pub fn new(l: f32, b: f32, tegund: Tegund, talva: Option<Tolva>) -> Self {
        Self {
            stofa_length: Length::new(l, b),
            tegund,
            talva,
        }
    }
    //setir up nýtt value fyrir length með því að yfir skrifa það
    pub fn breyta_staerd(&mut self, leingt: f32, breit: f32) {
        self.stofa_length = Length::new(leingt, breit);
    }
    //gerir það sama og fyrir ofan bara fyrir tölvu
    pub fn skipta_um_tolvu(&mut self, ny_tolva: Option<Tolva>) {
        self.talva = ny_tolva;
    }
}
//sínir allt sem er búið að gera
impl std::fmt::Display for Herbergi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let talva_str = if let Some(talva) = &self.talva {
            format!(
                "HDD: {} GB, RAM {} GB, CPU: {:?}",
                talva.diskur, talva.ram, talva.cpu
            )
        } else {
            "Engin tölva".to_string()
        };

        write!(
            f,
            "Stærð: {:.2} fm., Tegund: {:?}, Tölva: {}",
            self.stofa_length.area(),
            self.tegund,
            talva_str
        )
    }
}
