fn main() {
    let input = include_str!("./input-p1.txt");

    dbg!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let ascii_digits: Vec<String> = input
        .lines()
        .map(|s| {
            let mut nums = "".to_string();
            for c in s.chars() {
                if c.is_ascii_digit() {
                    nums.push(c)
                }
            }
            nums
        })
        .collect();

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
    fn it_works() {
        let result = part1(
            r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#,
        );
        assert_eq!(result, "142");
    }
}
