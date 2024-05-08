use std::str::FromStr;

fn main() {
    let input = include_str!("./input-p1.txt");

    dbg!("{}", part1(input));
}

#[derive(Debug, PartialEq, Eq)]
enum GameOption {
    Red(u16),
    Blue(u16),
    Green(u16),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameSetError;

impl FromStr for GameOption {
    type Err = ParseGameSetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let n = parts.first().and_then(|x| x.parse::<u16>().ok());
        let ball_type = parts.get(1).map(|x| *x);

        if n.is_none() || ball_type.is_none() {
            return Err(ParseGameSetError);
        }
        match ball_type {
            Some("red") => Ok(GameOption::Red(n.unwrap())),
            Some("blue") => Ok(GameOption::Blue(n.unwrap())),
            Some("green") => Ok(GameOption::Green(n.unwrap())),
            _ => Err(ParseGameSetError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u16,
    sets: Vec<GameOption>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;
fn part1(input: &str) -> String {
    todo!()
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let n = parts.first().and_then(|x| x.parse::<u16>().ok());
        let ball_type = parts.get(1).map(|x| *x);

        if n.is_none() || ball_type.is_none() {
            return Err(ParseGameError);
        }
        Ok(Game {
            id: n.unwrap(),
            sets: parse_multi_parts(ball_type.unwrap()),
        })
    }
}

fn parse_multi_parts(s: &str) -> Vec<GameOption> {
    s.split(',')
        .map(|x| x.trim())
        .flat_map(GameOption::from_str)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
        );
        assert_eq!(result, "8");
    }

    #[test]
    fn parse_game_test() {
        assert_eq!(GameOption::from_str("3 blue").unwrap(), GameOption::Blue(3));
        assert_eq!(GameOption::from_str("5 red").unwrap(), GameOption::Red(5));
        assert_eq!(
            GameOption::from_str("3 green").unwrap(),
            GameOption::Green(3)
        );
    }

    #[test]
    fn parse_multi_parts_test() {
        assert_eq!(
            parse_multi_parts("3 blue, 4 red"),
            vec![GameOption::Blue(3), GameOption::Red(4)]
        );
        assert_eq!(
            parse_multi_parts("1 red, 2 green, 6 blue"),
            vec![
                GameOption::Red(1),
                GameOption::Green(2),
                GameOption::Blue(6)
            ]
        );
    }
}