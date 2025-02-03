mod hestur;
mod stada;

use hestur::Hestur;

fn main() {
    let mut sleipnir = hestur::new(100, "Gráni", 15, "laus");
    println!("{}", sleipnir);
    // id: 100, nafn: Gráni, aldur: 15, staða: Laus
    sleipnir.breyta_stodu("leigður");
    println!("{}", sleipnir);
    // id: 100, nafn: Gráni, aldur: 15, staða: Leigður

    sleipnir.breyta_stodu("ekki til útleigu");
    println!("{}", sleipnir);
    // id: 100, nafn: Gráni, aldur: 15, staða: Ekki til útleigu

    sleipnir.breyta_stodu("eitthvað");
    println!("{}", sleipnir);
    // id: 100, nafn: Gráni, aldur: 15, staða: Óþekkt
}