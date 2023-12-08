use std::collections::HashMap;
use std::fs;

struct Day8 {
    moves: Vec<bool>,
    n: usize,
    map: HashMap<[u8; 3], ([u8; 3], [u8; 3])>
}

impl Day8 {
    fn str_to_u8_array(s: &str) -> Option<[u8; 3]> {
        let bytes = s.as_bytes();
        if bytes.len() != 3 {
            None
        } else {
            Some([bytes[0], bytes[1], bytes[2]])
        }
    }
    fn parse_input() -> Day8 {
        let data = fs::read_to_string("data/day8").unwrap_or_default();
        assert!(!data.is_empty());
        let mut parts = data.split_terminator('\n');
        let moves: Vec<bool> = parts.next().unwrap_or_else(|| panic!("Insufficient data"))
            .chars().map(|x| x == 'R').collect();
        parts.next();
        let mut map = HashMap::new();
        for line in parts {
            let mut line_parts = line.split(" = ");
            let key = Day8::str_to_u8_array(line_parts.next()
                .unwrap_or_else(|| panic!("Line part malformed")))
                .unwrap_or_else(|| panic!("Key does not contain 3 chars"));
            let val = line_parts.next()
                .unwrap_or_else(|| panic!("Line part malformed"));
            let left = Day8::str_to_u8_array(&val[1..4])
                .unwrap_or_else(|| panic!("Map val left does not contain 3 chars"));
            let right = Day8::str_to_u8_array(&val[6..9])
                .unwrap_or_else(|| panic!("Map val right does not contain 3 chars"));
            map.insert(key, (left, right));
        }

        Day8 {
            n: moves.len(),
            moves,
            map
        }
    }

    fn part_1(&self) -> i32 {
        let mut start = b"AAA";
        let mut count = 0;
        while start != b"ZZZ" {
            let (left, right) = &self.map[start];
            if self.moves[count % self.n] {
                start = right;
            } else {
                start = left;
            }

            count += 1;
        }
        count as i32
    }
}


#[cfg(test)]
mod tests {
    use crate::day8::Day8;

    #[test]
    fn test_input() {
        let day = Day8::parse_input();
        assert_eq!(20777, day.part_1());
    }
}