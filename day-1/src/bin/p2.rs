fn main() {
    let input = include_str!("./input-p2.txt");
    dbg!("{}", part2(input));
}

fn convert_numeric_str(s: &str) -> String {
    let mut nums = "".to_string();
    for (i, c) in s.chars().enumerate() {
        let start = &s[i..];

        if c.is_ascii_digit() {
            nums.push(c);
        }

        if start.starts_with("one") {
            nums.push('1');
        }
        if start.starts_with("two") {
            nums.push('2');
        }
        if start.starts_with("three") {
            nums.push('3');
        }
        if start.starts_with("four") {
            nums.push('4');
        }
        if start.starts_with("five") {
            nums.push('5');
        }
        if start.starts_with("six") {
            nums.push('6');
        }
        if start.starts_with("seven") {
            nums.push('7');
        }
        if start.starts_with("eight") {
            nums.push('8');
        }
        if start.starts_with("nine") {
            nums.push('9');
        }
    }

    nums
}

fn part2(input: &str) -> String {
    let ascii_digits: Vec<String> = input.lines().map(convert_numeric_str).collect();

    let sum = ascii_digits
        .iter()
        .map(|s| {
            let mut two_digit = String::new();

            two_digit.push(s.chars().next().expect("Unable to find first digit"));
            two_digit.push(s.chars().last().expect("Unable to find last digit"));

            two_digit.parse::<u32>().expect("Unable to parse number")
        })
        .sum::<u32>();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_numeric_str_works() {
        assert_eq!(convert_numeric_str("eightwothree"), "823");
        assert_eq!(convert_numeric_str("zoneight234"), "18234");
    }

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen",
        );
        assert_eq!(result, "281");
    }
}
