#[derive(Debug)]
enum Instruction {
    Move(usize),
    TurnRight,
    TurnLeft,
}
pub fn part_one(input: &str) -> Option<usize> {
    let board: Vec<Vec<char>> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let path = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .next()
        .unwrap();
    // parse path
    let instructions = {
        let mut instructions = Vec::new();
        let mut num = 0;
        for c in path.chars() {
            if c.is_ascii_digit() {
                num = num * 10 + c.to_digit(10).unwrap();
            } else {
                if num != 0 {
                    instructions.push(Instruction::Move(num as usize));
                    num = 0;
                }
                match c {
                    'R' => instructions.push(Instruction::TurnRight),
                    'L' => instructions.push(Instruction::TurnLeft),
                    _ => unreachable!(),
                }
            }
        }
        if num != 0 {
            instructions.push(Instruction::Move(num as usize));
        }
        instructions
    };

    let mut x = board[0].iter().position(|&c| c != ' ').unwrap() as usize;
    let mut y = 0usize;
    let mut direction = 0; // 0 = right, 1 = down, 2 = left, 3 = up
    for instruction in instructions {
        match instruction {
            Instruction::Move(count) => {
                for _ in 0..count {
                    let mut nx = x as isize;
                    let mut ny = y as isize;
                    match direction {
                        0 => {
                            nx += 1;
                            if nx >= board[ny as usize].len() as isize {
                                nx = board[y]
                                    .iter()
                                    .position(|&ch| ch != ' ')
                                    .expect("has to find new start")
                                    as isize;
                            }
                        }
                        1 => {
                            ny += 1;
                            if ny >= board.len() as isize
                                || board[ny as usize].len() <= x
                                || board[ny as usize][x as usize] == ' '
                            {
                                ny = board
                                    .iter()
                                    .position(|row| row.len() > x && row[x] != ' ')
                                    .unwrap() as isize;
                            }
                        }
                        2 => {
                            nx -= 1;
                            if nx < 0 || board[y][nx as usize] == ' ' {
                                nx = board[y]
                                    .iter()
                                    .rposition(|&ch| ch != ' ')
                                    .expect("has to find new start")
                                    as isize;
                            }
                        }

                        3 => {
                            ny -= 1;
                            if ny < 0
                                || board[ny as usize].len() <= x
                                || board[ny as usize][x] == ' '
                            {
                                ny = board
                                    .iter()
                                    .rposition(|row| row.len() > x && row[x] != ' ')
                                    .unwrap() as isize;
                            }
                        }
                        _ => unreachable!("Direction wrong..."),
                    }
                    // check if we can move
                    if board[ny as usize][nx as usize] == '#' {
                        break;
                    }

                    // lock in the move
                    x = nx as usize;
                    y = ny as usize;
                }
            }
            Instruction::TurnRight => direction = (direction + 1) % 4,
            Instruction::TurnLeft => direction = (direction + 3) % 4,
        }
    }
    //1000 * 6 + 4 * 8 + 0: 6032
    x += 1;
    y += 1;
    Some(1000 * y + 4 * x + direction)
}

