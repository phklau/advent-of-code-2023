use std::cmp;

struct Game {
    pub game_rounds: Vec<GameRound>,
    limits: Limits,
    min_limits: Limits,
    pub number: u32,
}

impl Game {
    pub fn new(input: &str, limits: &Limits) -> Self {
        // parse game number
        let subinput: Vec<&str> = input.split(':').collect();
        let game_nr_str: Vec<&str> = subinput[0].split_whitespace().collect();
        let game_nr: u32 = game_nr_str[1].to_string().parse::<u32>().unwrap();

        // parse rounds
        let mut game_rounds: Vec<GameRound> = Vec::new();
        let rounds_strings: Vec<&str> = subinput[1].split(';').collect();
        for round_string in rounds_strings {
            game_rounds.push(GameRound::new(round_string));
        }
        let min_limits = Limits {
            red: game_rounds.first().unwrap().red,
            blue: game_rounds.first().unwrap().blue,
            green: game_rounds.first().unwrap().green,
        };
        Game {
                game_rounds,
                limits: limits.clone(),
                min_limits,
                number: game_nr,
             }
    }

    pub fn is_possible(&self) -> bool {
        let mut possible: bool = true;
        for round in &self.game_rounds {
            possible &= round.is_possible(&self.limits);
        }
        possible
    }

    fn calc_min_limit(&mut self) {
        for round in &self.game_rounds {
            self.min_limits.red = cmp::max(self.min_limits.red, round.red);
            self.min_limits.green = cmp::max(self.min_limits.green, round.green);
            self.min_limits.blue = cmp::max(self.min_limits.blue, round.blue);
        }
    }

    pub fn calc_power(&mut self) -> u16 {
        self.calc_min_limit();
        self.min_limits.red * self.min_limits.green * self.min_limits.blue
    }
}

struct GameRound {
   pub green: u16,
   pub blue: u16,
   pub red: u16,
}

impl GameRound {
    pub fn new(round_string: &str) -> Self {
        let mut blue: u16 = 0;
        let mut green: u16 = 0;
        let mut red: u16 = 0;
        for color in round_string.split(',') {
            if color.find("blue") != None {
                let blue_string: Vec<&str> = color.split_whitespace().collect();
                blue = blue_string[0].to_string().parse::<u16>().unwrap();
            }
            else if color.find("green") != None {
                let green_string: Vec<&str> = color.split_whitespace().collect();
                green = green_string[0].to_string().parse::<u16>().unwrap();
            }
            else if color.find("red") != None {
                let red_string: Vec<&str> = color.split_whitespace().collect();
                red = red_string[0].to_string().parse::<u16>().unwrap();
            }
        }
        Self {
            blue,
            green,
            red,
        }
    }
    pub fn is_possible(&self, limits: &Limits) -> bool {
        self.green <= limits.green && self.blue <= limits.blue && self.red <= limits.red
    }
}

#[derive(Clone)]
struct Limits {
   green: u16,
   blue: u16,
   red: u16,
}

fn part1(input: &str) -> u32
{
    // Limits
    let limits = Limits{blue: 14, green: 13, red: 12};
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        games.push(Game::new(line, &limits));
    }
    let mut possible_sum: u32 = 0;
    for game in games {
      if game.is_possible() {
          possible_sum += game.number;
      }
    }
    possible_sum
}

fn part2(input: &str) -> u32
{
    // Limits
    let limits = Limits{blue: 14, green: 13, red: 12};
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        games.push(Game::new(line, &limits));
    }
    let mut power_sum: u32 = 0;
    for mut game in games {
          power_sum += u32::from(game.calc_power());
    }
    power_sum
}

fn main() {
    // parse text into games
    let input: &str = include_str!("./input.txt");
    let result = part1(input);
    let result2 = part2(input);
    println!("Result Part1: {}", result);
    println!("Result Part2: {}", result2);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gameround_is_possible_test()
    {
        let exmp_round = GameRound { green: 10, blue: 10, red: 10};
        let possible_limits: Limits = Limits {
            green: exmp_round.green+1,
            blue: exmp_round.blue+1,
            red: exmp_round.red+1,
        };
        let unpossible_limits: Limits = Limits{
            green: exmp_round.green-1,
            blue: exmp_round.blue-1,
            red: exmp_round.red-1,
        };
        assert_eq!(true, exmp_round.is_possible(&possible_limits));
        assert_eq!(false, exmp_round.is_possible(&unpossible_limits));
    }
    #[test]
    fn game_new_from_string()
    {
        let sample_input_line_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let sut_game = Game::new(sample_input_line_1, &Limits{blue:0, green:0, red:0} );
        assert_eq!(1, sut_game.number);
        assert_eq!(sut_game.game_rounds.len(), 3);
        assert_eq!(sut_game.game_rounds[0].blue, 3);
        assert_eq!(sut_game.game_rounds[0].green, 0);
        assert_eq!(sut_game.game_rounds[0].red, 4);
        assert_eq!(sut_game.game_rounds[1].blue, 6);
        assert_eq!(sut_game.game_rounds[1].green, 2);
        assert_eq!(sut_game.game_rounds[1].red, 1);
        assert_eq!(sut_game.game_rounds[2].blue, 0);
        assert_eq!(sut_game.game_rounds[2].green, 2);
        assert_eq!(sut_game.game_rounds[2].red, 0);
        let sample_input_line_2: &str = "Game 123: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let sut_game_2 = Game::new(sample_input_line_2, &Limits{blue:0, green:0, red:0});
        assert_eq!(123, sut_game_2.number);
    }
    #[test]
    fn gameround_from_string() {
        let sample_input_1 = " 3 blue, 4 red";
        let sut_gameround_1 = GameRound::new(sample_input_1);
        assert_eq!(sut_gameround_1.blue, 3);
        assert_eq!(sut_gameround_1.green, 0);
        assert_eq!(sut_gameround_1.red, 4);

        let sample_input_2 = " 1 green, 3 red, 6 blue";
        let sut_gameround_2 = GameRound::new(sample_input_2);
        assert_eq!(sut_gameround_2.blue, 6);
        assert_eq!(sut_gameround_2.green, 1);
        assert_eq!(sut_gameround_2.red, 3);
    }
    #[test]
    fn game_from_string_is_possible() {
        let sample_input_1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let limit: Limits = Limits { green: 13, blue: 14, red: 12 };
        let sut_game = Game::new(sample_input_1, &limit);
        assert_eq!(sut_game.is_possible(), true);

        let sample_input_not_possible = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let sut_game_not_possible = Game::new(sample_input_not_possible, &limit);
        assert_eq!(sut_game_not_possible.is_possible(), false);
    }
    #[test]
    fn task_example_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!(part1(input), 8);
    }
    #[test]
    fn task_example_input2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!(part2(input), 2286);
    }
}
