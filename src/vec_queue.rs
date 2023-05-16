use crate::skipped_list::PlayerScore;
use rand::Rng;
use uuid::Uuid;

pub fn old_construct_queue(number_of_players: usize) -> Vec<PlayerScore> {
    let mut queue = vec![];
    let mut rng = rand::thread_rng();
    let max_score = std::env::var("MAX_SCORE").unwrap_or(String::from("100000")).parse::<u32>().unwrap();
    for _ in 1..number_of_players {
        let score = rng.gen::<u32>() % max_score;
        let user_id = Uuid::new_v4().to_string();
        queue.push(PlayerScore::new(user_id, score as i32))
    }
    return queue;
}

pub fn old_find_matching_player(
    queue: &Vec<PlayerScore>,
    player_index: usize,
) -> Option<&PlayerScore> {
    let threshold = std::env::var("THRESHOLD")
        .unwrap_or(String::from("500"))
        .parse::<i32>()
        .unwrap();
    let selected_player = queue.get(player_index);
    if selected_player.is_none() {
        return None;
    }

    let selected_player = selected_player.unwrap();
    for player in queue {
        if (player.score - selected_player.score).abs() <= threshold {
            return Some(player);
        }
    }
    None
}
