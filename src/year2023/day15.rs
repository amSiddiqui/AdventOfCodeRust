use std::fs;
use crate::traits::Day;

#[derive(Clone)]
struct Lens {
    label: String,
    power: i32,
}

impl PartialEq<Self> for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for Lens {}


pub struct Day15 {
    lens_str: Vec<String>
}

impl Day15 {
    pub fn new() -> Self {
        let lens_str = fs::read_to_string("data/day15")
            .expect("Cannot read input file")
            .split(',')
            .filter_map(|x| if x.is_empty() { None } else { Some(String::from(x)) })
            .collect::<Vec<_>>();
        Day15 {
            lens_str
        }
    }

    fn hash_fn(input: &str) -> u8 {
        let mut hsh = 0_u32;
        for c in input.chars() {
            let asc = c as u8;
            hsh += asc as u32;
            hsh *= 17_u32;
            hsh %= 256_u32;
        }
        hsh as u8
    }

}

impl Day for Day15 {
    fn part_1(&self) -> u64 {
        self.lens_str.iter()
            .map(|x| Day15::hash_fn(x.as_str()) as u64)
            .sum::<u64>()
    }

    fn part_2(&self) -> u64 {
        let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
        self.lens_str
            .iter()
            .for_each(|x| {
                if x.contains('-') {
                    let label = x.split_once('-').unwrap().0;
                    let h = Day15::hash_fn(label);
                    let pair = Lens { label: String::from(label), power: 0 };
                    if let Some(index) = boxes[h as usize].iter().position(|x| x == &pair) {
                        boxes[h as usize].remove(index);
                    }
                } else if x.contains('=') {
                    let (label, pwr) = x.split_once('=').unwrap();
                    let h = Day15::hash_fn(label);
                    let power = pwr.parse::<i32>()
                        .unwrap_or_else(|err| panic!("Cannot parse power {pwr};;Err {err}"));
                    if let Some(lens) = boxes[h as usize]
                        .iter_mut()
                        .find(|l| &(*l.label) == label) {
                         lens.power = power;
                    } else {
                        boxes[h as usize].push(Lens { label: String::from(label), power });
                    }
                }
            });

        boxes.iter()
            .enumerate()
            .map(|(i, row)|
                row.iter().enumerate()
                    .map(|(y, lens)| {
                        (i as i32 + 1) * (y as i32 + 1) * lens.power
                    }).sum::<i32>()
            ).sum::<i32>() as u64
    }
}

