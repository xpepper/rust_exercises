use std::collections::HashMap;

pub(crate) fn median(numbers: &[i32]) -> f32 {
    let sorted = sort(numbers);

    let size = sorted.len();
    if size % 2 == 1 {
        sorted[size / 2] as f32
    } else {
        (sorted[size / 2 - 1] + sorted[size / 2]) as f32 / 2.0
    }
}

pub(crate) fn mode(numbers: &[i32]) -> Option<i32> {
    let frequencies = compute_frequencies(numbers);
    let max_frequency = frequencies.values().max().copied()?;

    let mut modes = frequencies
        .iter()
        .filter(|&(_, &frequency)| frequency == max_frequency)
        .map(|(&number, _)| number);

    let first_mode = modes.next()?;
    if modes.next().is_none() {
        Some(first_mode)
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

#[cfg(test)]
mod tests {
    use crate::random_list_of_integers;
    use super::*;

    #[test]
    fn test_median() {
        assert_eq!(median(&[1, 2, 3, 4, 5]), 3.0);
        assert_eq!(median(&[1, 2, 3, 4, 5]), 3.0);
        assert_eq!(median(&[1, 2, 3, 4, 5, 6]), 3.5);
    }

    #[test]
    fn test_mode() {
        assert_eq!(mode(&[]), None);
        assert_eq!(mode(&[1, 2, 3, 4, 5]), None);
        assert_eq!(mode(&[1, 2, 3, 4, 5, 5]), Some(5));
        assert_eq!(mode(&[1, 2, 2, 2, 3, 4, 5, 5]), Some(2));
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
