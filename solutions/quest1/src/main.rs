use aoclib::power_mod;
use std::{collections::HashMap, str::FromStr};

fn main() {
    println!("Quest 1: EniCode");
    let lines = aoclib::read_lines("input1.txt");
    let entries = lines
        .iter()
        .map(|line| line.parse::<Entry>().unwrap())
        .collect::<Vec<_>>();
    println!(
        "  part 1 = {}",
        entries.iter().map(|entry| entry.calculate()).max().unwrap()
    );

    let lines = aoclib::read_lines("input2.txt");
    let entries = lines
        .iter()
        .map(|line| line.parse::<Entry>().unwrap())
        .collect::<Vec<_>>();
    println!(
        "  part 2 = {}",
        entries
            .iter()
            .map(|entry| entry.trunc_calculate())
            .max()
            .unwrap()
    );

    let lines = aoclib::read_lines("input3.txt");
    let entries = lines
        .iter()
        .map(|line| line.parse::<Entry>().unwrap())
        .collect::<Vec<_>>();
    println!(
        "  part 3 = {}",
        entries
            .iter()
            .map(|entry| entry.loop_calculate())
            .max()
            .unwrap()
    );
}

#[derive(Debug)]
struct Entry {
    a: usize,
    b: usize,
    c: usize,
    x: usize,
    y: usize,
    z: usize,
    m: usize,
}

impl Entry {
    fn calculate(&self) -> usize {
        eni(self.a, self.x, self.m) + eni(self.b, self.y, self.m) + eni(self.c, self.z, self.m)
    }

    fn trunc_calculate(&self) -> usize {
        trunc_eni(self.a, self.x, self.m)
            + trunc_eni(self.b, self.y, self.m)
            + trunc_eni(self.c, self.z, self.m)
    }

    fn loop_calculate(&self) -> usize {
        loop_eni(self.a, self.x, self.m)
            + loop_eni(self.b, self.y, self.m)
            + loop_eni(self.c, self.z, self.m)
    }
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words = line.split(' ').collect::<Vec<_>>();
        let a = words[0].split_once('=').unwrap().1.parse().unwrap();
        let b = words[1].split_once('=').unwrap().1.parse().unwrap();
        let c = words[2].split_once('=').unwrap().1.parse().unwrap();
        let x = words[3].split_once('=').unwrap().1.parse().unwrap();
        let y = words[4].split_once('=').unwrap().1.parse().unwrap();
        let z = words[5].split_once('=').unwrap().1.parse().unwrap();
        let m = words[6].split_once('=').unwrap().1.parse().unwrap();

        Ok(Self {
            a,
            b,
            c,
            x,
            y,
            z,
            m,
        })
    }
}

fn eni(n: usize, exp: usize, mmod: usize) -> usize {
    let mut list = String::new();
    let mut score = 1;
    for _ in 0..exp {
        score = (score * n) % mmod;
        list = format!("{score}{list}");
    }
    list.parse().unwrap()
}

fn trunc_eni(n: usize, exp: usize, mmod: usize) -> usize {
    let mut score = if exp > 5 {
        power_mod(1, n, exp - 5, mmod)
    } else {
        1
    };

    let mut list = String::new();
    for _ in 0..exp.min(5) {
        score = (score * n) % mmod;
        list = format!("{score}{list}");
    }
    list.parse().unwrap()
}

fn loop_eni(n: usize, exp: usize, mmod: usize) -> usize {
    let mut score = 1;
    let mut sum = 0;
    let mut visited = HashMap::<usize, (usize, usize)>::new(); // (steps, running_total)

    let mut step = 0;
    while step < exp {
        score = (score * n) % mmod;
        sum += score;
        step += 1;
        if let Some((prev_step, prev_sum)) = visited.get(&score) {
            let delta_steps = step - prev_step;
            let delta_sum = sum - prev_sum;
            let amount = (exp - step) / delta_steps;
            sum += delta_sum * amount;
            step += delta_steps * amount;
        }
        visited.insert(score, (step, sum));
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    // A=8 B=4 C=7 X=8 Y=4 Z=6 M=12
    // A=2 B=8 C=6 X=2 Y=4 Z=5 M=13
    // A=5 B=9 C=6 X=8 Y=6 Z=8 M=14
    // A=5 B=9 C=7 X=6 Y=6 Z=8 M=15
    // A=8 B=8 C=8 X=6 Y=9 Z=6 M=16

    #[test]
    fn test_4_3_11() {
        assert_eq!(954, eni(4, 3, 11));
    }

    #[test]
    fn test_line1() {
        let entry: Entry = "A=4 B=4 C=6 X=3 Y=4 Z=5 M=11".parse().unwrap();
        assert_eq!(114644, entry.calculate());
    }
    //

    #[test]
    fn test_line3() {
        let entry: Entry = "A=5 B=9 C=6 X=8 Y=6 Z=8 M=14".parse().unwrap();
        assert_eq!(11611972920, entry.calculate());
    }

    #[test]
    fn test_trunc_eni_1() {
        assert_eq!(10510510, trunc_eni(5, 6, 15));
    }

    #[test]
    fn test_trunc_eni_2() {
        assert_eq!(0, trunc_eni(8, 6, 16));
    }

    #[test]
    fn test_part2() {
        let lines = aoclib::read_lines("test2_2.txt");
        let entries = lines
            .iter()
            .map(|line| line.parse::<Entry>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            1507702060886,
            entries
                .iter()
                .map(|entry| entry.trunc_calculate())
                .max()
                .unwrap()
        );
    }

    #[test]
    fn test_loop1() {
        assert_eq!(132000, loop_eni(4, 3000, 110));
        assert_eq!(559940, loop_eni(4, 14000, 120));
        assert_eq!(1200000, loop_eni(9, 16000, 150));
        assert_eq!(1279880, loop_eni(8, 16000, 160));
    }

    #[test]
    fn test_part3() {
        let lines = aoclib::read_lines("test3_2.txt");
        let entries = lines
            .iter()
            .map(|line| line.parse::<Entry>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            7276515438396,
            entries
                .iter()
                .map(|entry| entry.loop_calculate())
                .max()
                .unwrap()
        );
    }
}
