use rand::Rng;
use skiplist::OrderedSkipList;
use std::cmp::Ordering;
use std::collections::Bound;
use std::time;
use uuid::Uuid;

pub struct PlayerScore {
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

pub fn find_matching_player(
    queue: &OrderedSkipList<PlayerScore>,
    player_index: usize,
) -> Option<&PlayerScore> {
    let selected_player = queue.get(player_index);
    if selected_player.is_none() {
        return None;
    }
    let selected_player = selected_player.unwrap();

    let threshold = std::env::var("THRESHOLD")
        .unwrap_or(String::from("500"))
        .parse::<i32>()
        .unwrap();

    let lower_bound = PlayerScore::new(String::new(), selected_player.score - threshold);
    let upper_bound= PlayerScore::new(String::new(), selected_player.score + threshold);
    for player in queue.range(Bound::Included(&lower_bound), Bound::Included(&upper_bound)) {
        return Some(player)
    }
    return None;
}

pub fn construct_queue(number_of_players: usize) -> OrderedSkipList<PlayerScore> {
    let mut ordered_skip_list: OrderedSkipList<PlayerScore> = OrderedSkipList::new();
    let mut rng = rand::thread_rng();
    let max_score = std::env::var("MAX_SCORE").unwrap_or(String::from("100000")).parse::<u32>().unwrap();
    for _ in 1..number_of_players {
        let score = rng.gen::<u32>() % max_score;
        let user_id = Uuid::new_v4().to_string();
        ordered_skip_list.insert(PlayerScore::new(user_id, score as i32));
    }
    return ordered_skip_list;
}

pub fn measure_time() -> u128 {
    let number_of_players = std::env::var("PLAYERS").unwrap().parse::<i32>().unwrap();
    let ordered_skip_list = construct_queue(number_of_players as usize);

    let number_of_matches = std::env::var("MATCHES").unwrap().parse::<i32>().unwrap();
    let mut rng = rand::thread_rng();

    let begin = time::Instant::now();
    for _ in 1..number_of_matches + 1 {
        let random_player_index = rng.gen::<usize>() % number_of_players as usize;
        let _ = find_matching_player(&ordered_skip_list, random_player_index).is_some();
    }
    return begin.elapsed().as_micros();
}
