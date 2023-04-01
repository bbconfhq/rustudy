use std::io::{self, BufRead};

struct Hyeongsik {
    weight: i32,
    height: i32,
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut peoples: Vec<Hyeongsik> = Vec::new();

    lines.next();
    while let Some(Ok(line)) = lines.next() {
        let mut iter = line.split_whitespace();
        let weight = iter.next().unwrap().parse::<i32>().unwrap();
        let height = iter.next().unwrap().parse::<i32>().unwrap();
        peoples.push(Hyeongsik { weight, height });
    }

    let mut ranks: Vec<i32> = Vec::new();
    for i in 0..peoples.len() {
        let mut rank = 1;
        for j in 0..peoples.len() {
            if i == j {
                continue;
            }
            if peoples[i].weight < peoples[j].weight && peoples[i].height < peoples[j].height {
                rank += 1;
            }
        }
        ranks.push(rank);
    }

    for rank in ranks {
        print!("{} ", rank);
    }
}