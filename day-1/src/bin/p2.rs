fn main() {
    let input = include_str!("./input-p2.txt");
    dbg!("{}", part2(input));
}

fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("test");
        assert_eq!(result, "4");
    }
}
