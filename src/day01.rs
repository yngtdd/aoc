type Elf = u32;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Elf> {
    input
        .split("\n\n")
        .map(|bag| {
            bag.lines()
                .filter_map(|food| food.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect()
}

/// Get the largest number of calories collected by a single elf
#[aoc(day1, part1)]
pub fn max_calories(input: &[Elf]) -> u32 {
    let mut elves = input.to_vec();
    elves.sort();
    elves.reverse();
    elves[0]
}

/// Find the total calories collected by the top three gatherers
#[aoc(day1, part2)]
pub fn top_three_calories(input: &[Elf]) -> u32 {
    let mut elves = input.to_vec();
    elves.sort_by(|a, b| b.cmp(a));
    elves.iter().take(3).sum()
}
