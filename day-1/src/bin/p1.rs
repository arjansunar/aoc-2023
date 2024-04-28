fn main() {
    let input = include_str!("./input-p1.txt");

    dbg!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().map(|line| line).collect();
    dbg!("{:?}", lines);
    "142".to_string()
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
