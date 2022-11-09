fn main() {
    let inuput_nums = include_str!("fixtures/aoc2021_01.txt")
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let part1 = count_inc(&inuput_nums);
    let window_nums = inuput_nums
        .as_slice()
        .windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .collect::<Vec<i32>>();

    let part2 = count_inc(&window_nums);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn count_inc(nums: &[i32]) -> usize {
    nums.iter()
        .enumerate()
        .map(|(i, n)| {
            if i == 0 {
                return 0;
            }
            if n > &nums[i - 1] {
                return 1;
            }
            0
        })
        .filter(|x| x == &1)
        .count()
}
