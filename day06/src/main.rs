use clap::Parser;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_hint(clap::ValueHint::FilePath))]
    input_file: PathBuf,
    #[arg(short, long)]
    display_solution: bool,
}

fn main() {
    let args = Args::parse();

    let file_contents = fs::read_to_string(args.input_file).expect("Unable to read file");
    let board = SituationMap::try_from(file_contents.as_ref()).expect("Expected a valid board");

    println!("{}", board);

    let part_1_answer = solve_part_1(board.clone(), args.display_solution);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = solve_part_2(board, args.display_solution);
    println!("Part 2: {part_2_answer}");
}

fn solve_part_2(mut board: SituationMap, show_blocks: bool) -> usize {
    let mut coords: HashSet<Coord> = HashSet::new();

    if board.step().is_none() {
        return 0;
    }

    loop {
        if let Some((_, &element)) = &board.what_is_in_front(&board.player) {
            if element == MapElements::Free {
                match board.test_circular_path(board.player.coords, board.player.orientation) {
                    Some(c) => {
                        coords.insert(c);
                    }
                    None => {}
                };
            }
        }

        // println!("{board}");

        if board.step().is_none() {
            break;
        }
    }

    coords.len()
}

fn solve_part_1(mut board: SituationMap, display_solution: bool) -> usize {
    if display_solution {
        println!("{board}")
    }

    let mut step_count: usize = 0;
    let detailed_prints = 30 > (board.map_height + board.map_width);

    loop {
        match board.step() {
            Some(step) => {
                if display_solution {
                    if detailed_prints || step_count % 8 == 0 {
                        println!("Stepped to {}, {}", step.row, step.col);
                        println!("{board}");
                    }
                    step_count += 1;
                }
            }
            None => break,
        }
    }
    board.seen_tiles()
}

#[derive(Clone)]
struct Player {
    orientation: Orientation,
    coords: Coord,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(PartialEq, Clone, Debug, Hash, Copy, Eq)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    fn rotate_right(&mut self) -> Self {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }
}

#[derive(Clone, PartialEq, Copy, Debug)]
enum MapElements {
    Free,
    PrevouslySeen,
    Obstructed,
}

#[derive(Clone)]
struct SituationMap {
    player: Player,
    map: Vec<MapElements>,
    map_width: usize,
    map_height: usize,
}

impl SituationMap {
    fn test_circular_path(
        &mut self,
        starting_at: Coord,
        orientation: Orientation,
    ) -> Option<Coord> {
        let mut virtual_player = Player {
            coords: starting_at.clone(),
            orientation: orientation.clone(),
        };

        let mut visited_tiles: HashSet<(Coord, Orientation)> = HashSet::new();
        let (old_location, &old_tile) = self.what_is_in_front(&virtual_player)?;

        self.set_at(&old_location, MapElements::Obstructed);

        loop {
            match self.what_is_in_front(&virtual_player) {
                Some((coord, element)) => match element {
                    MapElements::Free | MapElements::PrevouslySeen => {
                        virtual_player.coords = coord;
                    }
                    MapElements::Obstructed => {
                        virtual_player.orientation = virtual_player.orientation.rotate_right();

                        if !visited_tiles
                            .insert((virtual_player.coords, virtual_player.orientation))
                        {
                            self.set_at(&old_location, old_tile);
                            return Some(old_location.clone());
                        }
                    }
                },
                None => {
                    self.set_at(&old_location, old_tile);
                    return None;
                }
            };
        }
    }

    fn what_is_at(&self, coord: &Coord) -> Option<&MapElements> {
        if coord.col >= self.map_width {
            None
        } else if coord.row >= self.map_height {
            None
        } else {
            self.map.get((self.map_width * coord.row) + coord.col)
        }
    }

    fn set_at(&mut self, coord: &Coord, element: MapElements) {
        self.map[(self.map_width * coord.row) + coord.col] = element;
    }

    fn seen_tiles(&self) -> usize {
        self.map
            .iter()
            .filter(|&t| t == &MapElements::PrevouslySeen)
            .count()
    }

    fn what_is_in_front(&self, player: &Player) -> Option<(Coord, &MapElements)> {
        let coords_in_front = match player.orientation {
            Orientation::Up => {
                if player.coords.row == 0 {
                    return None;
                } else {
                    Some(Coord {
                        row: player.coords.row - 1,
                        col: player.coords.col,
                    })
                }
            }
            Orientation::Right => {
                if player.coords.col >= self.map_width {
                    None
                } else {
                    Some(Coord {
                        row: player.coords.row,
                        col: player.coords.col + 1,
                    })
                }
            }
            Orientation::Down => {
                if player.coords.row + 1 >= self.map_height {
                    None
                } else {
                    Some(Coord {
                        row: player.coords.row + 1,
                        col: player.coords.col,
                    })
                }
            }
            Orientation::Left => {
                if player.coords.col == 0 {
                    None
                } else {
                    Some(Coord {
                        row: player.coords.row,
                        col: player.coords.col - 1,
                    })
                }
            }
        }?;

        let element_in_front = self.what_is_at(&coords_in_front)?;

        Some((coords_in_front, element_in_front))
    }

    fn update_map(&mut self, at: &Coord, element: MapElements) {
        if at.row + 1 > self.map_height || at.col + 1 > self.map_width {
            panic!("Tried writing outside of map at {}, {}", at.col, at.row);
        } else {
            self.map[(self.map_width * at.row) + at.col] = element;
        }
    }

