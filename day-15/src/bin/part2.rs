fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    const NEW_VEC: Vec<(&str, usize)> = Vec::new();
    let mut boxes = [NEW_VEC; 256];

    for step in input.split(',') {
        if step.contains('=') {
            let mut parts = step.split('=');
            let label = parts.next().unwrap().trim();
            let focal_length = parts.next().unwrap().parse().unwrap();
            let box_of_lens = &mut boxes[hash(label)];
            if let Some(i) = box_of_lens.iter().position(|v| v.0 == label) {
                box_of_lens[i].1 = focal_length;
            } else {
                box_of_lens.push((label, focal_length));
            }
        } else {
            let label = step.trim_end_matches('-');
            let box_of_lens = &mut boxes[hash(label)];
            if let Some(i) = box_of_lens.iter().position(|v| v.0 == label) {
                box_of_lens.remove(i);
            }
        }
    }

    calculate_focusing_power(&boxes)
}

fn calculate_focusing_power(boxes: &[Vec<(&str, usize)>; 256]) -> usize {
    let mut power = 0;
    for (i, box_values) in boxes.iter().enumerate() {
        for (j, value) in box_values.iter().enumerate() {
            power += (i + 1) * (j + 1) * value.1;
        }
    }
    power
}

fn hash(s: &str) -> usize {
    s.chars()
        .map(|c| c as usize)
        .fold(0, |acc, v| (acc + v) * 17 % 256)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 145);
    }
}
