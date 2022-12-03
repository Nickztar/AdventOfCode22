pub fn part_one(input: &str) -> Option<usize> {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let parts = input.lines().map(|line|{
        let first_part = line.chars().take(line.len() / 2).collect::<String>();

        let second_part = line.chars().skip(line.len() / 2).collect::<String>();
        (first_part, second_part)
    }).collect::<Vec<(String, String)>>();

    let mut score: usize = 0;
    for (left, right) in parts{
        let overlapped = left.chars().filter(|lc| right.contains(*lc)).nth(0);
        match overlapped {
            Some(c) => {
                score += alphabet.iter().position(|&a| a == c).unwrap() + 1;
            },
            None => todo!(),
        }
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let parts = input.lines().collect::<Vec<&str>>().chunks(3).map(|group| {
        //Not sure what i am doing here but i wanted to sort them?
        let mut owned_group = group.to_owned();
        owned_group.sort_by(|a, b| a.len().cmp(&b.len()));
        
        let common_char = owned_group[2].chars().filter(|c| owned_group[1].contains(*c) && owned_group[0].contains(*c)).nth(0).unwrap();
        //Lmao maybe i should read the prompt...
        // let concat = group.join("");
        // let matches = concat.matches(common_char).count();
        // println!("{matches}");
        // matches
        alphabet.iter().position(|&a| a == common_char).unwrap() + 1
    }).collect::<Vec<usize>>();
    // let mut score: usize = 0;
    Some(parts.iter().sum::<usize>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
