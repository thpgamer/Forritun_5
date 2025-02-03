#[derive(Debug)]
//classic enum svo hægt er að fá annað hvort CPU
pub enum CPU {
    Cisc,
    Risc,
}

#[derive(Debug)]
//kemur með öll variabuls sem við munum nota og setur á það public
pub struct Tolva {
    pub diskur: u32,
    pub ram: u32,
    pub cpu: CPU,
}
//gevur self value svo hægt er að nota það
impl Tolva {
    pub fn new(diskur: u32, ram: u32, cpu: CPU) -> Self {
        Self { diskur, ram, cpu }
    }
}
