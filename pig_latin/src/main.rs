use crate::pig_latin::pig_latin;

mod pig_latin;

fn main() {
    println!("{}", pig_latin("apple"));
    println!("{}", pig_latin("coração"));
    println!("{}", pig_latin("virtù"));
    println!("{}", pig_latin("è"));
    println!("{}", pig_latin("こんにちは"));
}



