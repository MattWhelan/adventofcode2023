use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Item {
    RoundRock,
    FlatRock,
    Empty,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct RowGrid {
    grid: Vec<Vec<Item>>,
}

impl FromStr for RowGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<Item>> = s.lines().map(|l| {
            l.chars().map(|ch| match ch {
                '.' => Item::Empty,
                '#' => Item::FlatRock,
                'O' => Item::RoundRock,
                _ => panic!("bad input {ch}")
            }).collect()
        }).collect();

        Ok(Self { grid })
    }
}

impl Display for RowGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for item in row {
                match item {
                    Item::RoundRock => write!(f, "O"),
                    Item::FlatRock => write!(f, "#"),
                    Item::Empty => write!(f, "."),
                }?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl RowGrid {
    pub fn col(&self, col: usize) -> impl Iterator<Item=&Item> {
        self.grid.iter().map(move |row| &row[col])
    }

    pub fn col_rev(&self, col: usize) -> impl Iterator<Item=&Item> {
        self.grid.iter().rev().map(move |row| &row[col])
    }

    pub fn load(&self) -> usize {
        let row_count = self.grid.len();
        let col_count = self.grid[0].len();
        (0..col_count).map(|col| {
            self.col(col).enumerate().map(|(row, item)| {
                match item {
                    Item::RoundRock => {
                        row_count - row
                    }
                    Item::FlatRock => {
                        0
                    }
                    Item::Empty => {
                        0
                    }
                }
            }).sum::<usize>()
        }).sum()
    }

    fn without_round(&self) -> Self {
        let mut ret = self.clone();
        ret.grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|item| {
                if let Item::RoundRock = item {
                    *item = Item::Empty;
                }
            })
        });

        ret
    }

    pub fn north(&self) -> Self {
        let mut ret = self.without_round();

        let col_count = self.grid[0].len();
        (0..col_count).for_each(|col| {
            let mut offset = 0usize;
            self.col(col).enumerate().for_each(|(row, item)| {
                match item {
                    Item::RoundRock => {
                        ret.grid[offset][col] = Item::RoundRock;
                        offset += 1;
                    }
                    Item::FlatRock => {
                        offset = row+1;
                    }
                    Item::Empty => {
                    }
                }
            })
        });

        ret
    }

    pub fn south(&self) -> Self {
        let mut ret = self.without_round();

        let row_count = self.grid.len();
        let col_count = self.grid[0].len();
        (0..col_count).for_each(|col| {
            let mut last = row_count;
            self.col_rev(col).enumerate().for_each(|(neg_row, item)| {
                match item {
                    Item::RoundRock => {
                        ret.grid[last-1][col] = Item::RoundRock;
                        last -= 1;
                    }
                    Item::FlatRock => {
                        last = row_count - neg_row - 1;
                    }
                    Item::Empty => {
                    }
                }
            })
        });

        ret
    }

    pub fn east(&self) -> Self {
        let mut ret = self.without_round();

        let row_count = self.grid.len();
        let col_count = self.grid[0].len();
        (0..row_count).for_each(|row| {
            let mut last = col_count;
            self.grid[row].iter().enumerate().rev().for_each(|(col, item)| {
                match item {
                    Item::RoundRock => {
                        ret.grid[row][last-1] = Item::RoundRock;
                        last -= 1;
                    }
                    Item::FlatRock => {
                        last = col;
                    }
                    Item::Empty => {
                    }
                }
            })
        });

        ret
    }

    pub fn west(&self) -> Self {
        let mut ret = self.without_round();

        let row_count = self.grid.len();
        (0..row_count).for_each(|row| {
            let mut offset = 0usize;
            self.grid[row].iter().enumerate().for_each(|(col, item)| {
                match item {
                    Item::RoundRock => {
                        ret.grid[row][offset] = Item::RoundRock;
                        offset += 1;
                    }
                    Item::FlatRock => {
                        offset = col + 1;
                    }
                    Item::Empty => {
                    }
                }
            })
        });

        ret
    }

    pub fn cycle(&self) -> Self {
        self.north().west().south().east()
    }
}

