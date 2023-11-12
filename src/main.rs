mod calorie_counter;
mod camp_cleanup;
mod rock_paper_scissors;
mod rucksack_reorganization;
mod supply_stacks;
mod tuning_tables;
mod no_space_left_on_device;
fn main() {
    let your_num = no_space_left_on_device::sum_sizes_of_directories();
    print!("Your num: {}", your_num);
}
