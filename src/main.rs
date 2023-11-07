mod calorie_counter;
mod rock_paper_scissors;

fn main() {
    let your_score = rock_paper_scissors::run_rps_tournament_part_two();
    print!("Your score: {}", your_score);
}
