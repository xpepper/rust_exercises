mod median_and_mode;

use crate::median_and_mode::{median, mode};

fn main() {
    let numbers = random_list_of_integers(10000, 99);
    println!("Numbers are: {:?}", numbers);

    let median = median(&numbers);
    println!("Median is: {}", median);

    match mode(&numbers) {
        Some(mode) => println!("Mode is: {}", mode),
        None => println!("No mode found"),
    }
}

fn random_list_of_integers(how_many: i32, max_value: u32) -> Vec<i32> {
    let mut numbers = Vec::new();
    for _ in 0..how_many {
        numbers.push((rand::random::<u32>() % (max_value + 1)) as i32);
    }
    numbers
}

#[cfg(test)]
mod tests {
    use super::random_list_of_integers;

    #[test]
    fn test_random_list_of_integers() {
        let numbers = random_list_of_integers(100, 99);
        assert_eq!(numbers.len(), 100);
        assert!(numbers.iter().all(|&number| number >= 0 && number <= 99));
    }
}
