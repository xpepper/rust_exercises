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
    use super::{median, mode};

    #[test]
    fn compute_median_of_a_list_of_numbers() {
        assert_eq!(median(&[1, 2, 3, 4, 5]), 3.0);
        assert_eq!(median(&[1, 2, 3, 4, 5]), 3.0);
        assert_eq!(median(&[1, 2, 3, 4, 5, 6]), 3.5);
    }

    #[test]
    fn compute_mode_of_a_list_of_numbers() {
        assert_eq!(mode(&[]), None);
        assert_eq!(mode(&[1, 2, 3, 4, 5]), None);
        assert_eq!(mode(&[1, 2, 3, 4, 5, 5]), Some(5));
        assert_eq!(mode(&[1, 2, 2, 2, 3, 4, 5, 5]), Some(2));
    }
}
