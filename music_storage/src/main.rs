use std::fs::File;
use std::fs::write;
fn main() {

let music_sheet = File::create("Music_Sheet.txt").expect("failed to create file");
let data1 = "this is a string in a text file";
let path = "/home/niels/RustProjects/music_storage/Music_Sheet.txt";
write(path, data1).expect("unable to write to file");




}
   
   fn writemusic(){

    println!("hello world!");

   }
   fn readmusic(){
    println!("hello world from read!");
   }