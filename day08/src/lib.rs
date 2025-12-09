use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

/// finds the Euclidean distance between two coordinates (skips the square root)
///
/// # Examples
///
/// ```
/// let a = day08::Coord { x:1, y:1, z:1 };
/// let b = day08::Coord { x:2, y:2, z:2 };
/// let result = day08::sorta_euclidean_distance(a, b);
/// assert_eq!(result, 3);
/// ```
///
/// ```
/// let a = day08::Coord { x:10, y:10, z:10 };
/// let b = day08::Coord { x:10, y:10, z:10 };
/// let result = day08::sorta_euclidean_distance(a, b);
/// assert_eq!(result, 0);
/// ```
pub fn sorta_euclidean_distance(a: Coord, b: Coord) -> i64 {
    let xd = a.x - b.x;
    let xd2 = xd * xd;

    let yd = a.y - b.y;
    let yd2 = yd * yd;

    let zd = a.z - b.z;
    let zd2 = zd * zd;

    let sum = xd2 + yd2 + zd2;
    //proper Euclidean distance would have a square root now, but we only need relative distance
    sum
}

/// Connects the N closest coordinates and returns the pairings
///
/// # Examples
///
/// ```
/// let input = vec![
///     day08::Coord { x:1, y:1, z:1 },
///     day08::Coord { x:2, y:2, z:2 },
///     day08::Coord { x:5, y:5, z:5 },
///     day08::Coord { x:40, y:40, z:40 },
/// ];
/// let expected = vec![
///     vec![day08::Coord { x:2, y:2, z:2 }, day08::Coord { x:1, y:1, z:1 },],
///     vec![day08::Coord { x:5, y:5, z:5 }, day08::Coord { x:2, y:2, z:2 },]
/// ];
/// let result = day08::connect_closest_coords(input, 2);
/// assert_eq!(result, expected);
/// ```
///
pub fn connect_closest_coords(coords: Vec<Coord>, connections: u64) -> Vec<Vec<Coord>> {
    let mut pairs: Vec<Vec<Coord>> = Vec::new();
    let mut remaining = coords.clone();
    let mut distances: Vec<(i64, Vec<Coord>)> = Vec::new();

    //get the distances between all coords and store them for later sorting
    loop {
        if remaining.is_empty() {
            break;
        }
        let a = remaining.pop().unwrap(); //remove a to avoid a->b and b->a dupes
        for b in remaining.clone() {
            let distance = sorta_euclidean_distance(a, b);
            distances.push((distance, vec![a, b]));
        }
    }
    //sort all the distances
    distances.sort_by(|(a, _), (b, _)| a.cmp(b));
    distances.reverse();

    //store the shortest connections as pairs
    for _ in 0..connections {
        let (_, v) = distances.pop().unwrap();
        pairs.push(v);
    }
    pairs
}

