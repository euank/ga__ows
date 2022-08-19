use std::collections::{HashMap, HashSet};
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
    println!("Puzzle: {}", puzzle.blanks);


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
    banned: String,
    blanks: String,
}

fn find_puzzle<R: Rng>(rng: &mut R, common: &HashSet<&str>, full: &HashSet<&str>) -> Puzzle {
    // otherwise, try to find any viable child path
    let mut full_minus_common = full.clone();
    for word in common {
        full_minus_common.remove(word);
    }
    find_sub_puzzle(rng, "", "", 0, common, &full_minus_common).unwrap()
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

fn find_sub_puzzle<R: Rng>(rng: &mut R, prefix: &str, banned: &str, num_blanks: u64, candidates: &HashSet<&str>, full: &HashSet<&str>) -> Option<Puzzle> {
    let c: HashSet<&str> = candidates.clone().into_iter().filter(|el| prefix_matches(el, prefix)).collect();
    let full_matches: HashSet<&str> = full.clone().into_iter().filter(|el| prefix_matches(el, prefix)).collect();

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
            banned: banned.to_string(),
            // prefix, + remaining chars as blanks
            blanks: prefix.to_string() + &(0..(answer.len() - prefix.len())).map(|_| '_').collect::<String>(),
        });
    }

    println!("candidates: {}", c.len());

    // otherwise, let's use rng to decide what to do
    let mut next_options = ('a'..='z').collect_vec();
    next_options.shuffle(rng);
    // weight blanks at 30%
    if rng.gen::<f64>() < 0.3 {
        next_options.push('_');
        next_options.rotate_right(1);
    } else {
        next_options.rotate_right(rng.gen_range(0..27));
    }

    // see if there's any viable puzzles down this path
    for next in next_options {
        let next_prefix = prefix.to_owned() + &next.to_string();
        if let Some(p) = find_sub_puzzle(rng, &next_prefix, &banned, num_blanks + (if next == '_' { 1 } else { 0 }), &c, full) {
            return Some(p);
        }
    }
    None
}
