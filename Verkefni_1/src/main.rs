use rand::{self, Rng};
use std::io::{self, Write};
fn main() {
    //byrjun kóðans sem er með öll gögn sem  við notum 
    //power er fyrir whiel loopið
    let mut power = true;
    //þetta bæði eru listar sem munu vera nottaðir til að halda random tölum
    let mut rant_num_list: [[i32; 10]; 10]   = [[0; 10]; 10];
    let mut rant_count_list:[i32;10] = [0;10];
    //þetta  for loops seta öll gögn in á listana
    for i in 0..10{
        for l in 0..10{
            //búir til random tölu milli 0-9
           let rng_num = rand::thread_rng().gen_range(0..10);
           rant_num_list[i][l] = rng_num as i32;
           rant_count_list[rng_num] += 1 as i32;
    }}
    //bara lengdin á listonum til að nota fyrir for loops
    let leangth_count = rant_count_list.len();
    let leangth_num = rant_count_list.len();
    //while val
    while power == true{
        println!("1: Sjá allar 100 tölur");
        println!("2: Sjá fjölda af hverjúm tölustaf");
        println!("3: Sjá töluna sem kom upp oftast");
        println!("4: Sjá töluna sem kom upp sjaldnast");
        println!("5: Hætta");
        print!("Enter: ");
        //þetta alt fyrir neða lætur það svo input er fyrir framan enter og er notað Write fyrir það
        io::stdout().flush().unwrap();
        let mut svar =String::new();
        std::io::stdin().read_line(&mut svar).unwrap();
        let svar  = svar.trim();
        //liður 1
        if svar == "1"{
            //printar 10 stafi í eins markar línur og það eru í listanum
            for i in 0..leangth_num{
                for l in 0..10{
                print!("{} ",rant_num_list[i][l])
            }
            println!("")
        }
            println!("")
        }
        //liður 2
        else if svar == "2"{
            //prentar út gögnin ú rant_count_list til að væri bettra að lesa
            for i in 0..leangth_count{
                println!("{i} er {} mörk",rant_count_list[i])
            }
        }
        //liður 3
        else if svar == "3"{
            //þetta sko'ar hvaða tala er mest af og með því vinur kepnina
            let mut winner = 0;
            let mut winning = 0;
            for i in 0..leangth_count{
                println!("{}",rant_count_list[i]);
                if rant_count_list[i] > winning{
                    winning = rant_count_list[i];
                    winner = i;
                }
            }
            //prenta út sá sem van
            println!("{winner} er með mestu tölurnar sem eru {winning}");
        }
        //liður 4
        else if svar == "4"{
            //gerir það sama og liður 3 bara leita af minni töluni
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
        //liður 5(samt ekki var got að affa það fyrir alla if functions)
        else if svar == "5"{
            //hættir while loop
            power = false;
        }
        else {
        //ef það er skrifað eithvað vitlaus kemur þetta
            println!("skrifaðir vitlaust. skrifaðu 1,2,3,4,5");
        }
    }
}
