use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let splitted = line.split_whitespace().collect::<Vec<_>>();

            let mut springs = (splitted[0].to_string() + "?").repeat(5);
            springs.pop();

            let nums = splitted[1].split(',').map(|s| s.parse().unwrap());
            let nums = std::iter::repeat(nums)
                .take(5)
                .flatten()
                .collect::<Vec<_>>();

            let mut cache = HashMap::new();
            count(&springs, &nums, &mut cache)
        })
        .sum()
}

fn count<'a>(
    springs: &'a str,
    nums: &'a [usize],
    cache: &mut HashMap<(&'a str, &'a [usize]), usize>,
) -> usize {
    if springs.is_empty() {
        return if nums.is_empty() { 1 } else { 0 };
    }

    if nums.is_empty() {
        return if springs.contains('#') { 0 } else { 1 };
    }

    let key = (springs, nums);
    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let mut result = 0;
    if springs.starts_with(['.', '?']) {
        result += count(&springs[1..], nums, cache);
    }

    if springs.starts_with(['#', '?']) {
        let cond = nums[0];
        if cond <= springs.len()
            && !springs[..cond].contains('.')
            && (cond == springs.len() || springs.chars().nth(cond) != Some('#'))
        {
            if cond == springs.len() {
                result += count(&springs[cond..], &nums[1..], cache);
            } else {
                result += count(&springs[cond + 1..], &nums[1..], cache);
            }
        }
    }

    cache.insert(key, result);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 525152);
    }
}
