use std::collections::HashMap;

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

fn median(numbers: &[i32]) -> f32 {
    let sorted = sort(numbers);

    let size = sorted.len();
    if size % 2 == 1 {
        sorted[size / 2] as f32
    } else {
        (sorted[size / 2 - 1] + sorted[size / 2]) as f32 / 2.0
    }
}

fn mode(numbers: &[i32]) -> Option<i32> {
    let frequencies = compute_frequencies(numbers);
    let max_frequency = frequencies.values().max().copied().unwrap_or(0);
    let modes: Vec<_> = frequencies
        .iter()
        .filter(|&(_, &frequency)| frequency == max_frequency)
        .collect();

    if modes.len() == 1 {
        Some(*modes[0].0)
    } else {
        None
    }
}

fn compute_frequencies(numbers: &[i32]) -> HashMap<i32, i32> {
    let mut frequencies = HashMap::new();
    numbers.iter().for_each(|&number| {
        *frequencies.entry(number).or_insert(0) += 1;
    });
    frequencies
}

fn sort(numbers: &[i32]) -> Vec<i32> {
    let mut n = numbers.to_vec();
    n.sort_unstable();
    n
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
    use super::*;

    #[test]
    fn test_median() {
        assert_eq!(median(&[1, 2, 3, 4, 5]), 3.0);
        assert_eq!(median(&[1, 2, 3, 4, 5, 6]), 3.5);
    }

    #[test]
    fn test_mode() {
        assert_eq!(mode(&[1, 2, 3, 4, 5]), None);
        assert_eq!(mode(&[1, 2, 3, 4, 5, 5]), Some(5));
        assert_eq!(mode(&[1, 2, 3, 4, 5, 5, 5]), Some(5));
    }

    #[test]
    fn test_compute_frequencies() {
        let frequencies = compute_frequencies(&[1, 2, 2, 3, 4, 5, 5, 5]);
        assert_eq!(frequencies[&1], 1);
        assert_eq!(frequencies[&2], 2);
        assert_eq!(frequencies[&3], 1);
        assert_eq!(frequencies[&4], 1);
        assert_eq!(frequencies[&5], 3);
    }

    #[test]
    fn test_sort() {
        assert_eq!(sort(&[5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_list_of_integers() {
        let numbers = random_list_of_integers(100, 99);
        assert_eq!(numbers.len(), 100);
        assert!(numbers.iter().all(|&number| number >= 0 && number <= 99));
    }    
}