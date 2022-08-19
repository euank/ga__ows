use std::collections::HashSet;
use std::io::{self, Write};

use itertools::Itertools;
use rand::Rng;
use rand::seq::SliceRandom;

const DICT_COMMON: &str = include_str!("../dicts/common.en.lines");
const DICT_FULL: &str = include_str!("../dicts/full.en.lines");

fn main() -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();

    let common: HashSet<&str> = DICT_COMMON.lines().collect();
    let full: HashSet<&str> = DICT_FULL.lines().collect();

    let puzzle = find_puzzle(&mut rng, &common, &full);
    println!("Puzzle: {}, Banned: {}", puzzle.blanks, puzzle.banned.iter().join(", "));


    loop {
        print!("Answer: ");
        std::io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if buffer.trim() == puzzle.answer {
            println!("Correct!");
            break;
        } else {
            print!("Incorrect - try again? [Y/n]");
            std::io::stdout().flush()?;
            buffer.clear();
            io::stdin().read_line(&mut buffer)?;
            if buffer.to_lowercase().starts_with('n') {
                println!("The correct answer was {}", puzzle.answer);
                break;
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct Puzzle {
    answer: String,
    banned: HashSet<char>,
    blanks: String,
}

fn find_puzzle<R: Rng>(rng: &mut R, common: &HashSet<&str>, full: &HashSet<&str>) -> Puzzle {
    // otherwise, try to find any viable child path
    let mut full = full.clone();
    for word in common {
        full.remove(word);
    }
    let mut puzzle = find_sub_puzzle(rng, "", &HashSet::new(), 0, common, &full).unwrap();

    // trim unessential bans
    let banned = puzzle.banned.clone();
    let mut minimal_bans = puzzle.banned.clone();
    for ban in banned {
        println!("Trying to remove {}", ban);
        minimal_bans.remove(&ban);
        let c = Puzzle{answer: puzzle.answer.clone(), banned: minimal_bans.clone(), blanks: puzzle.blanks.clone()};
        if is_valid(c.clone(), common, &full) {
            println!("valid");
            puzzle = c
        } else {
            minimal_bans.insert(ban);
        }
    }
    puzzle
}

fn exact_matches(word: &str, blanks: &str) -> bool {
    if word.len() != blanks.len() {
        return false
    }
    let word_chars = word.chars().collect_vec();
    let blanks_chars = blanks.chars().collect_vec();
    for i in 0..word_chars.len() {
        if blanks_chars[i] == '_' {
            continue
        }
        if word_chars[i] != blanks_chars[i] {
            return false
        }
    }
    true
}

fn is_valid(p: Puzzle, common: &HashSet<&str>, full: &HashSet<&str>) -> bool {
    let num_common = common.iter()
        .filter(|el| exact_matches(el, &p.blanks))
        .filter(|el| p.banned.is_empty() || !(el.chars().any(|c| p.banned.contains(&c))))
        .count();
    let num_full = full.iter()
        .filter(|el| exact_matches(el, &p.blanks))
        .filter(|el| p.banned.is_empty() || !(el.chars().any(|c| p.banned.contains(&c))))
        .count();
    println!("num common and full: {} {}", num_common, num_full);
    num_common == 1 && num_full == 0
}


fn prefix_matches(el: &str, prefix: &str) -> bool {
    if prefix.len() > el.len() {
        return false
    }
    let elc = el.chars().collect_vec();
    for c in prefix.chars().enumerate() {
        if c.1 == '_' {
            continue
        }
        if elc[c.0] != c.1 {
            return false
        }
    }
    true
}

fn find_sub_puzzle<R: Rng>(rng: &mut R, prefix: &str, banned: &HashSet<char>, num_blanks: u64, candidates: &HashSet<&str>, full: &HashSet<&str>) -> Option<Puzzle> {
    let c: HashSet<&str> = candidates.clone().into_iter()
        .filter(|el| prefix_matches(el, prefix))
        .filter(|el| banned.is_empty() || !(el.chars().any(|c| banned.contains(&c))))
        .collect();
    let full_matches: HashSet<&str> = full.clone().into_iter()
        .filter(|el| prefix_matches(el, prefix))
        .filter(|el| banned.is_empty() || !(el.chars().any(|c| banned.contains(&c))))
        .collect();

    // failed path, nothing to see here
    if c.is_empty() {
        println!("Dead end: {}", prefix);
        return None;
    }

    if c.len() == 1 && full_matches.is_empty() {
        // we're done, we have a working puzzle. For now, just blank out the rest
        let answer = c.iter().next().unwrap().to_string();

        let blanks = num_blanks + (answer.len() - prefix.len()) as u64;
        if blanks == 1 {
            // dead end, we don't like puzzles with no blanks.
            return None;
        }

        return Some(Puzzle{
            answer: answer.clone(),
            banned: banned.clone(),
            // prefix, + remaining chars as blanks
            blanks: prefix.to_string() + &(0..(answer.len() - prefix.len())).map(|_| '_').collect::<String>(),
        });
    }

    println!("candidates: {}", c.len());
    // so our next options are:
    // 1. blank
    // 2. ban a letter
    // 3. append a letter
    //
    // We want to weight things appropriately. Let's have 30% chance of a blank, 40% of a ban, and
    // 60% of a letter.
    // Add all options so we might do any of them of course, but then weight appropriately.
    #[derive(PartialEq, Clone, Debug)]
    enum Move {
        Blank,
        Ban(char),
        Append(char),
    }
    let mut moves = vec![Move::Blank];
    moves.append(&mut (('a'..='z').map(Move::Ban).collect_vec()));
    moves.append(&mut (('a'..='z').map(Move::Append).collect_vec()));

    while !moves.is_empty() {
        let next_move = moves.choose_weighted(rng, |m| match m {
            Move::Blank => 30f64,
            Move::Ban(_) => 100f64 / 26f64,
            Move::Append(_) => 40f64 / 26f64,
        }).unwrap().clone();
        moves.retain(|el| *el != next_move);

        println!("Trying move: {:?}", next_move);

        let p = match next_move {
            Move::Blank => {
                find_sub_puzzle(rng, &(prefix.to_owned() + &'_'.to_string()), banned, num_blanks + 1, &c, full)
            }
            Move::Ban(chr) => {
                if banned.contains(&chr) {
                    continue
                }
                let mut subbanned = banned.clone();
                subbanned.insert(chr);
                find_sub_puzzle(rng, prefix, &subbanned, num_blanks, &c, full)
            }
            Move::Append(chr) => {
                find_sub_puzzle(rng, &(prefix.to_owned() + &chr.to_string()), banned, num_blanks, &c, full)
            }
        };

        if p.is_some() {
            return p
        }
    }
    None
}