/// Coverts a vector of pairs to a vector of connected circuits
///
/// Examples
/// ```
/// use std::collections::HashSet;
/// let input = vec![
///     vec![day08::Coord { x:2, y:2, z:2 }, day08::Coord { x:1, y:1, z:1 },],
///     vec![day08::Coord { x:5, y:5, z:5 }, day08::Coord { x:2, y:2, z:2 },]
/// ];
/// let mut set = HashSet::new();
/// set.insert(day08::Coord { x:2, y:2, z:2 });
/// set.insert(day08::Coord { x:1, y:1, z:1 });
/// set.insert(day08::Coord { x:5, y:5, z:5 });
/// let expected = vec![set];
/// let result = day08::pairs_to_circuits(input);
/// assert_eq!(result, expected);
/// ```
///
/// ```
/// use std::collections::HashSet;
/// let input = vec![
///     vec![day08::Coord { x:2, y:2, z:2 }, day08::Coord { x:1, y:1, z:1 },],
///     vec![day08::Coord { x:5, y:5, z:5 }, day08::Coord { x:9, y:9, z:9 },]
/// ];
/// let expected = vec![
///     HashSet::from([day08::Coord { x:2, y:2, z:2 }, day08::Coord { x:1, y:1, z:1 },]),
///     HashSet::from([day08::Coord { x:5, y:5, z:5 }, day08::Coord { x:9, y:9, z:9 },])
/// ];
/// let result = day08::pairs_to_circuits(input);
/// assert_eq!(result, expected);
/// ```
pub fn pairs_to_circuits(pairs: Vec<Vec<Coord>>) -> Vec<HashSet<Coord>> {
    let mut circuits: Vec<HashSet<Coord>> = Vec::new();

    for pair in pairs {
        let mut merged = false;
        for (i, c) in circuits.clone().iter().enumerate() {
            if c.contains(&pair[0]) && c.contains(&pair[1]) {
                //this circuit already has both, don't add a dupe but its merged
                merged = true;
                break;
            }
            if c.contains(&pair[0]) {
                //we already have a circuit with this coord, add its  pair to the circuit
                circuits[i].insert(pair[1]);
                merged = true;
                break;
            }
            if c.contains(&pair[1]) {
                //we already have a circuit with this coord, add its pair to the circuit
                circuits[i].insert(pair[0]);
                merged = true;
                break;
            }
        }
        if !merged {
            //no circuits contain either coord of the pair, add the pair as a new circuit
            let mut set: HashSet<Coord> = HashSet::new();
            for c in pair {
                set.insert(c);
            }
            circuits.push(set);
        }
    }

    //circuits now has everything BUT there may be overlapping circuits to merge
    'multipass: loop {
        let mut merged = false;
        let mut unmerged_circuits: Vec<HashSet<Coord>> = circuits.clone();
        'all_circuits: loop {
            if unmerged_circuits.is_empty() {
                break 'all_circuits;
            }
            let this_circuit = unmerged_circuits.pop().unwrap();
            'other_circuits: for (i, c) in circuits.clone().iter().enumerate() {
                if c == &this_circuit {
                    continue 'other_circuits;
                }
                //check each entry in the circuit for a match
                for t in this_circuit.clone() {
                    if c.contains(&t) {
                        merged = true;
                        //overlapping circuits, merge them
                        for t2 in this_circuit.clone() {
                            circuits[i].insert(t2);
                        }
                        //remove the merged circuit from the full list
                        for (i, c2) in circuits.clone().iter().enumerate() {
                            if c2 == &this_circuit {
                                circuits.remove(i);
                                break;
                            }
                        }
                        //go to the next
                        continue 'all_circuits;
                    }
                }
            }
        }
        if !merged {
            // nothing left to merge
            break 'multipass;
        }
    }

    circuits
}

pub fn get_total_big_circuits(input: File, connections: u64) -> u64 {
    let mut result: u64 = 1;
    let buf = BufReader::new(input);
    let mut data: Vec<Coord> = Vec::new();

    //gather the whole file into vector of coords
    for line in buf.lines() {
        let validated_line = line.expect("weird line");
        let split_data: Vec<&str> = validated_line.split(',').collect();
        let x = split_data[0].parse().unwrap();
        let y = split_data[1].parse().unwrap();
        let z = split_data[2].parse().unwrap();
        let c = Coord { x, y, z };
        data.push(c);
    }
    let pairs = connect_closest_coords(data, connections);
    let mut circuits = pairs_to_circuits(pairs);
    println!("circuits: {circuits:?}");
    circuits.sort_by(|a, b| a.len().cmp(&b.len()));

    for _ in 0..3 {
        let c = circuits.pop().unwrap();
        result *= c.len() as u64;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn connect_closest_coords_handles_distance_collision() {
        let input = vec![
            Coord { x: 1, y: 1, z: 1 },
            Coord { x: 2, y: 2, z: 2 },
            Coord { x: 3, y: 3, z: 3 },
            Coord { x: 9, y: 9, z: 9 },
        ];
        let expected = vec![
            vec![Coord { x: 3, y: 3, z: 3 }, Coord { x: 2, y: 2, z: 2 }],
            vec![Coord { x: 2, y: 2, z: 2 }, Coord { x: 1, y: 1, z: 1 }],
        ];
        let result = connect_closest_coords(input, 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_total_big_circuits_example_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test1.txt");
        let data = File::open(path).expect("test1.txt file missing");
        let result = get_total_big_circuits(data, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn get_total_big_circuits_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("input.txt file missing");
        let result = get_total_big_circuits(data, 1000);
        assert_eq!(result, 98696);
    }
}
