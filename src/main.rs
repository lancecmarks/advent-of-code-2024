mod util;
mod day01;
mod day02;
mod day03;
mod day04;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let session_cookie = env::var("SESSION_COOKIE").expect("SESSION_COOKIE must be set");

    // Solve puzzles
    // day01::solve1(&session_cookie);
    // day01::solve2(&session_cookie);
    // day02::solve1(&session_cookie);
    //day02::solve2(&session_cookie);
    //day03::solve1(&session_cookie);
    //day03::solve2(&session_cookie);
    //day04::solve1(&session_cookie);
    day04::solve2(&session_cookie);
}
