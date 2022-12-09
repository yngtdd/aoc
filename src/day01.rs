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

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Elf]) -> u32 {
    let mut elves = input.to_vec();
    elves.sort();
    elves.reverse();
    elves[0]
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Elf]) -> u32 {
    let mut elves = input.to_vec();
    elves.sort_by(|a, b| b.cmp(a));
    elves.iter().take(3).sum()
}
