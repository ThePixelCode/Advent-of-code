use std::{env::args, fs::OpenOptions, io::Read, process::exit};

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Usage {} [input file] [method]", &args[0]);
        exit(1);
    }
    let path_file = &args[1];
    let method = args[2].parse::<u32>().unwrap();
    let mut file = OpenOptions::new().read(true).open(path_file).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    match method {
        1 => solve_part_1(file_content),
        2 => solve_part_2(file_content),
        _ => {
            println!("Invalid method, only 1 or 2 are valid methods");
            exit(1);
        }
    }
}

type Card = char;
const CARDS: [Card; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const CARDS2: [Card; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn check_value(value: char) -> Result<Card, char> {
    if CARDS.iter().any(|ch| *ch == value) {
        return Ok(value);
    }
    Err(value)
}

fn cmp_value(self_card: &Card, other: &Card, order: &[Card; 13]) -> std::cmp::Ordering {
    let self_value = order.iter().position(|ch| ch == self_card).unwrap();
    let other_value = order.iter().position(|ch| ch == other).unwrap();
    self_value.cmp(&other_value)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    FiveOfHand = 7,  // 11111
    FourOfHand = 6,  // 11112
    FullHouse = 5,   // 11122
    ThreeOfKind = 4, // 11123
    TwoPair = 3,     // 11223
    OnePair = 2,     // 11234
    HighCard = 1,    // 12345
}

impl Hand {
    fn get_hand(hand: &Vec<Card>, method: &Method) -> Hand {
        let mut card1 = (None, 0);
        let mut card2 = (None, 0);
        let mut card3 = (None, 0);
        let mut card4 = (None, 0);
        let mut card5 = (None, 0);
        for card in hand {
            if card1.0.is_none() {
                card1 = (Some(card), card1.1 + 1);
                continue;
            }
            if card1.0.is_some_and(|x| x == card) {
                card1 = (card1.0, card1.1 + 1);
                continue;
            }
            if card2.0.is_none() {
                card2 = (Some(card), card2.1 + 1);
                continue;
            }
            if card2.0.is_some_and(|x| x == card) {
                card2 = (card2.0, card2.1 + 1);
                continue;
            }
            if card3.0.is_none() {
                card3 = (Some(card), card3.1 + 1);
                continue;
            }
            if card3.0.is_some_and(|x| x == card) {
                card3 = (card3.0, card3.1 + 1);
                continue;
            }
            if card4.0.is_none() {
                card4 = (Some(card), card4.1 + 1);
                continue;
            }
            if card4.0.is_some_and(|x| x == card) {
                card4 = (card4.0, card4.1 + 1);
                continue;
            }
            if card5.0.is_none() {
                card5 = (Some(card), card5.1 + 1);
                continue;
            }
            if card5.0.is_some_and(|x| x == card) {
                card5 = (card5.0, card5.1 + 1);
                continue;
            }
        }
        if let &Method::Method2 = method {
            if card1.0.is_some_and(|c| *c == 'J') {
                match card1.1 {
                    5 => (),
                    4 => {
                        card1 = (card2.0, 5);
                        card2 = (None, 0);
                    }
                    3 => match card2.1 {
                        2 => {
                            card1 = (card2.0, 5);
                            card2 = (None, 0);
                        }
                        1 => {
                            card1 = (card3.0, 4);
                            card3 = (None, 0);
                        }
                        _ => unreachable!(),
                    },
                    2 => match card2.1 {
                        3 => {
                            card1 = (card2.0, 5);
                            card2 = (None, 0);
                        }
                        2 => {
                            card1 = (card2.0, 4);
                            card2 = card3;
                            card3 = (None, 0);
                        }
                        1 => {
                            card1 = (card3.0, card3.1 + card1.1);
                            card3 = card4;
                            card4 = (None, 0);
                        }
                        _ => unreachable!(),
                    },
                    1 => match card2.1 {
                        4 => {
                            card1 = (card2.0, 5);
                            card2 = (None, 0);
                        }
                        3 => {
                            card1 = (card2.0, 4);
                            card2 = card3;
                            card3 = (None, 0);
                        }
                        2 => {
                            card1 = (card2.0, card2.1 + card1.1);
                            card2 = card3;
                            card3 = card4;
                            card4 = (None, 0);
                        }
                        1 => {
                            if card3.1 == 3 {
                                card1 = (card3.0, 4);
                                card3 = (None, 0);
                            } else if card3.1 == 2 {
                                card1 = (card3.0, 3);
                                card3 = card4;
                                card4 = (None, 0);
                            } else {
                                card1 = (card4.0, card4.1 + card1.1);
                                card4 = card5;
                                card5 = (None, 0);
                            }
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
            if card2.0.is_some_and(|c| *c == 'J') {
                match card2.1 {
                    4 => {
                        card1.1 += card2.1;
                        card2 = (None, 0);
                    }
                    3 => {
                        if card1.1 == 2 {
                            card1.1 += card2.1;
                            card2 = (None, 0);
                        } else {
                            card2 = (card3.0, card3.1 + card2.1);
                            card3 = (None, 0);
                        }
                    }
                    2 => match card1.1 {
                        3 => {
                            card1.1 += card2.1;
                            card2 = (None, 0);
                        }
                        2 => {
                            card1.1 += card2.1;
                            card2 = card3;
                            card3 = (None, 0);
                        }
                        1 => {
                            card2 = (card3.0, card3.1 + card2.1);
                            card3 = card4;
                            card4 = (None, 0);
                        }
                        _ => unreachable!(),
                    },
                    1 => match card1.1 {
                        4 => {
                            card1.1 += card2.1;
                            card2 = (None, 0);
                        }
                        3 => {
                            card1.1 += card2.1;
                            card2 = card3;
                            card3 = (None, 0);
                        }
                        2 => {
                            card1.1 += card2.1;
                            card2 = card3;
                            card3 = card4;
                            card4 = (None, 0);
                        }
                        1 => match card3.1 {
                            3 => {
                                card2 = (card3.0, card3.1 + card2.1);
                                card3 = (None, 0);
                            }
                            2 => {
                                card2 = (card3.0, card3.1 + card2.1);
                                card3 = card4;
                                card4 = (None, 0);
                            }
                            1 => {
                                card2 = (card4.0, card4.1 + card2.1);
                                card4 = card5;
                                card5 = (None, 0);
                            }
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
            if card3.0.is_some_and(|c| *c == 'J') {
                match card3.1 {
                    3 => {
                        card2.1 += card3.1;
                        card3 = (None, 0);
                    }
                    2 => {
                        if card1.1 == 2 {
                            card1.1 += card3.1;
                            card3 = (None, 0);
                        } else if card2.1 == 2 {
                            card2.1 += card3.1;
                            card3 = (None, 0);
                        } else {
                            card3 = (card4.0, card4.1 + card3.1);
                            card4 = (None, 0);
                        }
                    }
                    1 => {
                        if card1.1 == 3 || card1.1 == 2 {
                            card1.1 += card3.1;
                            card3 = card4;
                            card4 = (None, 0);
                        } else if card2.1 == 3 || card2.1 == 2 {
                            card2.1 += card3.1;
                            card3 = card4;
                            card4 = (None, 0);
                        } else {
                            card3 = (card4.0, card4.1 + card3.1);
                            card4 = card5;
                            card5 = (None, 0);
                        }
                    }
                    _ => unreachable!(),
                }
            }
            if card4.0.is_some_and(|c| *c == 'J') {
                if card5.1 == 1 {
                    card4 = (card5.0, card5.1 + card4.1);
                    card5 = (None, 0);
                } else {
                    if card1.1 == 2 {
                        card1.1 += card4.1;
                    } else if card2.1 == 2 {
                        card2.1 += card4.1;
                    } else {
                        card3.1 += card4.1;
                    }
                    card4 = (None, 0);
                }
            }
            if card5.0.is_some_and(|c| *c == 'J') {
                card4.1 += card5.1;
            }
        }
        match card1.1 {
            5 => Hand::FiveOfHand,                                                // 11111
            4 => Hand::FourOfHand,                                                // 11112
            3 if card2.1 == 2 => Hand::FullHouse,                                 // 11122
            3 => Hand::ThreeOfKind,                                               // 11123
            2 if card2.1 == 3 => Hand::FullHouse,                                 // 11222
            2 if card2.1 == 2 || card3.1 == 2 => Hand::TwoPair,                   // 11223 | 11233
            2 => Hand::OnePair,                                                   // 11234
            1 if card2.1 == 4 => Hand::FourOfHand,                                // 12222
            1 if card2.1 == 3 => Hand::ThreeOfKind,                               // 12223
            1 if card2.1 == 2 && card3.1 == 2 => Hand::TwoPair,                   // 12233
            1 if (card2.1 == 2 || card3.1 == 2) && card4.1 == 1 => Hand::OnePair, // 12234 | 12334
            1 if card2.1 == 1 && card3.1 == 1 && card4.1 == 2 => Hand::OnePair,   // 12344
            1 if card2.1 == 1 && card3.1 == 3 => Hand::ThreeOfKind,               // 12333
            1 => Hand::HighCard,                                                  // 12345
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use shuffle::shuffler::Shuffler;

    use super::*;

    #[test]
    fn test_hand_with_1_j() -> Result<(), &'static str> {
        let mut rng = rand::rngs::OsRng::default();
        let mut irs = shuffle::irs::Irs::default();

        let mut input = vec!['7', '7', '7', '7', 'J'];

        for _ in 0..1000 {
            irs.shuffle(&mut input, &mut rng)
                .map_err(|_| "I don't know how to shuffle...")?;
            let hand = Hand::get_hand(&input, &Method::Method2);
            assert_eq!(hand, Hand::FiveOfHand);
        }

        let mut input = vec!['A', '7', '7', '7', 'J'];

        for _ in 0..1000 {
            irs.shuffle(&mut input, &mut rng)
                .map_err(|_| "I don't know how to shuffle...")?;
            let hand = Hand::get_hand(&input, &Method::Method2);
            assert_eq!(hand, Hand::FourOfHand);
        }

        let mut input = vec!['A', 'A', '7', '7', 'J'];

        for _ in 0..1000 {
            irs.shuffle(&mut input, &mut rng)
                .map_err(|_| "I don't know how to shuffle...")?;
            let hand = Hand::get_hand(&input, &Method::Method2);
            assert_eq!(hand, Hand::FullHouse);
        }

        let mut input = vec!['A', 'A', 'K', '7', 'J'];

        for _ in 0..1000 {
            irs.shuffle(&mut input, &mut rng)
                .map_err(|_| "I don't know how to shuffle...")?;
            let hand = Hand::get_hand(&input, &Method::Method2);
            assert_eq!(hand, Hand::ThreeOfKind);
        }

        let mut input = vec!['A', 'K', 'Q', '7', 'J'];

        for _ in 0..1000 {
            irs.shuffle(&mut input, &mut rng)
                .map_err(|_| "I don't know how to shuffle...")?;
            let hand = Hand::get_hand(&input, &Method::Method2);
            assert_eq!(hand, Hand::OnePair);
        }
        Ok(())
    }

    #[test]
    fn test_hand_with_2_j() -> Result<(), &'static str> {
        let mut rng = rand::rngs::OsRng::default();
        let mut irs = shuffle::irs::Irs::default();

        let mut input = vec!['7', '7', '7', 'J', 'J'];

        for _ in 0..1000 {
            irs.shuffle(&mut input, &mut rng)
                .map_err(|_| "I don't know how to shuffle...")?;
            let hand = Hand::get_hand(&input, &Method::Method2);
            assert_eq!(hand, Hand::FiveOfHand);
        }

        let mut input = vec!['A', '7', '7', 'J', 'J'];

        for _ in 0..1000 {
            irs.shuffle(&mut input, &mut rng)
                .map_err(|_| "I don't know how to shuffle...")?;
            let hand = Hand::get_hand(&input, &Method::Method2);
            assert_eq!(hand, Hand::FourOfHand);
        }

        let mut input = vec!['A', '7', 'K', 'J', 'J'];

        for _ in 0..1000 {
            irs.shuffle(&mut input, &mut rng)
                .map_err(|_| "I don't know how to shuffle...")?;
            let hand = Hand::get_hand(&input, &Method::Method2);
            assert_eq!(hand, Hand::ThreeOfKind);
        }
        Ok(())
    }

    #[test]
    fn test_hand_with_3_j() -> Result<(), &'static str> {
        let mut rng = rand::rngs::OsRng::default();
        let mut irs = shuffle::irs::Irs::default();

        let mut input = vec!['7', '7', 'J', 'J', 'J'];

        for _ in 0..1000 {
            irs.shuffle(&mut input, &mut rng)
                .map_err(|_| "I don't know how to shuffle...")?;
            let hand = Hand::get_hand(&input, &Method::Method2);
            assert_eq!(hand, Hand::FiveOfHand);
        }

        let mut input = vec!['A', '7', 'J', 'J', 'J'];

        for _ in 0..1000 {
            irs.shuffle(&mut input, &mut rng)
                .map_err(|_| "I don't know how to shuffle...")?;
            let hand = Hand::get_hand(&input, &Method::Method2);
            assert_eq!(hand, Hand::FourOfHand);
        }
        Ok(())
    }

    #[test]
    fn test_hand_with_4_j() -> Result<(), &'static str> {
        let mut rng = rand::rngs::OsRng::default();
        let mut irs = shuffle::irs::Irs::default();

        let mut input = vec!['7', 'J', 'J', 'J', 'J'];

        for _ in 0..1000 {
            irs.shuffle(&mut input, &mut rng)
                .map_err(|_| "I don't know how to shuffle...")?;
            let hand = Hand::get_hand(&input, &Method::Method2);
            assert_eq!(hand, Hand::FiveOfHand);
        }
        Ok(())
    }

    #[test]
    fn test_hand_with_5_j() {
        let hand = Hand::get_hand(&vec!['J', 'J', 'J', 'J', 'J'], &Method::Method2);
        assert_eq!(hand, Hand::FiveOfHand);
    }

    #[test]
    fn test_value() {
        for (c_i, c) in CARDS2.iter().enumerate() {
            for (o_i, o) in CARDS2.iter().enumerate() {
                assert_eq!(cmp_value(c, o, &CARDS2), c_i.cmp(&o_i))
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum Method {
    #[default]
    Method1,
    Method2,
}

#[derive(Debug, PartialEq, Eq)]
struct HandAndBid(Vec<Card>, u32, Method);

impl std::str::FromStr for HandAndBid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand = &s[0..5];
        let mut _hand = Vec::new();
        for c in hand.chars() {
            _hand.push(check_value(c).map_err(|_| ())?);
        }
        let bid = s[6..].parse::<u32>().map_err(|_| ())?;

        Ok(HandAndBid(_hand, bid, Method::default()))
    }
}

impl PartialOrd for HandAndBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let h_hand = Hand::get_hand(&self.0, &self.2);
        let h_other_hand = Hand::get_hand(&other.0, &self.2);

        if h_hand != h_other_hand {
            return Some(h_hand.cmp(&h_other_hand));
        }

        if let Some((card1, card2)) = self.0.iter().zip(other.0.iter()).find(|(x, y)| x != y) {
            return Some(match self.2 {
                Method::Method1 => cmp_value(card1, card2, &CARDS),
                Method::Method2 => cmp_value(card1, card2, &CARDS2),
            });
        } else {
            return Some(std::cmp::Ordering::Equal);
        }
    }
}

impl Ord for HandAndBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let h_hand = Hand::get_hand(&self.0, &self.2);
        let h_other_hand = Hand::get_hand(&other.0, &self.2);

        if h_hand != h_other_hand {
            return h_hand.cmp(&h_other_hand);
        }

        if let Some((card1, card2)) = self.0.iter().zip(other.0.iter()).find(|(x, y)| x != y) {
            return match self.2 {
                Method::Method1 => cmp_value(card1, card2, &CARDS),
                Method::Method2 => cmp_value(card1, card2, &CARDS2),
            };
        } else {
            return std::cmp::Ordering::Equal;
        }
    }
}

fn solve_part_1(file_content: String) {
    let mut hands = Vec::new();

    for line in file_content.lines() {
        match <HandAndBid as std::str::FromStr>::from_str(line) {
            Ok(hand) => hands.push(hand),
            Err(_) => {
                eprintln!("Something went horrificly wrong!!!");
                exit(1)
            }
        }
    }

    hands.sort();

    let sum = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1_u32) * hand.1)
        .sum::<u32>();
    println!("{}", sum);
}

fn solve_part_2(file_content: String) {
    let mut hands = Vec::new();

    for line in file_content.lines() {
        match <HandAndBid as std::str::FromStr>::from_str(line) {
            Ok(mut hand) => hands.push({
                hand.2 = Method::Method2;
                hand
            }),
            Err(_) => {
                eprintln!("Something went horrificly wrong!!!");
                exit(1)
            }
        }
    }

    hands.sort();

    let sum = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| {
            println!("{}. {:?}", &rank, &hand);
            (rank, hand)
        })
        .map(|(rank, hand)| (rank as u32 + 1_u32) * hand.1)
        // .map(|x| {
        //     println!("{}", &x);
        //     x
        // })
        .sum::<u32>();
    println!("{}", sum);
}
