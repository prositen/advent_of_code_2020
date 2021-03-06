use regex::Regex;

pub fn first(input: String) -> String {
    let mut ship = Ship::new();

    input
        .lines()
        .filter_map(|line| Direction::from(&line.trim()))
        .for_each(|d| ship.mov(d));

    (ship.lat.abs() + ship.long.abs()).to_string()
}

pub fn second(input: String) -> String {
    let mut wp = Waypoint { lat: 1, long: 10 };
    let mut ship = Ship::new();

    input
        .lines()
        .filter_map(|line| Direction::from(&line.trim()))
        .for_each(|d| match d {
            Direction::Foward(n) => ship.mov_wp(&wp, n),
            _ => wp.mov(&d),
        });

    (ship.lat.abs() + ship.long.abs()).to_string()
}

struct Ship {
    pub lat: isize,
    pub long: isize,
    heading: isize,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            long: 0,
            lat: 0,
            heading: 90,
        }
    }

    pub fn mov(&mut self, dir: Direction) {
        match dir {
            Direction::North(n) => self.lat += n,
            Direction::South(n) => self.lat -= n,
            Direction::East(n) => self.long += n,
            Direction::West(n) => self.long -= n,
            Direction::Port(n) => self.turn(-n),
            Direction::Starboard(n) => self.turn(n),
            Direction::Foward(n) => match self.heading {
                0 => self.mov(Direction::North(n)),
                90 => self.mov(Direction::East(n)),
                180 => self.mov(Direction::South(n)),
                270 => self.mov(Direction::West(n)),
                _ => panic!("Invalid heading {}", self.heading),
            },
        }
    }

    fn turn(&mut self, degrees: isize) {
        let new_heading = self.heading + degrees;
        self.heading = if new_heading < 0 {
            new_heading + 360
        } else {
            new_heading % 360
        };
    }

    pub fn mov_wp(&mut self, wp: &Waypoint, n: isize) {
        self.lat += wp.lat * n;
        self.long += wp.long * n;
    }
}
#[derive(Debug, Copy, Clone)]
struct Waypoint {
    pub lat: isize,
    pub long: isize,
}

impl Waypoint {
    pub fn mov(&mut self, dir: &Direction) {
        match dir {
            Direction::North(n) => self.lat += n,
            Direction::South(n) => self.lat -= n,
            Direction::East(n) => self.long += n,
            Direction::West(n) => self.long -= n,
            Direction::Port(n) => self.rotate(-n),
            Direction::Starboard(n) => self.rotate(*n),
            Direction::Foward(_) => {}
        }
    }

    fn rotate(&mut self, degrees: isize) {
        // N4, E10 => R90 => S10, E4
        let prior = self.clone();
        match degrees {
            90 | -270 => {
                self.lat = -prior.long;
                self.long = prior.lat;
            }
            -90 | 270 => {
                self.lat = prior.long;
                self.long = -prior.lat;
            }
            180 | -180 => {
                self.lat = -prior.lat;
                self.long = -prior.long
            }
            0 | 360 | -360 => {}
            _ => panic!("Unexpected degree {}", degrees),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North(isize),
    East(isize),
    South(isize),
    West(isize),
    Port(isize),
    Foward(isize),
    Starboard(isize),
}

lazy_static::lazy_static! {
    static ref REG: Regex = Regex::new(r"(^\D|\d+$)").unwrap();
}

impl Direction {
    pub fn from(input: &str) -> Option<Direction> {
        let mut iter = REG.find_iter(input);
        let dir: String = iter.next()?.as_str().to_string();
        let n: isize = iter.next()?.as_str().parse::<isize>().ok()?;
        match dir.as_str() {
            "N" => Some(Direction::North(n)),
            "E" => Some(Direction::East(n)),
            "S" => Some(Direction::South(n)),
            "W" => Some(Direction::West(n)),
            "L" => Some(Direction::Port(n)),
            "F" => Some(Direction::Foward(n)),
            "R" => Some(Direction::Starboard(n)),
            _ => panic!("Invalid direction {}", dir),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{first, second, Direction};

    #[test]
    fn test_part1() {
        let input = r"
        F10
        N3
        F7
        R90
        F11
        ";

        assert_eq!("25", first(input.to_string()));
    }

    #[test]
    fn test_create_direction() {
        let dir = Direction::from("N30");
        assert_eq!(Some(Direction::North(30)), dir);
    }

    #[test]
    fn test_part2() {
        let input = r"
        F10
        N3
        F7
        R90
        F11
        ";

        assert_eq!("286", second(input.to_string()));
    }
}
