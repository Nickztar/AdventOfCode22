
#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock, 
    Paper,
    Scissor,
    Win,
    Lose,
    Draw
}

pub fn part_one(input: &str) -> Option<u32> {
    let matches: Vec<Vec<RPS>> = input.lines().map(|line| line.split(" ").map(|play| match play {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissor,
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissor,
        _ => panic!("Shoud not have missed?")
    }).collect()).collect();
    let mut score: u32 = 0;

    let count = matches.len();
    for game in matches {
        let opponent = game[0];
        let me = game[1];
        //shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors
        //outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
        let res: u32 = match (opponent, me) {
            (RPS::Rock, RPS::Rock) => 1 + 3, //Tie + Rock
            (RPS::Rock, RPS::Paper) =>  2 + 6, //Win + Paper
            (RPS::Rock, RPS::Scissor) =>  3 + 0, //Loss + Scissor
            (RPS::Paper, RPS::Rock) =>  1 + 0, //Loss + Rock
            (RPS::Paper, RPS::Paper) =>  2 + 3, //Tie + Paper
            (RPS::Paper, RPS::Scissor) =>  3 + 6, //Loss + Scissor
            (RPS::Scissor, RPS::Rock) =>  1 + 6, //Win + Rock
            (RPS::Scissor, RPS::Paper) =>  2 + 0, //Loss + Paper
            (RPS::Scissor, RPS::Scissor) =>  3 + 3, //Tie + Scissor
            _ => 0
        };
        score += res;
        println!("O: {:?}, M: {:?}, Res: {res}, Score: {score}", opponent, me);
    }
    println!("Total matches: {count}");
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matches: Vec<(RPS, RPS)> = input.lines().map(|line| line.split(" ").map(|play| match play {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissor,
        "X" => RPS::Lose,
        "Y" => RPS::Draw,
        "Z" => RPS::Win,
        _ => panic!("Shoud not have missed?")
    }).collect()).map(|games: Vec<RPS>| (games[0], games[1])).collect();
    let mut score: u32 = 0;
    for game in matches {
        //shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors
        //outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
        let res: u32 = match game {
            (RPS::Rock, RPS::Win) => 2 + 6, //Choose paper to win
            (RPS::Rock, RPS::Lose) => 3 + 0, //Choose scissor to lose
            (RPS::Rock, RPS::Draw) => 1 + 3, //Choose rock to draw
            (RPS::Paper, RPS::Win) => 3 + 6, //Choose scissor to win
            (RPS::Paper, RPS::Lose) => 1 + 0, //Choose rock to lose
            (RPS::Paper, RPS::Draw) => 2 + 3, //Choose paper to draw
            (RPS::Scissor, RPS::Win) => 1 + 6, //Choose rock to win
            (RPS::Scissor, RPS::Lose) => 2 + 0, //Choose paper to lose
            (RPS::Scissor, RPS::Draw) => 3 + 3, //Choose scissor to draw
            _ => 0
        };
        score += res;
        // println!("O: {:?}, M: {:?}, Res: {res}, Score: {score}", opponent, me);
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
