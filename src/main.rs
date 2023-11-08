mod calorie_counter;
mod camp_cleanup;
mod rock_paper_scissors;
mod rucksack_reorganization;
mod supply_stacks;
fn main() {
    let your_num = supply_stacks::get_top_crates_part_two();
    print!("Your num: {}", your_num);
}
