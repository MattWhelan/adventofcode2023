
struct Race {
    time: u64,
    best: u64,
}

impl Race {
    fn winning_approaches(&self) -> impl Iterator<Item=u64> + '_ {
        (1..self.time).filter(|t| t * (self.time-t) > self.best)
    }

    fn win_count(&self) -> u64 {
        self.winning_approaches().count() as u64
    }
}

fn main() {
    part1();

    let data_matrix: Vec<u64> = INPUT.lines()
        .map(|l| l.split_whitespace().skip(1).collect::<String>().parse().unwrap())
        .collect();

    let race = Race { time: data_matrix[0], best: data_matrix[1] };

    println!("Part 2 {}", race.win_count());
}

fn part1() {
    let data_matrix: Vec<Vec<u64>> = INPUT.lines()
        .map(|l| l.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect())
        .collect();

    let races: Vec<Race> = data_matrix[0].iter()
        .zip(data_matrix[1].iter())
        .map(|(t, d)| Race { time: *t, best: *d })
        .collect();

    println!("Part 1 {}", races.iter().map(|r| r.win_count()).product::<u64>());
}

const INPUT: &str = r"Time:        56     97     77     93
Distance:   499   2210   1097   1440
";