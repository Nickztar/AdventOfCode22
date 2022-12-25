pub fn part_one(input: &str) -> Option<String> {
    let decimal_number: i64 = input.lines().map(|line| {
        line.chars().rev().enumerate().map(|(pos,ch)| {
            let pos_value = if pos == 0 {
                1
            } else {
                5usize.pow(pos as u32)
            } as i64;
            match ch {
                '2' => {
                    2 * pos_value
                }
                '1' => {
                    pos_value
                }
                '0' => {
                    0
                },
                '-' => {
                    -1 * pos_value
                }
                '=' => {
                    -2 * pos_value
                }
                _ => unreachable!("Should parse")
            }
        }).collect::<Vec<i64>>().iter().sum::<i64>()
    }).sum();
    let snafu = to_snafu(decimal_number); 
    Some(snafu)
}

fn to_snafu(mut number: i64) -> String {
    let mut snafu: Vec<char> = Vec::new();
    while number > 0 {
        let (new_number, rem) = div_rem(number, 5);
        number = new_number;
        snafu.push(match rem {
            0 => {
                '0'
            },
            1 => {
                '1'
            },
            2 => {
                '2'
            },
            3 => {
                '='
            },
            4 => {
                '-'
            },
            _ => unreachable!("Should not be hit") 
        });
        if rem > 2 {
            number += 1
        }
    }
    snafu.iter().rev().collect()
}

fn div_rem(num: i64, other: i64) -> (i64, i64) {
    (num / other, num % other)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }

    #[test]
    #[ignore = "doesn't exist"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
