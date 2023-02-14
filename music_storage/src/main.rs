use std::fs::write;
use std::fs::File;
fn main() {
    let music_sheet = File::create("Music_Sheet.txt").expect("failed to create file");
    let data1 = "this is a string in a text file";
    let path = "/home/niels/RustProjects/music_storage/Music_Sheet.txt";
    write(path, data1).expect("unable to write to file");
    loop {
        println!("Hello! Please choose your desired operation from the list below! ");
        println!("Add Songs, Read Songs, Quit ");
        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("error reading input ");

        if answer.trim().to_lowercase() == "add songs" {
            writemusic();
        } else {
            println!("Goodbye!");
            break;
        }
    }
}

fn writemusic() {
    println!("Enter the name of the Song you want to save: ");
    let path = "/home/niels/RustProjects/music_storage/Music_Sheet.txt";
    let mut music_to_add = String::new();
    std::io::stdin().read_line(&mut music_to_add).expect("Error reading info");
    let music_push = music_to_add.trim().to_lowercase();
    write(path, music_to_add).expect("failed to write to file");
}
fn readmusic() {
    println!("hello world from read!");
}
