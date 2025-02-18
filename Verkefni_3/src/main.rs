mod flugfel;
mod flug;

use std::io::Write;

use flugfel::Flugvelar;

fn main() {
    loop {
        print!("Enter: ");
        std::io::stdout().flush().expect("flush náðist ekki");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("náði ekki að lesa");
        let svor: Vec<&str> = input.split_ascii_whitespace().collect();
        match svor.first() {
            Some(svar) =>{
                let low_svar = svar.to_lowercase();
                let final_svar = low_svar.trim();
                match final_svar {
                    "hætta" => break,
                    "hjálp" | "h" => {
                        println!("new 'name' 'speed' 'leingt frá turn'");
                        
                        println!("hætta eða h");
                    },
                    "new" =>{
                        println!("{}",input);
                        let new_list: Vec<&str> = input.split(" ").collect();
                        let nafn = new_list[1];
                        let hradi = new_list[2].parse::<u32>();
                        let leingt = new_list[3].parse::<i32>();
                        match (hradi, leingt) {
                            (Ok(hradi),Ok(leignt)) =>{

                            }
                            (Err(e),)=> println!("{}",e)
                            
                        match flugfel::Flugvelar::new(nafn,hradi,leingt) {
                            
                        }
                    }
                    _ => println!("veit ekki hvað þú sagðir"),
        
                }
            }
            None => println!("Sláðu eithvað inn ekki vera leiðinlegur"),
    }
}
}
