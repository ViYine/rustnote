use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    // x is horizontal, y is vertical
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Target(i32);

#[derive(Debug, Clone)]
struct CoordinateError;

// impl FromStr for Coordinate
impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_s = s.splitn(2, ' ').collect::<Vec<&str>>();
        match split_s[0] {
            "forward" => {
                let x = split_s[1].parse::<i32>().unwrap();
                Ok(Coordinate { x, y: 0 })
            }
            "up" => {
                let y = split_s[1].parse::<i32>().unwrap();
                Ok(Coordinate { x: 0, y })
            }
            "down" => {
                let y = split_s[1].parse::<i32>().unwrap();
                Ok(Coordinate { x: 0, y: -y })
            }
            _ => Err("Invalid coordinate".to_string()),
        }
    }
}

// impl Add for Coordinate
impl std::ops::Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let input = include_str!("fixtures/aoc2021_02.txt");
    let coordinates = input
        .lines()
        .map(|line| line.parse::<Coordinate>().unwrap());
    let final_coordinate = coordinates.fold(Coordinate { x: 0, y: 0 }, |acc, c| acc + c);
    println!("Final coordinate: {:?}", final_coordinate);
    println!("Part1: {}", final_coordinate.x * final_coordinate.y);

    // Part 2
    let (final_target, final_coor) =
        input
            .lines()
            .fold((Target(0), Coordinate::new(0, 0)), |acc, s| {
                let (mut target, mut coordinate) = acc;
                let split_s = s.splitn(2, ' ').collect::<Vec<&str>>();
                match split_s[0] {
                    "forward" => {
                        let x = split_s[1].parse::<i32>().unwrap();
                        coordinate.x += x;
                        coordinate.y += target.0 * x;
                    }
                    "up" => {
                        let y = split_s[1].parse::<i32>().unwrap();
                        target.0 += y;
                    }
                    "down" => {
                        let y = split_s[1].parse::<i32>().unwrap();
                        target.0 -= y;
                    }
                    _ => {}
                };
                (target, coordinate)
            });

    println!("Final target: {:?}", final_target);
    println!("Final coordinate: {:?}", final_coor);
    println!("Part2: {}", final_coor.x * final_coor.y);
}
