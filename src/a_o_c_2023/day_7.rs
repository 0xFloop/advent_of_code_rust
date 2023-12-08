use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    Invalid,
}
impl From<u32> for HandType {
    fn from(i: u32) -> Self {
        //inverse because that is how the challenge ranks
        match i {
            7 => HandType::FiveOfAKind,
            6 => HandType::FourOfAKind,
            5 => HandType::FullHouse,
            4 => HandType::ThreeOfAKind,
            3 => HandType::TwoPair,
            2 => HandType::OnePair,
            1 => HandType::HighCard,
            _ => HandType::Invalid,
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Hand<'a> {
    cards: &'a str,
    bid: u64,
    hand_type: HandType,
    rank: Option<u64>,
}

pub fn solve_part_1() -> Option<u64> {
    let mut file = File::open("utils/2023/day_7_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut hands_map: HashMap<HandType, Vec<Hand>> = HashMap::new();

    for line in lines {
        let info = line.split(" ").collect::<Vec<&str>>();
        let hand_str = info[0];
        let bid_str = info[1];
        //determine the current hand

        let hand_type = determine_hand_type(hand_str);

        let hand = Hand {
            cards: hand_str,
            bid: bid_str.parse().unwrap(),
            hand_type,
            rank: None,
        };

        let hand_type_vec = hands_map.get_mut(&hand_type);
        if hand_type_vec.is_some() {
            hand_type_vec.unwrap().push(hand);
        } else {
            hands_map.insert(hand_type, vec![hand]);
        }
    }
    let mut curr_rank: u64 = 1;
    for i in 1..=7 {
        let curr_hand_type = HandType::from(i);
        if let Some(hands) = hands_map.get_mut(&curr_hand_type) {
            if hands.len() == 1 {
                let hand = hands.get_mut(0).unwrap();
                hand.rank = Some(curr_rank);
                curr_rank += 1;
                continue;
            }
            //loops through all hands of certain hand_type
            for i in 0..hands.len() {
                for j in 0..hands.len() - i - 1 {
                    let hand = hands.get(j).unwrap();
                    let cards = hand.cards;
                    let next_hand = hands.get(j + 1).unwrap();

                    for k in 0..cards.len() {
                        let curr_card = parse_card(cards.chars().nth(k).unwrap());

                        let next_hand_corresponding_card =
                            parse_card(next_hand.cards.chars().nth(k).unwrap());
                        if curr_card == next_hand_corresponding_card {
                            continue;
                        } else if curr_card > next_hand_corresponding_card {
                            hands.swap(j, j + 1);
                            break;
                        } else {
                            break;
                        }
                    }
                }
            }

            for hand in hands {
                hand.rank = Some(curr_rank);
                curr_rank += 1;
            }
        }
    }

    let mut total_winnings: u64 = 0;

    for (hand_type, hands) in hands_map {
        for hand in hands {
            total_winnings = total_winnings + hand.bid * hand.rank.unwrap();
        }
    }
    return Some(total_winnings);
}

pub fn solve_part_2() -> Option<u64> {
    let mut file = File::open("utils/2023/day_7_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut hands_map: HashMap<HandType, Vec<Hand>> = HashMap::new();

    for line in lines {
        let info = line.split(" ").collect::<Vec<&str>>();
        let hand_str = info[0];
        let bid_str = info[1];
        //determine the current hand

        let hand_type = determine_hand_type(hand_str);

        let hand = Hand {
            cards: hand_str,
            bid: bid_str.parse().unwrap(),
            hand_type,
            rank: None,
        };

        let hand_type_vec = hands_map.get_mut(&hand_type);
        if hand_type_vec.is_some() {
            hand_type_vec.unwrap().push(hand);
        } else {
            hands_map.insert(hand_type, vec![hand]);
        }
    }
    let mut curr_rank: u64 = 1;
    for i in 1..=7 {
        let curr_hand_type = HandType::from(i);
        if let Some(hands) = hands_map.get_mut(&curr_hand_type) {
            if hands.len() == 1 {
                let hand = hands.get_mut(0).unwrap();
                hand.rank = Some(curr_rank);
                curr_rank += 1;
                continue;
            }
            //loops through all hands of certain hand_type
            for i in 0..hands.len() {
                for j in 0..hands.len() - i - 1 {
                    let hand = hands.get(j).unwrap();
                    let cards = hand.cards;
                    let next_hand = hands.get(j + 1).unwrap();

                    for k in 0..cards.len() {
                        let curr_card = parse_card(cards.chars().nth(k).unwrap());

                        let next_hand_corresponding_card =
                            parse_card(next_hand.cards.chars().nth(k).unwrap());
                        if curr_card == next_hand_corresponding_card {
                            continue;
                        } else if curr_card > next_hand_corresponding_card {
                            hands.swap(j, j + 1);
                            break;
                        } else {
                            break;
                        }
                    }
                }
                //251025240 too low
                //251025240
            }

            for hand in hands {
                hand.rank = Some(curr_rank);
                curr_rank += 1;
            }
        }
    }

    let mut total_winnings: u64 = 0;

    for (hand_type, hands) in hands_map {
        for hand in hands {
            total_winnings = total_winnings + hand.bid * hand.rank.unwrap();
        }
    }
    return Some(total_winnings);
}

fn parse_card(card: char) -> u64 {
    if card.is_ascii_digit() {
        return card as u64 - '0' as u64;
    } else {
        match card {
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => {
                println!("unknown card here");
                0
            }
        }
    }
}

fn determine_hand_type(hand: &str) -> HandType {
    let mut pair_count = 0;
    let mut three_of_a_kind_count = 0;
    let mut mut_hand = hand.to_owned();
    let mut contains_joker = false;

    for i in 0..5 {
        let card = mut_hand.chars().nth(i).unwrap();
        if card == '*' {
            continue;
        }
        if card == 'J' {
            contains_joker = true;
        }

        let num_of_card = hand.matches(card).count();

        match num_of_card {
            5 => return HandType::FiveOfAKind,
            4 => {
                if hand.matches('J').count() > 0 {
                    return HandType::FiveOfAKind;
                } else {
                    return HandType::FourOfAKind;
                }
            }
            3 => {
                mut_hand = mut_hand.replace(card, "*");
                three_of_a_kind_count = 1
            }
            2 => {
                mut_hand = mut_hand.replace(card, "*");
                pair_count += 1
            }
            _ => (),
        }
    }
    if pair_count == 1 && three_of_a_kind_count == 1 {
        if contains_joker {
            return HandType::FiveOfAKind;
        }
        return HandType::FullHouse;
    } else if three_of_a_kind_count == 1 {
        if contains_joker {
            return HandType::FourOfAKind;
        }
        return HandType::ThreeOfAKind;
    } else if pair_count > 1 {
        if contains_joker {
            let joker_count = hand.matches('J').count();
            if joker_count == 1 {
                return HandType::FullHouse;
            } else if joker_count == 2 {
                return HandType::FourOfAKind;
            }
        }
        return HandType::TwoPair;
    } else if pair_count == 1 {
        if contains_joker {
            return HandType::ThreeOfAKind;
        }
        return HandType::OnePair;
    } else if contains_joker{
        return HandType::OnePair;
    }
    return HandType::HighCard;
}
