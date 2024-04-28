fn main() {
    let input = include_str!("./input-p1.txt");
    dbg!("{}", part1(input));
}

fn part1(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("test");
        assert_eq!(result, "4");
    }
}
