use std::fs::read_to_string;

#[derive(Debug)]
struct Client {
    lines: Vec<Line>,
}

impl Client {
    fn new(lines: Vec<&str>) -> Client {
        let lines = lines
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, line)| Line::new(idx, line.into()))
            .collect();

        Client { lines }
    }

    fn stars(&self) -> Vec<Symbol> {
        return self
            .lines
            .iter()
            .flat_map(|line| line.symbols.clone())
            .filter(|symbol| symbol.value == '*')
            .collect::<Vec<Symbol>>();
    }

    fn symbols(&self) -> Vec<Symbol> {
        return self
            .lines
            .iter()
            .flat_map(|line| line.symbols.clone())
            .collect::<Vec<Symbol>>();
    }

    fn numbers(&self) -> Vec<Number> {
        return self
            .lines
            .iter()
            .flat_map(|line| line.numbers.clone())
            .collect::<Vec<Number>>();
    }
}

#[derive(Debug, Clone)]
struct Line {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}
impl Line {
    fn new(index: usize, line: String) -> Line {
        let split_lines = line.split("");

        let res: Vec<(usize, usize)> = split_lines
            .clone()
            .enumerate()
            .filter_map(|(idx, val)| match val.parse::<usize>().ok() {
                Some(val) => Some((idx, val)),
                None => None,
            })
            .collect();

        let mut prev: Option<usize> = None;
        let mut subsequents: Vec<Vec<(usize, usize)>> = vec![];
        let mut new: Vec<(usize, usize)> = vec![];

        for (idx, val) in res.clone() {
            if prev.is_none() {
                new.push((idx, val));
                prev = Some(idx);
                continue;
            }

            if prev.unwrap() != idx - 1 {
                subsequents.push(new.clone());
                new = vec![];
            };

            new.push((idx, val));
            prev = Some(idx);
            if (idx, val) == *res.clone().last().unwrap() {
                subsequents.push(new.clone());
            }
        }

        let numbers = subsequents
            .iter()
            .filter_map(|pair| Number::new(index, pair))
            .collect();

        let symbols = split_lines
            .enumerate()
            .filter_map(|(idx, item)| {
                if item.is_empty() {
                    return None;
                }
                let character = item.chars().last().unwrap();
                if character == '.' || character.is_digit(10) {
                    return None;
                }

                Symbol::new(index, (idx, character))
            })
            .collect();

        Line { numbers, symbols }
    }
}

#[derive(Debug, Default, Clone)]
struct Number {
    y: usize,
    value: Vec<usize>,
    index: Vec<usize>,
}
impl Number {
    fn new(y: usize, pair: &Vec<(usize, usize)>) -> Option<Number> {
        let value: Vec<usize> = pair.iter().map(|(_, v)| v.clone()).collect();
        let index: Vec<usize> = pair.iter().map(|(i, _)| i.clone()).collect();

        Some(Number { y, value, index })
    }
    fn position(&self) -> (Vec<usize>, Vec<usize>) {
        let mut adjacent_x = self.index.clone();
        let left = match self.index.clone().first().unwrap() {
            0 => 0,
            v => v - 1,
        };
        let right = match self.index.clone().last().unwrap() {
            139 => 139,
            v => v + 1,
        };
        adjacent_x.insert(0, left);
        adjacent_x.push(right);

        let y_min = if self.y == 0 { 0 } else { self.y - 1 };
        let origin = vec![*adjacent_x.first().unwrap(), y_min];
        let end = vec![*adjacent_x.last().unwrap(), self.y + 1];

        return (origin, end);
    }
}

#[derive(Debug, Default, Clone)]
struct Symbol {
    y: usize,
    value: char,
    index: usize,
}
impl Symbol {
    fn new(y: usize, pair: (usize, char)) -> Option<Symbol> {
        let (index, value) = pair;
        Some(Symbol { value, y, index })
    }

    fn position(&self) -> (usize, usize) {
        return (self.index, self.y);
    }

    fn collisions(&self, numbers: Vec<Number>) -> Vec<usize> {
        let mut nums: Vec<usize> = vec![];
        for num in numbers {
            let (num_origin, num_end) = num.position();
            let (num_min_x, num_min_y) = (num_origin.first().unwrap(), num_origin.last().unwrap());
            let (num_max_x, num_max_y) = (num_end.first().unwrap(), num_end.last().unwrap());

            let (sym_x, sym_y) = self.position();
            if sym_x >= *num_min_x
                && sym_x <= *num_max_x
                && sym_y >= *num_min_y
                && sym_y <= *num_max_y
            {
                let r: Vec<String> = num.value.iter().map(|d| d.to_string()).collect();

                nums.push(r.join("").parse::<usize>().unwrap());
            }
        }

        nums
    }
}

fn solution_one() {
    let string: String = read_to_string("input.txt").expect("couldnt read file");
    let split: Vec<&str> = string.split("\n").filter(|x| !x.is_empty()).collect();
    let client = Client::new(split);

    let symbols = &client.symbols();
    let numbers = &client.numbers();

    let mut res: Vec<usize> = vec![];

    for sym in symbols {
        let z = sym.collisions(numbers.to_vec());
        res.extend(z.iter());
    }
    dbg!(res.iter().sum::<usize>());
}

fn solution_two() {
    let string: String = read_to_string("input.txt").expect("couldnt read file");
    let split: Vec<&str> = string.split("\n").filter(|x| !x.is_empty()).collect();
    let client = Client::new(split);

    let stars = client.stars();
    let numbers = &client.numbers();

    let mut res: Vec<usize> = vec![];

    for sym in stars {
        let z = sym.collisions(numbers.to_vec());
        if z.len() < 2 {
            continue;
        }

        let x = z.clone().iter().product::<usize>();
        res.push(x);
    }
    dbg!(res.iter().sum::<usize>());
}
fn main() {
    solution_one();
    solution_two();
}
