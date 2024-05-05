use std::str::FromStr;

fn main() {
    let input = include_str!("./input-p1.txt");

    dbg!("{}", part1(input));
}

#[derive(Debug, PartialEq, Eq)]
enum GameSet {
    Red(u16),
    Blue(u16),
    Green(u16),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameSetError;

impl FromStr for GameSet {
    type Err = ParseGameSetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let n = parts.first().and_then(|x| x.parse::<u16>().ok());
        let ball_type = parts.get(1).map(|x| *x);

        if n.is_none() || ball_type.is_none() {
            return Err(ParseGameSetError);
        }
        match ball_type {
            Some("red") => Ok(GameSet::Red(n.unwrap())),
            Some("blue") => Ok(GameSet::Blue(n.unwrap())),
            Some("green") => Ok(GameSet::Green(n.unwrap())),
            _ => Err(ParseGameSetError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u16,
    sets: Vec<GameSet>
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
            return Err(ParseGameSetError);
        }
        match ball_type {
            Some("red") => Ok(GameSet::Red(n.unwrap())),
            Some("blue") => Ok(GameSet::Blue(n.unwrap())),
            Some("green") => Ok(GameSet::Green(n.unwrap())),
            _ => Err(ParseGameSetError),
        }
    }
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
        assert_eq!(GameSet::from_str("3 blue").unwrap(), GameSet::Blue(3));
        assert_eq!(GameSet::from_str("5 red").unwrap(), GameSet::Red(5));
        assert_eq!(GameSet::from_str("3 green").unwrap(), GameSet::Green(3));
    }
}
