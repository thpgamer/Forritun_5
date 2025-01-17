use rand::{self, Rng};
use std::io::{self, Write};
fn main() {
    let mut power = true;
    let mut rant_num_list: [[i32; 10]; 10]   = [[0; 10]; 10];
    let mut rant_count_list:[i32;10] = [0;10];
    for i in 0..10{
        for l in 0..10{
           let rng_num = rand::thread_rng().gen_range(0..10);
           rant_num_list[i][l] = rng_num as i32;
           rant_count_list[rng_num] += 1 as i32;
    }}
    let leangth_count = rant_count_list.len();
    let leangth_num = rant_count_list.len();
    while power == true{
        println!("1: Sjá allar 100 tölur");
        println!("2: Sjá fjölda af hverjúm tölustaf");
        println!("3: Sjá töluna sem kom upp oftast");
        println!("4: Sjá töluna sem kom upp sjaldnast");
        println!("5: Hætta");
        print!("Enter: ");
        io::stdout().flush().unwrap();
        let mut svar =String::new();
        std::io::stdin().read_line(&mut svar).unwrap();
        let svar  = svar.trim();
        if svar == "1"{
            for i in 0..leangth_num{
                for l in 0..10{
                print!("{} ",rant_num_list[i][l])
            }
            println!("")
        }
            println!("")
        }
        
        else if svar == "2"{
            for i in 0..leangth_count{
                println!("{i} er {} mörk",rant_count_list[i])
            }
        }
        
        else if svar == "3"{
            let mut winner = 0;
            let mut winning = 0;
            for i in 0..leangth_count{
                println!("{}",rant_count_list[i]);
                if rant_count_list[i] > winning{
                    winning = rant_count_list[i];
                    winner = i;
                }
            }
            println!("{winner} er með mestu tölurnar sem eru {winning}");
        }
        
        else if svar == "4"{
            let mut winner = 0;
            let mut winning = 100;
            for i in 0..leangth_count{
                println!("{}",rant_count_list[i]);
                if rant_count_list[i] < winning{
                    winning = rant_count_list[i];
                    winner = i;
                }
            }
            println!("{winner} er með minstu tölurnar sem eru {winning}");
        }
        
        else if svar == "5"{
            power = false;
        }
        else {
            println!("skrifaðir vitlaust. skrifaðu 1,2,3,4,5");
        }
    }
}
