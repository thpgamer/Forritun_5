
fn main() {
    let mut name =String::new();
    println!("please enter your name: ");
    std::io::stdin().read_line(&mut name).unwrap();
    let on = true;
    let  lenght = name.chars().count();
    while on == true{
        for i in 0..lenght {
            let letter = name.chars().nth(i).unwrap();
            println!("{}",letter)
        }
    }
}
