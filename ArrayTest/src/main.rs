use std::io;
fn main() {
    println!("Input your index value: ");
    let x = [1,2,3,4,5,6];
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("shut up bitch!");
    let input: usize = input.trim().parse().expect("that shit aint a number");
    let y = input - 1;
    let element = x[y];
    println!("your mf value at index {input} is {element}");
}