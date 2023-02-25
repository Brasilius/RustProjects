use std::io;

fn main(){
println!("do you want to run the program(y/n)");
let mut input = String::new();
let mut market_demand = 1;
io::stdin().read_line(&mut input).expect("failed to read line");
match input.trim().to_lowercase().as_str() { 
"y" | "yes" => {
    //run the program under here
    println!("running the program!");
    let mut input = String::new();
    println!("please enter the market demand: ");
    io::stdin().read_line(&mut input).expect("failed to read integer value");
    let market_demand: i32 = input.trim().parse().expect("failed to parse integer value");
    market_array(market_demand);
    },
"n" | "no" => {
    //exit the program under here
    println!("exiting the program!");
    },
_ => {
    //if the user inputs something other than y or n
    println!("invalid input!");
     main();
    }

 }
}
pub fn market_array(market_demand:i32) -> i32 {
    let market_list: [i32; 6] = [9,2,5,6,1,8];
    let index = market_list.iter().position(|&r| r == market_demand).unwrap();
    println!("successful run!");
    return market_list[index+market_list.len()]
}
