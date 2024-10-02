// const INPUT: &str = include_str!("test_data/plane-pts.txt");

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use url::quirks::username;

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}

mod my_reader_alt {
    use std::{
        fs::File,
        io::{self, prelude::*},
        rc::Rc,
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
        buf: Rc<String>,
    }

    fn new_buf() -> Rc<String> {
        Rc::new(String::with_capacity(1024)) // Tweakable capacity
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            let buf = new_buf();

            Ok(Self { reader, buf })
        }
    }

    impl Iterator for BufReader {
        type Item = io::Result<Rc<String>>;

        fn next(&mut self) -> Option<Self::Item> {
            let buf = match Rc::get_mut(&mut self.buf) {
                Some(buf) => {
                    buf.clear();
                    buf
                }
                None => {
                    self.buf = new_buf();
                    Rc::make_mut(&mut self.buf)
                }
            };

            self.reader
                .read_line(buf)
                .map(|u| {
                    if u == 0 {
                        None
                    } else {
                        Some(Rc::clone(&self.buf))
                    }
                })
                .transpose()
        }
    }
}

#[test]
fn test_ascii_playback_naive() {
    let file = File::open("./tests/test_data/plane-pts.txt").unwrap();
    let reader = io::BufReader::new(file);

    // reader.lines().map(|line| {
    //     let value = line.unwrap();
    //     match value.parse::<i64>() {
    //         Ok(pts) => {}
    //         Err(_) => {}
    //     }
    // });

    #[derive(Default)]
    struct Frame {
        pts: u64,
        data: Vec<String>,
    }

    let mut frames: Vec<Frame> = Vec::new();
    // let mut frame = Frame::default();

    let mut buffer = Vec::new();

    for line in reader.lines() {
        if let Ok(entry) = line {
            if !entry.starts_with("END") {
                match entry.parse::<u64>() {
                    Ok(pts) => {
                        frames.push(Frame {
                            pts,
                            data: std::mem::take(&mut buffer),
                        });
                    }
                    Err(_) => {
                        buffer.push(entry);
                    }
                }
            }
        }
    }

    let mut last: u64 = 0;
    for f in frames {
        // print!("{esc}c", esc = 27 as char);
        // std::process::Command::new("clear").status().unwrap();
        print!("\x1Bc");
        for line in f.data {
            println!("{}", line);
        }
        std::thread::sleep(std::time::Duration::from_millis(f.pts - last));
        last = f.pts;
    }
}

const TEXT_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn process_line(line: &str, with_text: bool) -> (i32, i32) {
    let mut first_digit = -1;
    let mut last_digit = -1;

    for (i, ch) in line.chars().enumerate() {
        if ch.is_digit(10) {
            last_digit = ch.to_digit(10).unwrap() as i32;
        } else {
            if !with_text {
                continue;
            }
            for (index, text_digit) in TEXT_DIGITS.iter().enumerate() {
                if line[i..].starts_with(text_digit) {
                    last_digit = index as i32 + 1;
                }
            }
        }
        if first_digit == -1 {
            first_digit = last_digit;
        }
    }

    (first_digit, last_digit)
}

#[test]
fn test_aoc_2023_day_01() {
    let file = File::open("./tests/test_data/AOC_2023_day_01.txt").unwrap();
    let buf_read = io::BufReader::new(file);

    let mut result = 0;
    for line in buf_read.lines().map_while(Result::ok) {
        let (first_digit, last_digit) = process_line(&line, false);
        result += first_digit * 10 + last_digit;
    }
    println!("{}", result);
}

#[test]
fn test_aoc_2023_day_01_alt() {
    let file = File::open("./tests/test_data/AOC_2023_day_01.txt").unwrap();
    let buf_read = io::BufReader::new(file);

    let ans = buf_read
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut char = line.chars().filter(|c| c.is_digit(10));

            // if there is no last, then double the first char
            let first = char.next().unwrap();
            let num = match char.last() {
                Some(last) => {
                    format!("{}{}", first, last)
                }
                None => {
                    format!("{}{}", first, first)
                }
            };
            num.parse::<u32>().unwrap()
        })
        .sum::<u32>();
    println!("Answer: {}", ans);
    assert_eq!(55029, ans);
}

#[derive(Debug, PartialEq)]
struct NumberPosition {
    number: u32,
    position: usize,
}

