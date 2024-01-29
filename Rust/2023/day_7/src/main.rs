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

fn check_value(value: char) -> Result<Card, char> {
    if CARDS.iter().any(|ch| *ch == value) {
        return Ok(value);
    }
    Err(value)
}

fn cmp_value(self_card: &Card, other: &Card) -> std::cmp::Ordering {
    let self_value = CARDS.iter().position(|ch| ch == self_card).unwrap();
    let other_value = CARDS.iter().position(|ch| ch == other).unwrap();
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
    fn get_hand(hand: &Vec<Card>) -> Hand {
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

#[derive(Debug, PartialEq, Eq)]
struct HandAndBid(Vec<Card>, u32);

impl std::str::FromStr for HandAndBid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand = &s[0..5];
        let mut _hand = Vec::new();
        for c in hand.chars() {
            _hand.push(check_value(c).map_err(|_| ())?);
        }
        let bid = s[6..].parse::<u32>().map_err(|_| ())?;

        Ok(HandAndBid(_hand, bid))
    }
}

impl PartialOrd for HandAndBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let h_hand = Hand::get_hand(&self.0);
        let h_other_hand = Hand::get_hand(&other.0);

        if h_hand != h_other_hand {
            return Some(h_hand.cmp(&h_other_hand));
        }

        if let Some((card1, card2)) = self.0.iter().zip(other.0.iter()).find(|(x, y)| x != y) {
            return Some(cmp_value(card1, card2));
        } else {
            return Some(std::cmp::Ordering::Equal);
        }
    }
}

impl Ord for HandAndBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let h_hand = Hand::get_hand(&self.0);
        let h_other_hand = Hand::get_hand(&other.0);

        if h_hand != h_other_hand {
            return h_hand.cmp(&h_other_hand);
        }

        if let Some((card1, card2)) = self.0.iter().zip(other.0.iter()).find(|(x, y)| x != y) {
            return cmp_value(card1, card2);
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
        // .map(|(rank, hand)| {
        //     println!("{}. {:?}", &rank, &hand);
        //     (rank, hand)
        // })
        .map(|(rank, hand)| (rank as u32 + 1_u32) * hand.1)
        // .map(|x| {
        //     println!("{}", &x);
        //     x
        // })
        .sum::<u32>();
    println!("{}", sum);
}

fn solve_part_2(file_content: String) {
    todo!()
}