pub fn part_two(input: &str) -> Option<isize> {
    type Grid<T> = Vec<Vec<T>>;
    type Edge = (/*x*/ usize, /*y*/ usize, /*direction*/ usize);
    type Chunk = (Vec<Vec<char>>, [Option<Edge>; 4]);

    let board: Grid<char> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let path = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .next()
        .unwrap();
    // parse path
    let instructions = {
        let mut instructions = Vec::new();
        let mut num = 0;
        for c in path.chars() {
            if c.is_ascii_digit() {
                num = num * 10 + c.to_digit(10).unwrap();
            } else {
                if num != 0 {
                    instructions.push(Instruction::Move(num as usize));
                    num = 0;
                }
                match c {
                    'R' => instructions.push(Instruction::TurnRight),
                    'L' => instructions.push(Instruction::TurnLeft),
                    _ => unreachable!(),
                }
            }
        }
        if num != 0 {
            instructions.push(Instruction::Move(num as usize));
        }
        instructions
    };
    // group grid in 50x50 chunks
    const CHUNK_SIZE: usize = 50; // CHANGE FOR EXAMPLE INPUT
    const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;
    let mut chunks: Grid<Option<Chunk>> = vec![vec![None; 4]; 4];
    let mut x = 0;
    let mut y = 0;
    while y < board.len() {
        let mut chunk = vec![vec![' '; CHUNK_SIZE]; CHUNK_SIZE];
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                if y + i < board.len() && x + j < board[y + i].len() {
                    chunk[i][j] = board[y + i][x + j];
                }
            }
        }
        if chunk[0][0] != ' ' {
            chunks[y / CHUNK_SIZE][x / CHUNK_SIZE] = Some((chunk, [None; 4]));
        }
        x += CHUNK_SIZE;
        if x >= board[y].len() {
            x = 0;
            y += CHUNK_SIZE;
        }
    }

    // find direct connections
    for y in 0..chunks.len() {
        for x in 0..chunks[y].len() {
            if let Some((_, [c_right, c_down, c_left, c_up])) =
                unsafe { &mut *(&mut chunks[y][x] as *mut Option<Chunk>) }
            {
                if x + 1 < chunks[y].len() && chunks[y][x + 1].is_some() {
                    *c_right = Some((x + 1, y, 0usize));
                }
                if y + 1 < chunks.len() && chunks[y + 1][x].is_some() {
                    *c_down = Some((x, y + 1, 0usize));
                }
                if x > 0 && chunks[y][x - 1].is_some() {
                    *c_left = Some((x - 1, y, 0usize));
                }
                if y > 0 && chunks[y - 1][x].is_some() {
                    *c_up = Some((x, y - 1, 0usize));
                }
            }
        }
    }

    // find connections through cube net folds
    let mut changed = true;
    while changed {
        changed = false;

        for y in 0..chunks.len() {
            for x in 0..chunks[y].len() {
                if let Some((_, [c_right, c_down, c_left, c_up])) =
                    unsafe { &mut *(&mut chunks[y][x] as *mut Option<Chunk>) }
                {
                    if x == 2 && y == 1 {
                        println!("{c_right:?} {c_down:?} {c_left:?} {c_up:?}");
                    }
                    match (c_right, c_down, c_left, c_up) {
                        (Some(c_right), Some(c_down), _, _)
                            if {
                                chunks[c_right.1][c_right.0].as_ref().unwrap().1
                                    [(1 + c_right.2) % 4]
                                    .is_none()
                                    && chunks[c_down.1][c_down.0].as_ref().unwrap().1[c_down.2 % 4]
                                        .is_none()
                            } =>
                        {
                            changed = true;
                            chunks[c_right.1][c_right.0].as_mut().unwrap().1[(1 + c_right.2) % 4] =
                                Some((c_down.0, c_down.1, c_down.2 + (4 - c_right.2 % 4) + 1));
                            chunks[c_down.1][c_down.0].as_mut().unwrap().1[c_down.2 % 4] =
                                Some((c_right.0, c_right.1, c_right.2 + (4 - c_down.2 % 4) + 3));
                        }
                        (_, Some(c_down), Some(c_left), _)
                            if {
                                chunks[c_down.1][c_down.0].as_ref().unwrap().1[(2 + c_down.2) % 4]
                                    .is_none()
                                    && chunks[c_left.1][c_left.0].as_ref().unwrap().1
                                        [(1 + c_left.2) % 4]
                                        .is_none()
                            } =>
                        {
                            changed = true;
                            chunks[c_down.1][c_down.0].as_mut().unwrap().1[(2 + c_down.2) % 4] =
                                Some((c_left.0, c_left.1, c_left.2 + (4 - c_down.2 % 4) + 1));
                            chunks[c_left.1][c_left.0].as_mut().unwrap().1[(1 + c_left.2) % 4] =
                                Some((c_down.0, c_down.1, c_down.2 + (4 - c_left.2 % 4) + 3));
                        }
                        (_, _, Some(c_left), Some(c_up))
                            if {
                                chunks[c_left.1][c_left.0].as_ref().unwrap().1[(3 + c_left.2) % 4]
                                    .is_none()
                                    && chunks[c_up.1][c_up.0].as_ref().unwrap().1[(2 + c_up.2) % 4]
                                        .is_none()
                            } =>
                        {
                            changed = true;
                            chunks[c_left.1][c_left.0].as_mut().unwrap().1[(3 + c_left.2) % 4] =
                                Some((c_up.0, c_up.1, c_up.2 + (4 - c_left.2 % 4) + 1));
                            chunks[c_up.1][c_up.0].as_mut().unwrap().1[(2 + c_up.2) % 4] =
                                Some((c_left.0, c_left.1, c_left.2 + (4 - c_up.2 % 4) + 3));
                        }
                        (Some(c_right), _, _, Some(c_up))
                            if {
                                chunks[c_up.1][c_up.0].as_ref().unwrap().1[c_up.2 % 4].is_none()
                                    && chunks[c_right.1][c_right.0].as_ref().unwrap().1
                                        [(3 + c_right.2) % 4]
                                        .is_none()
                            } =>
                        {
                            changed = true;
                            chunks[c_up.1][c_up.0].as_mut().unwrap().1[c_up.2 % 4] =
                                Some((c_right.0, c_right.1, c_right.2 + (4 - c_up.2 % 4) + 1));
                            chunks[c_right.1][c_right.0].as_mut().unwrap().1[(3 + c_right.2) % 4] =
                                Some((c_up.0, c_up.1, c_up.2 + (4 - c_right.2 % 4) + 3));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // main loop
    let mut x = board[0].iter().position(|&c| c != ' ').unwrap() as isize;
    let mut y = 0isize;
    let mut direction = 0; // 0 = right, 1 = down, 2 = left, 3 = up
    for instr in instructions {
        match instr {
            Instruction::Move(n) => {
                for _ in 0..n {
                    let (mut nx, mut ny) = match direction {
                        0 => (x + 1, y),
                        1 => (x, y + 1),
                        2 => (x - 1, y),
                        3 => (x, y - 1),
                        _ => unreachable!(),
                    };
                    let mut nd = direction;

                    // check if we need to change chunk
                    if nx < 0
                        || ny < 0
                        || ny >= board.len() as isize
                        || nx >= board[ny as usize].len() as isize
                        || board[ny as usize][nx as usize] == ' '
                    {
                        // find connection
                        let (cx, cy) = (x / CHUNK_SIZE_I, y / CHUNK_SIZE_I);
                        let chunk = chunks[cy as usize][cx as usize].as_ref().unwrap();
                        let (ncx, ncy, ndir) = chunk.1[direction].unwrap();
                        let ncx = ncx as isize;
                        let ncy = ncy as isize;

                        // calculate edge offset
                        let edge_offset = match direction {
                            0 => ny - cy * CHUNK_SIZE_I,
                            1 => (cx + 1) * CHUNK_SIZE_I - nx - 1,
                            2 => (cy + 1) * CHUNK_SIZE_I - ny - 1,
                            3 => nx - cx * CHUNK_SIZE_I,
                            _ => unreachable!(),
                        };

                        // calculate new position
                        (nx, ny) = match (direction + ndir) % 4 {
                            0 => (ncx * CHUNK_SIZE_I, ncy * CHUNK_SIZE_I + edge_offset),
                            1 => (
                                (ncx + 1) * CHUNK_SIZE_I - edge_offset - 1,
                                ncy * CHUNK_SIZE_I,
                            ),
                            2 => (
                                (ncx + 1) * CHUNK_SIZE_I - 1,
                                (ncy + 1) * CHUNK_SIZE_I - edge_offset - 1,
                            ),
                            3 => (
                                ncx * CHUNK_SIZE_I + edge_offset,
                                (ncy + 1) * CHUNK_SIZE_I - 1,
                            ),
                            _ => unreachable!(),
                        };
                        nd = (direction + ndir) % 4;
                    }

                    // check if we can move
                    if board[ny as usize][nx as usize] == '#' {
                        break;
                    }

                    // move
                    x = nx;
                    y = ny;
                    direction = nd;
                }
            }
            Instruction::TurnRight => {
                direction = (direction + 1) % 4;
            }
            Instruction::TurnLeft => {
                direction = (direction + 3) % 4;
            }
        }
    }

    // calculate result
    let x = x + 1;
    let y = y + 1;
    Some(y * 1000 + x * 4 + direction as isize)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}