    fn step(&mut self) -> Option<&Coord> {
        match self.what_is_in_front(&self.player) {
            Some((coord, element)) => match element {
                MapElements::Free | MapElements::PrevouslySeen => {
                    self.player.coords = coord;
                    self.update_map(&self.player.coords.clone(), MapElements::PrevouslySeen);
                    return Some(&self.player.coords);
                }
                MapElements::Obstructed => {
                    self.player.orientation = self.player.orientation.rotate_right();
                    return Some(&self.player.coords);
                }
            },
            // We went out of bounds, the desired end state.
            None => return None,
        };
    }
}

impl Display for SituationMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.map_height {
            for col in 0..self.map_width {
                match self.what_is_at(&Coord { row, col }) {
                    Some(MapElements::Free) => write!(f, "\x1b[31;42m")?,
                    Some(MapElements::PrevouslySeen) => write!(f, "\x1b[31;106m")?,
                    Some(MapElements::Obstructed) => write!(f, "\x1b[31;40m")?,
                    None => todo!("Read outside of map"),
                }

                if self.player.coords.col == col && self.player.coords.row == row {
                    match self.player.orientation {
                        Orientation::Up => write!(f, "^")?,
                        Orientation::Right => write!(f, ">")?,
                        Orientation::Down => write!(f, "V")?,
                        Orientation::Left => write!(f, "<")?,
                    }
                } else {
                    write!(f, " ")?
                }

                write!(f, "\x1b[0m")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for SituationMap {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut player: Option<Player> = None;
        let mut map = Vec::with_capacity(value.bytes().len());
        let mut map_height = 0;
        let map_width = if let Some(first_row) = value.lines().next() {
            first_row.len()
        } else {
            return Err(String::from(
                "Failed to get map width because the board does not contain at least one line.",
            ));
        };

        for (row, line) in value.lines().enumerate() {
            if line.len() != map_width {
                return Err(format!("Line {line} is not of width {map_width}"));
            }

            map_height += 1;

            for (col, c) in line.chars().enumerate() {
                match c {
                    '.' => map.push(MapElements::Free),
                    '#' => map.push(MapElements::Obstructed),
                    '^' | '>' | 'v' | '<' => {
                        map.push(MapElements::PrevouslySeen);

                        let orientation = match c {
                            '^' => Orientation::Up,
                            '>' => Orientation::Right,
                            'v' => Orientation::Down,
                            '<' => Orientation::Left,
                            _ => panic!("This is literally impossible"),
                        };

                        match player {
                            Some(p) => {
                                return Err(format!(
                                    "Duplicate player first at {}, {} then at {row}, {col}",
                                    p.coords.row, p.coords.col
                                ))
                            }
                            None => {
                                player = Some(Player {
                                    coords: Coord { row, col },
                                    orientation,
                                })
                            }
                        }
                    }
                    _ => return Err(format!("Map contains char {c} that we cannot parse")),
                }
            }
        }

        match player {
            Some(player) => Ok(SituationMap {
                player,
                map,
                map_height,
                map_width,
            }),
            None => Err(String::from("Failed to detect player")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1_example() {
        let board = SituationMap::try_from(EXAMPLE_INPUT).unwrap();
        assert_eq!(solve_part_1(board, true), 41);
    }

    #[test]
    fn test_insert() {
        let mut board = SituationMap::try_from(
            "....
....
..^.
..#.",
        )
        .unwrap();
        assert_eq!(board.map_height, 4);
        assert_eq!(board.map_width, 4);

        let coords = Coord { col: 0, row: 0 };
        board.set_at(&coords, MapElements::Obstructed);
        println!("{board}");

        assert!(board
            .what_is_at(&coords)
            .is_some_and(|e| *e == MapElements::Obstructed));

        let coords = Coord { col: 3, row: 3 };
        board.set_at(&coords, MapElements::Obstructed);
        println!("{board}");

        assert!(board
            .what_is_at(&coords)
            .is_some_and(|e| *e == MapElements::Obstructed));

        assert!(board
            .what_is_in_front(&board.player)
            .is_some_and(|(_, e)| *e == MapElements::Free));

        board.player.orientation = Orientation::Down;

        assert!(board
            .what_is_in_front(&board.player)
            .is_some_and(|(_, e)| *e == MapElements::Obstructed));
    }

    #[test]
    fn test_part_2_example() {
        let board = SituationMap::try_from(EXAMPLE_INPUT).unwrap();

        assert_eq!(solve_part_2(board, true), 6);
    }

    #[test]
    fn test_circular_path_detection() {
        let boards = [".............
...........#.
#v..........#
.#.........#."];

        for board in boards {
            println!("{board}");
            let parsed_board = SituationMap::try_from(board).unwrap();
            println!("{parsed_board}");
            assert_eq!(solve_part_2(parsed_board, true), 1);
        }
    }

    #[test]
    fn test_detection_near_edges() {
        let boards = [
            "#<..
....
....
....",
            "#...
^...
....
....",
            "..>#
....
....
....",
            "...#
...^
....
....",
            "....
....
....
#<..",
            "....
....
v...
#...",
            "....
....
....
..>#",
            "....
....
...v
...#",
        ];
        for board in boards {
            let parsed_board = SituationMap::try_from(board).unwrap();
            match parsed_board.what_is_in_front(&parsed_board.player) {
                Some((_, element)) => assert_eq!(element, &MapElements::Obstructed),
                _ => panic!("Failed to get expected element."),
            }
        }
    }
}