fn main() {
    let world: RowGrid = INPUT.parse().unwrap();

    let north = world.north();
    println!("Part 1: \n{}\n {}", &north, north.load());

    let test: RowGrid = _TEST.parse().unwrap();

    let (cycle_grid, start, period) = find_cycle(&world);

    println!("Cycle of period {period} at {start}:\n{cycle_grid}");

    const BILLION: usize = 1_000_000_000;
    let offset = (BILLION - start) % period;
    {
        let mut w = cycle_grid;
        for _ in 0..offset {
            w = w.cycle()
        }

        println!("Part 2: \n{}\n {}", &w, w.load())
    }
}

fn find_cycle(world: &RowGrid) -> (RowGrid, usize, usize) {
    let mut history: HashMap<RowGrid, usize> = HashMap::new();
    let mut world = world.clone();

    for i in 0.. {
        if history.contains_key(&world) {
            let cycle_start = history[&world];
            return (world, cycle_start, i-cycle_start);
        }
        let next = world.cycle();
        history.insert(world, i);
        world = next;
    }
    panic!("no cycle found");
}

const INPUT: &str = "O#....#.........#.##...O...#.O#..#...O..O#.##.OOO.O...#..#...#OO...OOO#O##..O.O.O..#....O..#...#....
...O...O...#OO......#.........O.##....O.#O#..OO.#.O##.O.#.#.O...#O.OOOO.O#..O.#OO....O.....O.O...O..
.....#..O....O....O....O..#OO.O..#O.......O......O.O.....OO....OO......##.O.#.#....#..#.#..OOO.#.O#.
.....#..##OO..O.....#..#...O..#..O..#O..#OOO..#......O...##......OO#....#......#O.....O#.#O#O...#...
#..O..OO##O...O#O#.OOO...#.....#.#O.O#....#..........O...O#O..........#........O.O.O#O...#..OO#.....
#O.#.....##.#.OO.#..#.OO#..#.##...#..O.O..........#O.#..OO.#..O.......O.#O..O..O.O..O.....O..OOO...#
#......O.##.##...OO..O.##.##.O.#..........#..O.###O...O...##.#....###OO...#...#............O.O..#.#O
....O...O.###.#.#...O..O..O.OO...O...#........#O.....O...O#.O.O.....O#O........##.O..#OO.OO#.O##....
.....O..O#..OO.....#.O..O.#.#............#.###...#.O.###......#O#.OO.#O..OO...O.....O#O......O......
.....O.O...O#.......O.......#.OO.O##..#...O.#....#..O.....O...O.O..O.O.#.O#O...#.#..O...#O..O..OO...
O..#..O......O.....O....##OO.O..O.....#.O.#..#.#O.#.#O..............##..#..O......O...O.##..O....#..
.#...#......#..O...###....#.O#...#..O..#O......##O.OO.O.......O#OO...O#.O.#....O.O#..#.#OO.O#O.OO...
...#...O#..#..##...OO####O#.#.##....###OO#.O..........O....#O...#.....OO#.#....O....O#.##...O.#.#...
...O.OO.OO#O#.##O..O.....O...#....#O.....#.#...#O#..#..OO.O.#.#O.#.O.#..OO....O..#..#.O.#O......OO..
......#...O#...O#...O...O#..........O.....#OOO#.......#.O.O...O...#.#...#O##..O##.OO..O.......O#O...
......#...O.O...O.O.....#....#.#.....#.#...#.O#....#OO.#...OO.....#.......O.#.....O......O..##......
#O.O.O.O.OO...O###.........O.#.....O#.......O#...#O......O..#.O...O.....O...#.##O.#..##O..O#...O.#.O
O.O...#...OO#.#....OO.O...###..OO#O...O...#...##..O#.#.##..OO..O#..O...O.#...##.......#...OO.#OO.O..
..##OO#.............#...O..OO.#...##....O.O.....O.#O...O.O...O#.###....#....#....#.O..O...#....O#O#.
...O..#.......O.....O......O..O.#O#O.#......OOO..#...#..O....#..O#.#.O..O..#......OO..O..#.OO...O...
O#..........#OOOO.##..O.O#O.#.#...OO......##..#OO#.O.O#OOO.OO#.......#O.#...O#O.O..OO.....O...#O....
..#O#..O....#........#OO###...#.##.##.O....#..#..O....#O..#......O....#..O..#.....O.#......O##..#.#.
##.#..##.O.O.#.O.....O....O#.#...O..#..#....##O.#....O..OO.O.....#O.#O....O.......#.OO..O.OOO#OO#...
#.........#.#.#..OO.O.....O...#......O#O.O....#..O.#OO....O..O...#O...####O...O.....#..#....O.#.##..
#........O.O..OO.....O.....#O.O.....O....#....#...O.#.#.#.O...O#..#....#OO##.#..##O.OO...#O#.OO.#.O.
O........O.....O...O..#..#.O..............#..##.#.....O#O#.#...O#.O...O..O...#.#..O..O.OO.O....#....
..O.O.........##.#O..O...O.O....O#.#..O..##.........#.#O.O.....#.#......OO#..OO.#.O.OO...O.OO..#.O.O
#OO...O##..O#..OO.O#O.#.#.O..#..O....O.O#.#O.#........OO.O..........#....O.O....#.OO.#O..........#O.
.O..O##.#.O.##.O..O......#...###...#..#OOO.#...#O.......O.....O.#.....O.OO#...O#O.O...#OO.....O.#.#.
....#.#.O...O#OO..#..OOOO...O.............O..O..O#..O.#.O.....#.O......#OO.O#OOO........#..#..OO...#
.O...#....O..O##.OO.###.O..OO.O....#.O..##.OO#O.....##..........###.O...O#..OO..O.O#O.....O.#.......
#.O......OOO......O.#..O..##...#O.O#.O#..OO..#..#O.OOO#..#.O.O...O..O....#O...O...#..#.##.O...#.O#.#
...O...O....O.#.#OOO.....O#O......O#..OO........O.....O..O.......##O..#...OOOOO#....O..#.O......#..O
#.....#....O.....O..O.O.#O#O.O.#.....#..#O.....#.#.#...OO.OOO#O##.##O.O.###.O..O...O#.#.O......##..O
.O.#..#.O##..#..OO#.#.O#.#.O..O.....#....O##...O.....##O..##...O...O...O.#.#...#O..OO...O....#....#.
O#..#.###....O.#..#.O.O...O.O....#.......#.O.#O.#...#.O.O#..#.#..O..O.O#...O....O.O.O##...#.....OOO.
...#.#O.#......OO#.##..O..#.#.O.###O...O.#O.O..O..#.....O..#O.O.O....OO##O..OO...O##.#....O.O...O...
.OO.O#..#.OO.O.O..O#OOO.OO.##.O........O..#.O..#..O.............O.O..#.OOO.#................##O.....
....##O...O..O..O.........#..O..O...O...O..O.#.....O......O#.O#.#.......OO...O.#.O#.....#.#........#
...#.O...OO.OOO.#O............O..O#..#...#..OO.....O........O..#.O..O...O#O....##O.O...O.O..#.O....#
...#.#O..O#.#..O....O..OO....O..O#O..........OO.O..O.......#.#..#....O..##.O.#....O#O.....#OO.....O.
..#.O...#.O..O#.....O...OO##.#.OO.O.O.O.O.#.O#...#O...O..###.....O#..#...........O.##O......#..O.#..
O.....O......O.O....#.O.O..#.#...O...#.O.O#OO..#...O#......O.OO.O.O..#...#.#.....O#....OOO..OOO.O..#
O....#O#....#..O#....##........O##O......O##.#O.....O.O.....#.##..#.OO..##....#.O#...OO........#..O.
.#....O...O.#......#O##.#......#..#.#...............OO.OOOOO.O.OO..OO.O.#.O.......#O....O#.#O..OO.O#
#O.OOO#OO....##.O.#.O#.O...#....#.....#....OO#.O.#OOOO....#..#.#.##.O...#...OOO...#......O......#.#.
O.#.#O.#.#.#.O.#.O........#.O.#.OO#O.O##.#..#...O.O.....O.#...O#.....#.OO..O#.OO.#...#.O....#.......
..O..O.O.O..###....#O...#O#.##O......#O....OO.O..O.#...O#.O....#..#.#.O.........#OO..........O#..O#.
#.#...........#.......#..OO..#.....##...#........#O.#O..#O....#OO.O.O....#.O..O#.....##.O###O##....O
..OO..OOOO.....O...#.O.#....O.....O.##O#...#..O...#...O.OOO.O..#.#...#....O#.#...#.O.O.O.....##..O.#
..OOO..#.#O..OO.O......O...#.#..O.....O.OO.O.O.OO...O..#.O.O.....#..O..#.##.O....#.#O.OOOOO#..O#.O..
.....#.....O......#..#.....O..........O####..O..O.#O#.....#O...#..O.#...#....#.OOOO.OO.O.O.O#...O..#
O.#.#O.#.O...O.OO..O......#...#.O#......O...##O#..O.OO.#....O#...OO...#...................O..#O...O.
.#.OO...OO....O#......O.O#O#O.O.O....O...O.OO#.#O.O.O.#O.O...#.....O..##.#..##..O#...O#O#...O...#...
.#.....#..##O.OOO#.O#.##.O..#..O#O..#..#...O#O.O##....O...O.O..O#..#.O.O.O.......#O..O..#O.O...O.#.O
......O..O.#..#.##.O.O....OO..O.....#.O..#O...#O.O..#..#..O...O..O..#.#O......O..O......O.....O.##..
..O....#O..O.#O...#O...#.O.#...O...#.#.#...O..O.#.......#...O...OO.#O..#OO..OO.O#OOO###.......#.O..O
..O..O...#..#....#....O..O#.#.O.........O.#.##.#O..#.OO.#O.O.O#.....#O..O....O#.........#.#.#.....O#
......O.#O#..#OO.#...O..#....#O.....#.......##.#.............O.#OO..#.O...#..O#.####.#OO..O.O...#.#.
#......O....O..O....O.....#.O......#.........#.O.O..#...#.O..O#...#.O#.#O.#..#.O....O.#...#O..O...#.
.O...#...OO.#O.....#...O#...O.O#.#.O.#O...OO..#.O##O#.#OO#O...O#......##.O.##O.#......O.........OO..
..O#.O..#OO..O........O.OO.O..O...O.#....#..#.#O#..O...#.....#.#OO.O.......#..O...OO..O.OO.#.#O.OO..
OO.O........O.#..O...O.O...OO#.O#O.O.O...OOO.O.#...O.......OO.#.#..O#.#..#..O...OO.#..O#O.#..#....O.
O..#..O.O#.OO..#...O#........#...#.O...O...#...#.#.O...#...#O.OO.#......O...O.O........#.##O#.O..#O.
OO.OO.#..#..#....O..O.O..O..........OO#.....##OO.O..##.......#.....O.......#...O....O..#........#O.O
O#.....O.O...#..#..O#..#.O.O##.O.#O...#O..O##O....O.O##.#...###...........O#.O...........O..O.O.....
....O......##....O.........#O...O#.#.#....O.#..........#.#.O..O..#..O...O....OOO......OO.O..O#..OO.O
.O.#..O........O.O.......O..##.#..OOO..#.#.....##.O.O.#.#...#.O.#.###.O.....O....O.....O..O.O.O#...#
...O..O.O#.....O.O.......O.O.#..#O#.O#..OOOO..#......O.O.#....O...#.#..#O....#..OOO.O.OO.#O.O....O#.
...O.....#....#OOO#.O#...#.O......#.#O.O...OO.O#.OO.OO..O...#........O#.#.O#.....O..O#.OOO........#.
O..O..O...O.............O..O.O.OO...O..O.....O...O#..##OO#O..#.##...#O..#O#......O##.O........#..#..
.OO.#.#O...O...O.#.O.O.O.........O..O..O..OOO....#..#.O#..O......#..#O..#....#..#.......OO.......O#.
#.....#.O#.....#..O..O.O..O...O.##O.O..#..O.......O.O..O........#....O.#...#OO.O..O...O..O.#..O..O.O
#..##.#O##.##..#.......##.O..OO.#.#O#O#...#.#.O....OOO.##OOO.OO.O#.O#........#...#...#.##.OOO..#..#.
....O.##O#O..#O..O.O#....OO....O...#.O..O.O..#O..O..#..OOO.....##OO.....#.O.#OO...#.O.OOO.O....#....
.O........#O...O.O.O.O#..O#.O.....#OO.#.O..#.#.O.....O....O......O....O..O.#..##.....O.#.....O..O...
O...O......#..O#.......#O.....O#.#.....#.O.OO.O.#O..O##.....OO#..O.O.O........#O..O.....#..........O
..#O.O.#.O.#..#......O#..O#..O..#...O..#.O.O#.##..#.#O..O..#.#...##.#...#..O#......#...O#O.O.#..#...
.OO...O..O...O.O..O..O#.....O.#O#.O#O.#.....O.#...#..#...O..OO.....O..O.O....#...#O#.#.OO#....O....O
.#...O.........#.O.O#.OO.#O..O....O.....O....##.....O..#OOO.....#...#.#OO..#O.O.O...#O.OO.O.#.......
..OO.#O......O.O..##.#........OO..O#..O...O..#..#...#O..##O...O#...##.#..O..#......#..O.O#.O.#...O.O
..##.O..###..OO.O#O..#.......O#...#.O#..OO.##O.O.O........#O#.#.OO....#.##.O.OO#.....O.O##........#.
..O.OO.O##.O...##O...#O..#O.#.O...#O.##..#.O.#..##....##.O....O....O.....###.....#O..#..O..#O...O...
.##.....#.....O#.O...#.....#.##.OO..OOO...OO#..O......#........#O.O..O.O...#...OO....O.#...O.O...O..
...O.....O.#....O...#..O.O......#..#OO.#.O.O#.#O....#.##...O...##O.O.O#.........O.#O......#O.......O
..###O..#O.#.O.....O.#........O.#........#O##.#..O#.##.O.O........O###.#.O....O.....###....O##O.....
#O#.....#.....O..............O...#..O..#O.O...........###.O.#.....##O.....O#O..#.....O.OO....O#O....
.#...O...O..#.#...............O.O#....O..#.#OO.O..O........O....O..O......##..#.#OO#...O#..O##......
.OO.O...O##O.#O#..O.......O..##.#O#.....#..O..O#.#O#.#...#...#..O..##..#....O.....#.#..O....O.##.O#.
#...O#.#O.O...#.O....O#.O#..O..O......OO#.O....#.....OO#O....#O.OOO...#OO..#..#O...#.#O..OO.#...O...
.O...O..O.O....#....#O...#.....#...O...#..OOO.....OO#OOO.O..#O.#....#..O...#...O..O.O##...#O.O.OO..#
O#O..O#......#.#O..#O....OO#O#...#...O....##...#.O...O.#O...OO....O.#.O.O...O#........#O.O..O#..#..O
OOO#....O...##.O.........OO..O.OOOOO.O..O..#..###.O.#....##.....O.O.O#.#..OOO....OO.....OO....O...#.
#O###O...O...#..OO...#O.....#.....O.O...O.#..OO#..##....O..##..O##.......O..O.O#OOO...O..#.....##...
..#O.....#.O#O#.O.#.#....#O..#O.O.O..#..O#..O.O.O...OO..#.#..##..O.O...OO..O.##.O.OO.#.#..##.......#
O...OO.O......#..#...O.#.O.#O...#.O.OO#...O....#OOO..#..O..O.....O.....#.O.#O#...OO.O#.OO..O.##...#.
.....#OO....O.O#O......O.O.O.#.O..O#O#...#O...#.#O...#.O..#.OO.O....OO...O#........#O...#.#........O
.O....#..OOO........#.O.#O.#.........#.#....O#OO#.......O..O#O.###.O....O.O.O......O.#.O..O..#...O.O
#.O.#O..##.OO...O##O......O#.OO........OO...O#....#....#..O#........O#..O.O...O.#O...#...#.#.....O##
.O.O.O.OO..OO.#..#....O....O.O#...O...OOO..#.OO#.....##........O...O#O.#..OO##.O.#..#..#O#.O..OO##..
";
const _TEST: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";