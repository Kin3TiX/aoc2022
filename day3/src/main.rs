fn main() {

    let input_data = helpers::pathing::get_input_data("resources/day3.txt");
    let rucksacks: Vec<&str> = input_data.split("\n").collect();
    let item_priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut total_priority = 0;
    for rucksack in rucksacks.iter() {
        let clean_rucksack = rucksack.trim();
        let split_index = clean_rucksack.len() / 2;
        let (pocket1, pocket2) = clean_rucksack.split_at(split_index);
        let shared_item = helpers::strings::first_shared_character(pocket1, pocket2);
        let item_priority = item_priorities.find(shared_item)
            .expect("Could not prioritize shared item");
        total_priority += item_priority + 1;
    }

    println!("Total priority of items in both pockets is {}", total_priority);

    let mut badge_priority = 0;
    for rucksack_group in rucksacks.chunks(3) {
        let clean_rucksacks = rucksack_group
            .iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        let largest_rucksack = clean_rucksacks
            .iter()
            .max_by_key(|x| x.len()).unwrap();
        let smallest_rucksack = clean_rucksacks
            .iter()
            .min_by_key(|x| x.len()).unwrap();

        let shared_item = helpers::strings::first_shared_character(
            largest_rucksack, smallest_rucksack);
        let item_priority = item_priorities.find(shared_item)
            .expect("Could not prioritize shared item");
        badge_priority += item_priority + 1;
    }

    println!("Total priority of badges is {}", badge_priority);

}
