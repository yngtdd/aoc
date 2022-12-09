type Elf = u32;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Elf> {
    input
        .split("\n\n")
        .map(|food| {
            food.lines()
                .map(|item| item.parse::<u32>().unwrap())
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
    elves.sort();
    elves.reverse();

    elves[..3].iter().sum()
}
