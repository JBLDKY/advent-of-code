use std::cmp::max;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Color {
    type Err = ParseError;

    fn from_str(color: &str) -> Result<Self, Self::Err> {
        match color {
            "green" => Ok(Color::Green),
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            _ => panic!("Unknown color."),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn new(line: &str) -> Game {
        Game {
            id: Game::get_id(&line),
            hands: Hand::from_line(&line),
        }
    }

    /// Checks if a game has all valid hands against a bag of maximum cubes
    fn is_valid(&self, bag: &HashMap<Color, u32>) -> bool {
        let total = self.max();

        for (color, amount) in total.cubes.iter() {
            let limit = bag.get(color).expect("I suck at coding");

            if limit < amount {
                return false;
            }
        }
        return true;
    }

    // Returns a new hand that holds the maximum cubes
    fn max(&self) -> Hand {
        let mut total: HashMap<Color, u32> =
            HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]);

        self.hands.iter().for_each(|hand| {
            hand.cubes.iter().for_each(|(color, amount)| {
                total
                    .entry(color.clone())
                    .and_modify(|count| *count = max(*count, *amount));
            });
        });

        Hand { cubes: total }
    }

    /// Create a game id that we need to count for the solution
    fn get_id(line: &str) -> u32 {
        line.split(":")
            .collect::<Vec<&str>>()
            .first()
            .expect("Couldnt find Game x: on this line.")
            .split(" ")
            .last()
            .expect("Couldnt find a number on this line.")
            .parse()
            .expect("ID is not a number")
    }
}

#[derive(Debug, Default)]
struct Hand {
    cubes: HashMap<Color, u32>,
}

impl Hand {
    fn from_line(line: &str) -> Vec<Hand> {
        let hands = line
            .split(":")
            .last()
            .expect("Couldnt find hands on this line.")
            .split(";")
            .collect::<Vec<&str>>();

        let mut res = vec![];

        for hand in &hands {
            let cubes = Hand::color_amount(hand);
            res.push(Hand { cubes });
        }
        res
    }

    /// Counts the colored cubes from a single hand
    fn color_amount(string: &str) -> HashMap<Color, u32> {
        string
            .split(",")
            .map(|pair| pair.trim())
            .map(|pair| pair.split(" ").collect())
            .map(|pair: Vec<&str>| {
                (
                    Color::from_str(pair.last().unwrap()).unwrap(),
                    pair.first().unwrap().parse().unwrap(),
                )
            })
            .collect()
    }
}

fn main() {
    let string: String = read_to_string("input.txt").expect("couldnt read file");
    let split: Vec<&str> = string.split("\n").filter(|x| !x.is_empty()).collect();
    let games: Vec<Game> = split.iter().map(|x| Game::new(x)).collect();

    let bag: HashMap<Color, u32> =
        HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    let solution_part_one = games
        .iter()
        .filter(|game| game.is_valid(&bag))
        .map(|game| game.id)
        .sum::<u32>();

    dbg!(&solution_part_one);

    let product_per_hand: Vec<u32> = games
        .iter()
        .map(|game| game.max())
        .map(|hand| {
            hand.cubes
                .into_values()
                .collect::<Vec<u32>>()
                .iter()
                .product()
        })
        .collect();

    dbg!(&product_per_hand.iter().sum::<u32>());
}
