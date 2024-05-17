use derive_more::Display;
use std::fmt::{Display, Formatter};

const ITEMS: &'static str = r#"
tx_id,start,end
a,1,5
b,2,3
c,4,6
"#;

const INPUT: &str = r#"
tx_id,start,end
a,1,5
b,2,3
c,4,6
"#;

//tx_id,type,time
// a,start,1
// b,start,2
// b,end,3
// c,start,4
// a,end,5
// c,end,6

#[derive(Debug)]
struct Transaction<'a> {
    tx_id: &'a str,
    start: u64,
    end: u64,
}

#[derive(Debug)]
enum Event<'a> {
    Start { id: &'a str, t: u64 },
    End { id: &'a str, t: u64 },
}

#[derive(Debug)]
struct EventData<'a> {
    tx_id: &'a str,
    time: u64,
    typ: EventType,
}

#[derive(Debug)]
enum EventType {
    Start,
    End,
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Start => {
                write!(f, "start")
            }
            EventType::End => {
                write!(f, "end")
            }
        }
    }
}

// impl<'a> From<(usize, &'a str)> for Transaction<'a> {
//     fn from(value: (usize, &'a str)) -> Self {
//         match (value.0, value.1) {
//             (0, va) => Item::Tx(va),
//             (1, va) if va.parse::<u64>().is_ok() => Item::Start(va.parse().unwrap()),
//             (2, va) if va.parse::<u64>().is_ok() => Item::End(va.parse().unwrap()),
//             _ => {
//                 unreachable!()
//             }
//         }
//     }
// }

#[derive(Debug)]
enum Item<'a> {
    Tx(&'a str),
    Start(u64),
    End(u64),
}

impl<'a> From<(usize, &'a str)> for Item<'a> {
    fn from(value: (usize, &'a str)) -> Self {
        match (value.0, value.1) {
            (0, va) => Item::Tx(va),
            (1, va) if va.parse::<u64>().is_ok() => Item::Start(va.parse().unwrap()),
            (2, va) if va.parse::<u64>().is_ok() => Item::End(va.parse().unwrap()),
            _ => {
                unreachable!()
            }
        }
    }
}

// pub fn parse_reorder(input: &str) -> impl Iterator<Item = Transaction> {
//     todo!()
// }

pub fn parse(s: &str) -> Item<'_> {
    match s.parse::<u64>() {
        Ok(v) => Item::Start(v),
        _ => Item::Tx(s),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::Lines;

    #[test]
    fn test_parse() {
        // let v: Vec<Item<'_>> = ITEMS.split(",").map(Item::from).collect();
        // println!("{:?}", v);

        let mut a = ITEMS
            .lines()
            .filter(|line| !line.is_empty())
            .skip(1)
            .map(|line| {
                let mut a = line.splitn(3, ",");
                match (a.next(), a.next(), a.next()) {
                    (Some(tx_id), Some(start), Some(end)) => Transaction {
                        tx_id,
                        start: start.parse::<u64>().unwrap(),
                        end: end.parse::<u64>().unwrap(),
                    },
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>();

        println!("{:?}", a);

        // let mut b = a
        //     .iter()
        //     .map(|tx| {
        //         let s = Event::Start {
        //             id: tx.tx_id,
        //             t: tx.start,
        //         };
        //         let e = Event::End {
        //             id: tx.tx_id,
        //             t: tx.end,
        //         };
        //
        //         (s, e)
        //     })
        //     .collect::<Vec<_>>()
        //     .sort_by(|(s, e)| match s {
        //         Event::Start { .., t: } => { }
        //         Event::End { .. } => {}
        //     });
    }

    #[test]
    fn test_alt_a() {
        let mut a = ITEMS
            .lines()
            .filter(|line| !line.is_empty())
            .skip(1)
            .map(|line| {
                let mut a = line.splitn(3, ",");
                match (a.next(), a.next(), a.next()) {
                    (Some(tx_id), Some(start), Some(end)) => (
                        EventData {
                            tx_id,
                            time: start.parse::<u64>().unwrap(),
                            typ: EventType::Start,
                        },
                        EventData {
                            tx_id,
                            time: end.parse::<u64>().unwrap(),
                            typ: EventType::End,
                        },
                    ),
                    _ => unreachable!(),
                }
            })
            .fold(Vec::new(), |mut acc, p| {
                acc.push(p.0);
                acc.push(p.1);
                // acc.extend(&[p.0, p.1]);
                acc
            });

        a.sort_by_key(|a| a.time);
        // .fold(Vec::new(), |mut acc, p| {
        //     acc.extend(&[p.0, p.1]);
        //     acc
        // });

        for x in a {
            println!("{},{},{}", x.tx_id, x.typ, x.time)
        }

        // let (mut lower_idx, mut upper_idx) = (0usize, a.len() - 1);
        // let mut i = 0usize;
        // while i <= upper_idx {
        //     if a[i].0
        // }
    }

    #[test]
    fn test_alt() {
        // let table: Vec<Vec<&str>> = INPUT
        //     .lines()
        //     .map(|line| line.split(",").collect())
        //     .collect();
        //
        // println!("{:?}", table);

        // let mut lines = ITEMS.lines();
        // loop {
        //     match lines {
        //         Lines(_) => {}
        //     }
        // }

        // let mut a = &ITEMS.lines();

        // ITEMS.split(",").all(|item| {
        //     let i = match item[1..] {
        //         "tx_id" => 0usize,
        //         "start" => 1,
        //         "end" => 2,
        //         _ => unreachable!(),
        //     };
        // })
        // for (idx, line) in ITEMS.split(",").enumerate() {
        //     println!("idx {} line {}", idx, line);
        // }

        // for line in ITEMS.lines() {
        //     println!("{}", line);
        // }

        let a = ITEMS
            .lines()
            .skip(2)
            .map(|line| {
                let parts: Vec<Item<'_>> = line.split(",").enumerate().map(Item::from).collect();
                parts
            })
            .collect::<Vec<_>>();
        println!("{:?}", a);

        // let mut lines = ITEMS.lines().map();
        // let header = lines.next();

        // ITEMS.lines().all(|line| {
        //     let a = match line[..1].split(",").nth(0).unwrap() {
        //         "tx_id" => 0usize,
        //         "start" => 1,
        //         "end" => 2,
        //         _ => 3,
        //     };
        //     println!("{}", a);
        //     true
        // });

        // ITEMS.split(",").all(|item| {
        //     let a = match &item[1..] {
        //         "tx_id" => 0usize,
        //         "start" => 1,
        //         "end" => 2,
        //         _ => {
        //             println!("{:?}", &item[1..]);
        //             3
        //         }
        //     };
        //     println!("{}", a);
        //     true
        // });
    }
}
