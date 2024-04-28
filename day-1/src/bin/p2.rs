fn main() {
    let input = include_str!("./input-p2.txt");
    dbg!("{}", part2(input));
}

fn part2(input: &str) -> String {
    dbg!("{}", input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(result, "4");
    }
}
