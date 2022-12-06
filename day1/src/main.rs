fn main() {

    let input_data = helpers::get_input_data("resources/day1.txt");
    let split_by_elf: Vec<&str> = input_data.split("\n\n").collect();

    let mut elf_calories = Vec::new();
    for (index, elf) in split_by_elf.iter().enumerate() {
        let mut total_calories = 0;
        for string_calories in elf.lines() {
            total_calories += string_calories.parse::<i32>().unwrap();
        }
        elf_calories.push(total_calories);
        println!("{} - {:?} - {:?}", index + 1, total_calories, elf);
    }

    let mut total_top_three_calories = 0;
    for n in 1..4 {

        let elf_with_most = elf_calories
            .iter()
            .enumerate()
            .max_by_key(|(_idx, &val)| val)
            .map(|(idx, _val)| idx)
            .unwrap_or(0);
        let calorie_count = &elf_calories[elf_with_most];

        println!(
            "Elf number {:?} is ranked {} in calories, with {:?}",
            elf_with_most + 1,
            n,
            calorie_count);

        total_top_three_calories += calorie_count;
        elf_calories.remove(elf_with_most);

    }

    println!(
        "The top 3 elves are carrying {:?} calories",
        total_top_three_calories);

}
