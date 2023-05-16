use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{Rng, thread_rng};
use rust_data_structure::skipped_list::{construct_queue, find_matching_player};
use rust_data_structure::vec_queue::{old_construct_queue, old_find_matching_player};

pub fn criterion_benchmark(c: &mut Criterion) {
    let number_of_players = std::env::var("PLAYERS").unwrap_or(String::from("10000")).parse::<usize>().unwrap();
    let queue = construct_queue(number_of_players);
    let mut rng = thread_rng();
    c.bench_function("find_matching_player", |b| {
        b.iter(|| {
            let random_player_index = rng.gen::<usize>() % number_of_players;
            find_matching_player(&queue, random_player_index);
        })
    });

    let old_queue = old_construct_queue(number_of_players);
    c.bench_function("old_find_matching_player", |b| {
        b.iter(||{
            let random_player_index = rng.gen::<usize>() % number_of_players;
            old_find_matching_player(&old_queue, random_player_index)
        })
    });

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
