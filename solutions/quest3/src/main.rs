use std::{collections::HashSet, fmt::Display};

fn main() {
    println!("Quest 3: The Conical Snail Clock");
    let lines = aoclib::read_lines("everybody_codes_e1_q03_p1.txt");

    let mut device = Device::default();
    for line in lines {
        let (x, y) = line.get_xy();
        device.add(x, y);
    }
    for _ in 0..100 {
        device.step();
    }
    println!("  part 1 = {}", device.value());
}

#[derive(Debug, Default)]
struct Device {
    snails: HashSet<(i64, i64)>,
    size: i64,
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut max_x = self.size - 1;
        for y in 1..self.size {
            for x in 1..max_x {
                if self.snails.contains(&(x, y)) {
                    write!(f, "@")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
            max_x -= 1;
        }
        Ok(())
    }
}

impl Device {
    fn add(&mut self, x: i64, y: i64) {
        self.snails.insert((x, y));
        self.size = self.size.max(x + y + 1);
    }

    fn value(&self) -> i64 {
        self.snails
            .iter()
            .map(|snail| snail.0 + 100 * snail.1)
            .sum()
    }

    fn step(&mut self) {
        let mut next = HashSet::new();
        for snail in self.snails.iter() {
            if snail.1 == 1 {
                next.insert((1, snail.0));
            } else {
                next.insert((snail.0 + 1, snail.1 - 1));
            }
        }

        self.snails = next;
    }
}

trait GetXY {
    fn get_xy(self) -> (i64, i64);
}

impl GetXY for &str {
    fn get_xy(self) -> (i64, i64) {
        let (x, y) = self.split_once(' ').unwrap();
        let (_, x) = x.split_once('=').unwrap();
        let (_, y) = y.split_once('=').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }
}
