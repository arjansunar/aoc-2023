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
    sets: Vec<Vec<GameOption>>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;
fn part1(input: &str) -> String {
    let parsed = input.lines().map(Game::from_str).collect::<Vec<_>>();
    let valid_games = parsed
        .into_iter()
        .filter(|x| x.as_ref().map(is_valid_game).unwrap_or(false))
        .collect::<Vec<_>>();

    let total = valid_games
        .iter()
        .map(|x| x.as_ref().unwrap().id)
        .sum::<u16>();

    total.to_string()
}

fn is_valid_game(game: &Game) -> bool {
    game.sets.iter().all(is_valid_set)
}

const RED_LIMIT: u16 = 12;
const GREEN_LIMIT: u16 = 13;
const BLUE_LIMIT: u16 = 14;

fn is_valid_set(set: &Vec<GameOption>) -> bool {
    for x in set {
        let is_within_limit: bool = match *x {
            GameOption::Red(r) => r <= RED_LIMIT,
            GameOption::Blue(b) => b <= BLUE_LIMIT,
            GameOption::Green(g) => g <= GREEN_LIMIT,
        };
        if !is_within_limit {
            return false;
        }
    }
    true
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(':').collect::<Vec<_>>();

        let game_id = parts
            .first()
            .copied()
            .and_then(|x| x.split(' ').collect::<Vec<_>>().get(1).copied())
            .and_then(|x| x.parse::<u16>().ok())
            .ok_or(ParseGameError)?;

        let ball_type = parts.get(1).map(|x| x.trim()).ok_or(ParseGameError);
        let sets = ball_type
            .map(|x| x.split(';').collect::<Vec<_>>())
            .map(|x| x.iter().map(|&x| parse_multi_parts(x)).collect::<Vec<_>>())?;

        Ok(Game { id: game_id, sets })
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

    #[test]
    fn parse_single_game() {
        assert_eq!(
            Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok(Game {
                id: 1,
                sets: vec![
                    vec![GameOption::Blue(3), GameOption::Red(4)],
                    vec![
                        GameOption::Red(1),
                        GameOption::Green(2),
                        GameOption::Blue(6)
                    ],
                    vec![GameOption::Green(2)]
                ]
            })
        );
    }
    #[test]
    fn is_valid_set_test() {
        assert!(is_valid_set(&vec![
            GameOption::Red(1),
            GameOption::Green(2),
            GameOption::Blue(6)
        ]))
    }

    #[test]
    fn is_invalid_set_test() {
        assert!(!is_valid_set(&vec![
            GameOption::Red(100),
            GameOption::Green(2),
            GameOption::Blue(6)
        ]))
    }
}
