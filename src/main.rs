mod calorie_counter;
mod rock_paper_scissors;
mod rucksack_reorganization;
mod camp_cleanup;
fn main() {
    let your_num = camp_cleanup::cleanup_the_camp_part_two();
    print!("Your num: {}", your_num);
}
