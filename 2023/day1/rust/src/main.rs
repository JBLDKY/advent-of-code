use std::fs::read_to_string;

fn main() {
    let string: String = read_to_string("input.txt").unwrap();

    let solution: Vec<u32> = string
        .lines()
        .map(|x| {
            let tmp = x
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "one8one")
                .replace("nine", "nine9nine")
                .chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<u32>>();

            format!(
                "{}{}",
                tmp.first().unwrap().clone(),
                tmp.last().unwrap().clone()
            )
            .parse()
            .unwrap()
        })
        .collect();

    dbg!(&solution.iter().sum::<u32>());
}
