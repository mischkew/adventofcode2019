use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;

struct Object {
    name: String,
    orbits_around: Option<Rc<RefCell<Object>>>,
}

impl Object {
    pub fn empty(name: &str) -> Rc<RefCell<Object>> {
        Rc::new(RefCell::new(Object {
            name: name.to_string(),
            orbits_around: None,
        }))
    }

    pub fn has_orbit(&self) -> bool {
        self.orbits_around.is_some()
    }

    pub fn count_orbits(&self) -> u32 {
        self.track_orbits().len() as u32
    }

    pub fn track_orbits(&self) -> Vec<String> {
        let mut current = self.clone();
        let mut orbits = vec![];

        while current.has_orbit() {
            orbits.push(current.name.clone());

            if let Some(center) = current.orbits_around {
                current = center.borrow().clone();
            }
        }

        orbits
    }

    pub fn clone(&self) -> Object {
        Object {
            name: self.name.to_string(),
            orbits_around: match &self.orbits_around {
                Some(object) => Some(Rc::clone(&object)),
                None => None,
            },
        }
    }

    pub fn set_center(&mut self, other: &Rc<RefCell<Object>>) {
        self.orbits_around.replace(Rc::clone(other));
    }
}

struct Map {
    objects: HashMap<String, Rc<RefCell<Object>>>,
}

impl Map {
    pub fn new(orbits: Vec<String>) -> Map {
        let mut objects = HashMap::new();

        for orbit_str in orbits {
            let orbit: Vec<String> = orbit_str.split(")").map(|s| s.to_string()).collect();
            let (center, object) = (&orbit[0], &orbit[1]);

            objects
                .entry(center.to_string())
                .or_insert_with(|| Object::empty(&center));

            objects
                .entry(object.to_string())
                .or_insert_with(|| Object::empty(object));

            objects[object].borrow_mut().set_center(&objects[center]);
        }

        Map { objects }
    }

    pub fn from_file(filename: &str) -> io::Result<Map> {
        let mut io = File::open(filename)?;
        let mut contents = String::new();
        io.read_to_string(&mut contents)?;

        let orbits: Vec<String> = contents.lines().map(|a| a.to_string()).collect();
        Ok(Self::new(orbits))
    }

    pub fn count_orbits(&self) -> u32 {
        self.objects
            .values()
            .map(|object| object.borrow().count_orbits())
            .sum()
    }

    pub fn orbits_required(&self, from: &str, to: &str) -> u32 {
        let from_trail = self.objects[from].borrow().track_orbits();
        let to_trail = self.objects[to].borrow().track_orbits();

        // The shortest path between the objects is found when the reversed
        // paths from the COM object don't overlap anymore
        let common_path_length = from_trail
            .iter()
            .rev()
            .zip(to_trail.iter().rev())
            .take_while(|(f, t)| f == t)
            .count();

        // The total objects involved is the length of the uncommon paths without
        // the from- and to-start element plus the latest common element of the
        // common path. The total orbits required are the hops between the total
        // objects required, thus one-less hops than elements.
        (from_trail.len() + to_trail.len() - 2 * common_path_length - 2) as u32
    }
}

fn main() {
    if let Ok(map) = Map::from_file("inputs/day06.txt") {
        println!("Orbit Count: {}", map.count_orbits());
        println!("Hops required: {}", map.orbits_required("YOU", "SAN"));
    } else {
        panic!("Could not read input file.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_map_1() {
        let orbits = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];

        let map = Map::new(orbits.iter().map(|s| s.to_string()).collect());
        assert_eq!(map.count_orbits(), 42);
    }

    #[test]
    fn sample_map_unordered() {
        let orbits = vec![
            "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "COM)B", "B)C",
        ];

        let map = Map::new(orbits.iter().map(|s| s.to_string()).collect());
        assert_eq!(map.count_orbits(), 42);
    }

    #[test]
    fn rc_objects_count() {
        let COM = Object::empty("COM");
        let mut B = Object::empty("B");
        assert_eq!(COM.borrow().count_orbits(), 0);

        B.borrow_mut().orbits_around.replace(Rc::clone(&COM));
        assert_eq!(B.borrow().count_orbits(), 1);
    }

    #[test]
    fn track_orbits() {
        let orbits = vec![
            "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "COM)B", "B)C",
        ];

        let map = Map::new(orbits.iter().map(|s| s.to_string()).collect());
        assert_eq!(map.objects["C"].borrow().track_orbits(), vec!["C", "B"]);
        assert_eq!(
            map.objects["L"].borrow().track_orbits(),
            vec!["L", "K", "J", "E", "D", "C", "B"]
        );
    }

    #[test]
    fn find_orbit_hops() {
        let orbits = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];
        let map = Map::new(orbits.iter().map(|s| s.to_string()).collect());
        assert_eq!(map.orbits_required("YOU", "SAN"), 4);
    }
}
