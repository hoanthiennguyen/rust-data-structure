use rand::Rng;
use skiplist::OrderedSkipList;
use std::cmp::Ordering;
use std::time;
use uuid::Uuid;

mod skipped_list;

struct PlayerScore {
    pub _user_id: String,
    pub score: i32,
}

impl PlayerScore {
    pub fn new(_user_id: String, score: i32) -> Self {
        Self { _user_id, score }
    }
}

impl PartialEq<Self> for PlayerScore {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for PlayerScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

fn find_matching_player(
    queue: &OrderedSkipList<PlayerScore>,
    player_index: usize,
) -> Option<&PlayerScore> {
    let selected_player = queue.get(player_index);
    if selected_player.is_none() {
        return None;
    }
    let selected_player = selected_player.unwrap();

    let mut opponent: Option<&PlayerScore> = None;
    let threshold = std::env::var("THRESHOLD").unwrap_or(String::from("500")).parse::<i32>().unwrap();
    let max_diff_index = std::env::var("MAX_DIFF_INDEX").unwrap_or(String::from("10")).parse::<usize>().unwrap();
    for i in 1..max_diff_index {
        if player_index >= i {
            let first_player = queue.get(player_index - i).unwrap();
            if (first_player.score - selected_player.score).abs() <= threshold {
                opponent = Some(first_player);
                break;
            }
        }

        let second_player = queue.get(player_index + i);
        if let Some(second_player) = second_player {
            if (second_player.score - selected_player.score).abs() <= threshold {
                opponent = Some(second_player);
                break;
            }
        }
    }
    return opponent;
}

fn construct_queue(number_of_players: usize) -> OrderedSkipList<PlayerScore> {
    let mut ordered_skip_list: OrderedSkipList<PlayerScore> = OrderedSkipList::new();
    let mut rng = rand::thread_rng();
    let max_score = 20_000;
    for _ in 1..number_of_players {
        let score = rng.gen::<u32>() % max_score;
        let user_id = Uuid::new_v4().to_string();
        ordered_skip_list.insert(PlayerScore::new(user_id, score as i32))
    }
    return ordered_skip_list
}

fn main() {
    let number_of_players = std::env::var("PLAYERS").unwrap().parse::<i32>().unwrap();
    let ordered_skip_list = construct_queue(number_of_players as usize);

    let number_of_matches = std::env::var("MATCHES").unwrap().parse::<i32>().unwrap();
    let mut rng = rand::thread_rng();

    let begin = time::Instant::now();
    let mut found_matches = 0;
    for _ in 1..number_of_matches + 1 {
        let random_player_index = rng.gen::<usize>() % number_of_players as usize;
        if find_matching_player(&ordered_skip_list, random_player_index).is_some() {
            found_matches += 1;
        }
    }

    println!(
        "time to find {} matches: {}",
        number_of_matches,
        begin.elapsed().as_micros()
    );
    println!("found: {}", found_matches);
}
