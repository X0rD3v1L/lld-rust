use std::collections::{HashMap, BinaryHeap};
use std::cmp::{Reverse,max};

struct Leaderboard {
    player_scores: HashMap<u32, u32>
}

impl Leaderboard {
    fn new() -> Self {
        Leaderboard {
            player_scores: HashMap::new(),
        }
    }

    fn add_score(&mut self, player_id: u32, score: u32) {
        *self.player_scores.entry(player_id).or_insert(0) += score;
    }

    fn top_k(&self, k: usize) -> u32 {
        let mut k_largest = BinaryHeap::new();

        for &score in self.player_scores.values() {
            if k_largest.len() < k {
                k_largest.push(Reverse(score));
            } else {
                let Reverse(existing_score) = k_largest.pop().unwrap();
                let larger = max(score, existing_score);
                k_largest.push(Reverse(larger));
            }
        }
        k_largest.iter()
                .map(|Reverse(score)| score)
                .sum()

    }

    fn remove(&mut self, player_id: u32) {
        self.player_scores.remove(&player_id);
    }

    fn current_state(&self) -> String {
        let mut parts = Vec::new();
        for (&player, &score) in &self.player_scores {
            parts.push(format!("[{}:{}]", player, score));
        }
        parts.join("")
    }
}

fn main() {
    let mut score_board = Leaderboard::new();
    score_board.add_score(1, 30);
    score_board.add_score(2, 70);
    score_board.add_score(3, 50);
    println!("{}", score_board.current_state());
    println!("Top 2 scores sum :: {}", score_board.top_k(2));
    score_board.remove(2);
    println!("Removed player with id :: 2");
    println!("Top 2 scores sum :: {}", score_board.top_k(2));
}

/*
use std::cmp::{max, Reverse};
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
enum LeaderboardError {
    PlayerNotFound,
    InvalidK,
}

type LBResult<T> = Result<T, LeaderboardError>;

struct Leaderboard {
    player_scores: HashMap<u32, u32>,
}

impl Leaderboard {
    fn new() -> Self {
        Self {
            player_scores: HashMap::new(),
        }
    }

    fn add_score(&mut self, player_id: u32, score: u32) {
        *self.player_scores.entry(player_id).or_insert(0) += score;
    }

    fn top_k(&self, k: usize) -> LBResult<u32> {
        if k == 0 || k > self.player_scores.len() {
            return Err(LeaderboardError::InvalidK);
        }

        let mut k_largest = BinaryHeap::new();

        for &score in self.player_scores.values() {
            if k_largest.len() < k {
                k_largest.push(Reverse(score));
            } else {
                let Reverse(existing_score) = k_largest.pop().unwrap();

                let larger = max(score, existing_score);

                k_largest.push(Reverse(larger));
            }
        }

        Ok(
            k_largest
                .iter()
                .map(|Reverse(score)| score)
                .sum(),
        )
    }

    fn remove(&mut self, player_id: u32) -> LBResult<()> {
        match self.player_scores.remove(&player_id) {
            Some(_) => Ok(()),
            None => Err(LeaderboardError::PlayerNotFound),
        }
    }

    fn current_state(&self) -> String {
        let mut parts = Vec::new();

        for (&player, &score) in &self.player_scores {
            parts.push(format!("[{}:{}]", player, score));
        }

        parts.join("")
    }
}

fn main() {
    let mut score_board = Leaderboard::new();

    score_board.add_score(1, 30);
    score_board.add_score(2, 70);
    score_board.add_score(3, 50);

    println!("{}", score_board.current_state());

    match score_board.top_k(100) {
        Ok(sum) => println!("Top 2 scores sum :: {}", sum),
        Err(e) => println!("Error :: {:?}", e),
    }

    match score_board.remove(2) {
        Ok(_) => println!("Removed player with id :: 2"),
        Err(e) => println!("Error :: {:?}", e),
    }

    match score_board.top_k(2) {
        Ok(sum) => println!("Top 2 scores sum :: {}", sum),
        Err(e) => println!("Error :: {:?}", e),
    }
}

*/