/// Gets the position of the digits in the input string
/// ## Examples
/// - "1a2b3c" -> NumberPosition { number: 1, position: 0 }, NumberPosition { number: 2, position: 2 }, NumberPosition { number: 3, position: 4 }
fn find_digits(input: &str) -> Vec<NumberPosition> {
    input
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_digit(10))
        .map(|(i, c)| NumberPosition {
            number: c.to_digit(10).unwrap(),
            position: i,
        })
        .collect()
}

/// Gets the position of the first character of spelled numbers in the input string
/// ## Examples
/// - "onediaosd" -> NumberPosition { number: 1, position: 0 }
/// - "aidotwo" -> NumberPosition { number: 2, position: 4 }
fn find_spelled_numbers(input: &str) -> Vec<NumberPosition> {
    let spelled_numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut positions: Vec<NumberPosition> = Vec::new();
    for spelled_number in spelled_numbers.clone() {
        let mut start = 0;
        while let Some(position) = input[start..].find(spelled_number) {
            positions.push(NumberPosition {
                number: spelled_numbers
                    .iter()
                    .position(|&n| n == spelled_number)
                    .unwrap() as u32
                    + 1,
                position: start + position,
            });
            start += position + spelled_number.len();
        }
    }

    // sort the positions by position
    positions.sort_by_key(|np| np.position);
    positions
}

fn get_numbers_from_line(line: &str) -> u32 {
    // find all the numbers in the input
    // get only the first NumberPosition and last by position
    // sum the numbers
    // print the sum
    let digits = find_digits(line);
    let spelled_numbers = find_spelled_numbers(line);
    let mut numbers: Vec<NumberPosition> = digits;
    numbers.extend(spelled_numbers);
    numbers.sort_by_key(|np| np.position);

    // get the first and last number
    let first = numbers
        .first()
        .expect("there should be at least one number");
    let last = numbers.last().expect("there should be at least one number");
    let answer = first.number * 10 + last.number;

    println!("{} -> {:?} + {:?} = {}", line, first, last, answer);

    // sum the numbers
    answer
}

#[test]
fn test_aoc_2023_day_01_part_2() {
    let file = File::open("./tests/test_data/AOC_2023_day_01.txt").unwrap();
    let buf_read = io::BufReader::new(file);

    let ans = buf_read
        .lines()
        .map_while(Result::ok)
        .map(|line| get_numbers_from_line(&line))
        .sum::<u32>();
    println!("Answer: {}", ans);
}

type Pull = [usize; 3];

#[derive(Debug)]
pub struct Game {
    #[allow(dead_code)]
    game_id: usize,
    pulls: Vec<Pull>,
}

#[derive(Copy, Clone, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn idx(&self) -> usize {
        match self {
            Color::Red => 0,
            Color::Green => 1,
            Color::Blue => 2,
        }
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => panic!("Bad color {value}"),
        }
    }
}

fn is_possible(bag: &Pull, pull: &Pull) -> bool {
    bag.iter().zip(pull).all(|(b, p)| p <= b)
}

impl Game {
    fn is_possible(&self, bag: &Pull) -> bool {
        self.pulls.iter().all(|pull| is_possible(bag, pull))
    }
}

#[test]
fn test_aoc_2023_day_02() {
    let file = File::open("./tests/test_data/AOC_2023_day_02.txt").unwrap();
    let buf_read = io::BufReader::new(file);

    let games = buf_read
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let (game_desc, game_parts) = line.split_once(": ").unwrap();
            let game_id = game_desc
                .split_once(' ')
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();

            let pulls = game_parts
                .split("; ")
                .map(|part| {
                    let mut result = Pull::default();

                    // split the input string by ',' to get the individual cubes
                    let cubes_split = part.split(',');

                    // parse each cube into a GameSet struct
                    for cube in cubes_split {
                        let cube_split: Vec<&str> = cube.trim().split(' ').collect();
                        let num = cube_split[0].parse::<usize>().unwrap();
                        let color: Color = cube_split[1].into();
                        result[color.idx()] = num;
                    }
                    result
                })
                .collect();
            Game { game_id, pulls }
        })
        .collect::<Vec<Game>>();

    let possible_sum = games
        .iter()
        .filter(|game| game.is_possible(&[12, 13, 14]))
        .map(|game| game.game_id)
        .sum::<usize>();

    println!("Possible Sum: {}", possible_sum);
}
