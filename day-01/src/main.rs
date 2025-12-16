use std::{any::type_name_of_val, fs::File, io::Read};

fn main() {
    part_1();
}

fn part_1() {
    let mut file = File::open("input.txt").unwrap();

    let mut data = vec![];

    let read = file.read_to_end(&mut data).unwrap();

    println!("Read {} bytes", read);

    let mut start_idx: usize = 0;
    let mut end_idx: usize = 0;

    let mut strings = vec![];
    for byte in &data {
        if (*byte) == b'\n' {
            strings.push(str::from_utf8(&data[start_idx..end_idx]).unwrap());
            start_idx = end_idx + 1;
            end_idx = start_idx;
            continue;
        }
        end_idx = end_idx + 1;
    }

    let mut start: i16 = 50;
    let mut zero: u16 = 0;

    // let strings = vec![
    //     "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    // ];

    for s in strings {
        match s.chars().next() {
            Some('L') => {
                let parsed: i16 = (&s[1..]).parse().expect("No invalid values");
                start = ((start - parsed) % 100 + 100) % 100;

                if start == 0 {
                    zero = zero + 1;
                }
            }
            Some('R') => {
                let parsed: i16 = (&s[1..]).parse().expect("No invalid values");
                start = start + parsed;
                start = start % 100;

                if start == 0 {
                    zero = zero + 1;
                }
            }
            Some(c) => panic!("Invalid direction {}", c),
            None => panic!("Empty string"),
        }
    }

    println!("Start at the end: {}", start);
    println!("zeros : {}", zero);
}
