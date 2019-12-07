use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn go(&self, x: i32, y: i32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn length(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    /// Two lines cross when they are orthogonal to each other and if the
    /// x-value of the vertical line is between the start and end x-values of
    /// the horizontal line. Two lines don't cross otherwise, i.e. also when
    /// both are horizontal or both are vertical then the lines do not cross.
    fn crosses(&self, other: &Line) -> bool {
        let (h_line, v_line) = if self.is_horizontal() {
            (self, other)
        } else {
            (other, self)
        };

        // if the vertical line is horizontal or the horizontal line is vertical
        // then both lines overlap and don't cross by definition
        if v_line.is_horizontal() || !h_line.is_horizontal() {
            return false;
        }

        let (left, right) = if h_line.start.x < h_line.end.x {
            (&h_line.start, &h_line.end)
        } else {
            (&h_line.end, &h_line.start)
        };

        let (bottom, top) = if v_line.start.y < v_line.end.y {
            (&v_line.start, &v_line.end)
        } else {
            (&v_line.end, &v_line.start)
        };

        left.x <= top.x && top.x <= right.x && bottom.y <= left.y && left.y <= top.y
    }

    fn intersect(&self, other: &Line) -> Option<Point> {
        if !self.crosses(other) {
            return None;
        }

        let (h_line, v_line) = if self.is_horizontal() {
            (self, other)
        } else {
            (other, self)
        };

        Some(Point {
            x: v_line.start.x,
            y: h_line.start.y,
        })
    }
}

struct Wire {
    lines: Vec<Line>,
}

impl Wire {
    fn new(path: &str) -> Wire {
        let mut current = Point { x: 0, y: 0 };
        let mut lines = Vec::new();

        for segment in path.split(',') {
            let direction = segment
                .chars()
                .nth(0)
                .expect("Direction indicator not found");
            let distance: i32 = segment[1..].parse().expect("Unable to parse distance.");

            let next = match direction {
                'U' => Some(current.go(0, distance)),
                'D' => Some(current.go(0, -distance)),
                'R' => Some(current.go(distance, 0)),
                'L' => Some(current.go(-distance, 0)),
                _ => None,
            };

            if let Some(next) = next {
                lines.push(Line {
                    start: current,
                    end: next.clone(),
                });
                current = next;
            } else {
                panic!("Unknown direction indicator!")
            }
        }

        Wire { lines }
    }

    fn intersections(&self, other: &Wire) -> Vec<Point> {
        let mut intersections: Vec<Point> = Vec::new();
        for line in self.lines.iter() {
            for other_line in other.lines.iter() {
                if let Some(point) = line.intersect(&other_line) {
                    intersections.push(point);
                }
            }
        }
        intersections
    }

    fn closest_intersection(&self, other: &Wire) -> Option<Point> {
        let mut intersections = self.intersections(other);
        intersections.sort_by(|a, b| a.length().cmp(&b.length()));

        match intersections.iter().filter(|a| a.x > 0 || a.y > 0).next() {
            Some(point) => Some(point.clone()),
            None => None,
        }
    }

    fn closest_distance(&self, other: &Wire) -> Option<i32> {
        match self.closest_intersection(other) {
            Some(point) => Some(point.length()),
            None => None,
        }
    }
}

fn main() {
    let io = File::open("inputs/day03.txt").expect("Failed to read file!");
    let mut wire_strs = BufReader::new(io)
        .lines()
        .map(|line| line.expect("Failed to read line!"));

    let wire1 = Wire::new(&wire_strs.next().expect("No wire found."));
    let wire2 = Wire::new(&wire_strs.next().expect("No wire found."));

    println!(
        "Closest lines intersect at distance of {:?}.",
        wire1
            .closest_distance(&wire2)
            .expect("Could not find closest distance!")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_wire() {
        let wire_str = "R8,U5,L5,D3";
        let wire = Wire::new(wire_str);

        assert_eq!(wire.lines.len(), 4);
        assert_eq!(wire.lines[0].end, Point { x: 8, y: 0 });
        assert_eq!(wire.lines[1].start, wire.lines[0].end);
    }

    #[test]
    fn point_distance() {
        let point1 = Point { x: 0, y: 0 };
        let point2 = Point { x: 8, y: 8 };

        assert_eq!(point1.distance(&point2), 16);
        assert_eq!(point2.distance(&point1), 16);
    }

    #[test]
    fn two_h_lines_dont_cross() {
        let line1 = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 8, y: 0 },
        };
        let line2 = Line {
            start: Point { x: 0, y: 4 },
            end: Point { x: 8, y: 4 },
        };

        assert!(line1.is_horizontal());
        assert!(line2.is_horizontal());
        assert!(!line1.crosses(&line2));
        assert!(!line2.crosses(&line1));
    }

    #[test]
    fn two_v_lines_dont_cross() {
        let line1 = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 8 },
        };
        let line2 = Line {
            start: Point { x: 4, y: 4 },
            end: Point { x: 4, y: 8 },
        };

        assert!(!line1.is_horizontal());
        assert!(!line2.is_horizontal());
        assert!(!line1.crosses(&line2));
        assert!(!line2.crosses(&line1));
    }

    #[test]
    fn two_lines_dont_cross() {
        let line1 = Line {
            start: Point { x: 0, y: 1 },
            end: Point { x: 0, y: 8 },
        };
        let line2 = Line {
            start: Point { x: 4, y: 4 },
            end: Point { x: 2, y: 4 },
        };

        assert!(!line1.is_horizontal());
        assert!(line2.is_horizontal());
        assert!(!line1.crosses(&line2));
        assert!(!line2.crosses(&line1));
    }

    #[test]
    fn intersect_two_lines() {
        let line1 = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 8, y: 0 },
        };
        let line2 = Line {
            start: Point { x: 2, y: 4 },
            end: Point { x: 2, y: 0 },
        };

        assert_eq!(line1.intersect(&line2), Some(Point { x: 2, y: 0 }))
    }

    #[test]
    fn intersect_lines_when_start_is_right_of_end() {
        let line1 = Line {
            start: Point { x: 100, y: 50 },
            end: Point { x: 57, y: 50 },
        };
        let line2 = Line {
            start: Point { x: 75, y: 30 },
            end: Point { x: 75, y: 51 },
        };

        assert_eq!(line1.intersect(&line2), Some(Point { x: 75, y: 50 }));
    }

    #[test]
    fn lines_should_not_intersect() {
        let line1 = Line {
            start: Point { x: 100, y: 50 },
            end: Point { x: 57, y: 50 },
        };
        let line2 = Line {
            start: Point { x: 75, y: 30 },
            end: Point { x: 75, y: 49 },
        };

        assert!(!line1.crosses(&line2));
        assert_eq!(line1.intersect(&line2), None);
    }

    #[test]
    fn compute_closest_distance() {
        let wire1 = Wire::new("R8,U5,L5,D3");
        let wire2 = Wire::new("U7,R6,D4,L4");

        println!("{:?}", wire1.lines);
        println!("{:?}", wire2.lines);
        println!("{:?}", wire1.intersections(&wire2));
        println!("{:?}", wire1.closest_intersection(&wire2));
        assert_eq!(wire1.closest_distance(&wire2).unwrap(), 6);
    }

    #[test]
    fn compute_closest_distance_2() {
        let wire1 = Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = Wire::new("U62,R66,U55,R34,D71,R55,D58,R83");

        println!("{:?}", wire1.lines);
        println!("{:?}", wire2.lines);
        println!("{:?}", wire1.intersections(&wire2));
        println!("{:?}", wire1.closest_intersection(&wire2));
        assert_eq!(wire1.closest_distance(&wire2).unwrap(), 159)
    }

    #[test]
    fn compute_closest_distance_3() {
        let wire1 = Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(wire1.closest_distance(&wire2).unwrap(), 135);
    }
}
