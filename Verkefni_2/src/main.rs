/*
* struct fyrir stærð sem inniheldur lengd og breidd (f32).
* enum fyrir tegund sem getur verið Kennslustofa, Skrifstofa eða Annað.
* enum fyrir örgjörvagerð sem getur verið Cisc eða Risc
* struct fyrir tölvu sem inniheldur stærð disks og vinnsluminnis (u32) ásamt örgjörvagerð.
* struct fyrir herbergi sem inniheldur stærð, tegund og tölvu eða ekki (Option).
* herbergi þarf að eiga fallið breyta_staerd sem setur inn nýjar tölur fyrir lengd og
*/
mod stofa_info;
mod lengt;
mod tolva;
mod tegund;

use tegund::Tegund;
use tolva::{Tolva, CPU};
use stofa_info::Herbergi;

fn main() {
    let mut h202 = Herbergi::new(4.3, 3.7, Tegund::Kennslustofa, None);
    println!("{}", h202);
    // Stærð: 15.91 fm., tegund: Tegund: Kennslustofa, Tölva: Engin tölva
    h202.breyta_staerd(6.1, 5.3);
    h202.skipta_um_tolvu(Some(Tolva::new(1000, 16, CPU::Risc)));
    println!("{}", h202);
    // Stærð: 32.33 fm., tegund: Tegund: Kennslustofa, Tölva: HDD: 1000 GB, RAM 16 GB, CPU: RISC
}

