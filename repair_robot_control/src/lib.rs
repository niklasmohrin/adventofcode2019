mod types;

use crate::types::{Axis, BlockType, Coordinate, Direction};
use intcode_channel_io::IntcodeThread;
use intcode_computer::ProgramMemory;
use std::collections::{HashMap, VecDeque};

pub struct RepairRobotControl {
    map: HashMap<Coordinate, BlockType>,
    thread: IntcodeThread,
}

impl RepairRobotControl {
    pub fn new(program: ProgramMemory) -> Self {
        let map = HashMap::new();

        let identifier = String::from("Robot");
        let mut thread = IntcodeThread::new(program, Some(identifier));
        thread.hide_debug_messages = true;

        Self { map, thread }
    }

    fn move_robot_towards(&mut self, dir: Direction) -> BlockType {
        // Task protocol dictates to first send a direction and then receive the block the robot stands on.
        self.thread.send(dir.into());
        self.thread.recv().unwrap().into()
    }

    fn reveal_map_from(&mut self, pos: Coordinate) {
        use Direction::*;

        for &dir in [North, West, South, East].iter() {
            let pos = pos + dir;

            if self.map.contains_key(&pos) {
                // This field is already revealed, it can be ignored.
                continue;
            }

            // Moving towards the field yields the BlockType of it.
            let block = self.move_robot_towards(dir);
            self.map.insert(pos, block);

            if block != BlockType::Wall {
                // If the field is anything but a wall, then the robot now stands on this field.
                // The search is recursively continued from the current field.
                self.reveal_map_from(pos);

                // Once this is done, we move the robot back to where it came from.
                // Similarly, if the field was wall, the robot would not have moved in it,
                // so it does not need to be moved back.
                self.move_robot_towards(dir.inverse());
            }
        }
    }

    /// Use the information from the given intcode program to reveal the map and save it for later use.
    /// Earlier entries in self.map are cleared, since the keys / coordinates are based on the starting position of the robot.
    /// This position is assumed to be (0, 0) from now on.
    /// The internal method reveal_map_from uses depth-first search as it corresponds to the most efficient movement of the robot.
    pub fn reveal_map(&mut self) {
        self.map.clear();
        self.reveal_map_from(Coordinate(0, 0));
    }

    /// Find the position of the oxygen station according to the currently revealed map.
    pub fn oxygen_station_position(&self) -> Option<&Coordinate> {
        self.map
            .iter()
            .find(|&(_k, &v)| v == BlockType::OxygenSystem)
            .map(|(k, _v)| k)
    }

    /// Calculate the distances of every non-wall block from a starting point.
    /// Since every block is exactly one unit apart from every neighbour, breadth-first search guarantees that
    /// we will not encounter better solutions (that is: a shorter path) later.
    pub fn calculate_dists_from_position(&self, start: Coordinate) -> HashMap<Coordinate, usize> {
        use Direction::*;

        let mut dists_from_start = HashMap::new();
        dists_from_start.insert(start, 0);

        // The queue to hold the waiting nodes to be continued from.
        let mut waiting = VecDeque::new();
        waiting.push_back(start);

        while !waiting.is_empty() {
            let cur = waiting.pop_front().unwrap();

            // The distance to the neighbours of the current block.
            let dist = *dists_from_start.get(&cur).unwrap() + 1;

            for &dir in [North, West, South, East].iter() {
                // The position of the currently observed neighbour.
                let pos = cur + dir;

                // If this neighbour is a wall, we do not care about its distance.
                if *self.map.get(&pos).unwrap() == BlockType::Wall {
                    continue;
                }

                // If we already found a distance earlier, we can ignore this neighbour,
                // sincethis path is automatically worse (see function documentation).
                if dists_from_start.contains_key(&pos) {
                    continue;
                }

                // Otherwise, keep track of this neighbour and enqueue it to check its neighbours later.
                dists_from_start.insert(pos, dist);
                waiting.push_back(pos);
            }
        }

        // Finally, return all the collected distances.
        dists_from_start
    }

    /// Print the map.
    pub fn print_map(&self) {
        let coordinates: Vec<&Coordinate> = self.map.keys().collect();
        let xs: Vec<Axis> = coordinates.iter().map(|coord| coord.0).collect();
        let ys: Vec<Axis> = coordinates.iter().map(|coord| coord.1).collect();

        let x_min = *xs.iter().min().unwrap();
        let x_max = *xs.iter().max().unwrap();
        let y_min = *ys.iter().min().unwrap();
        let y_max = *ys.iter().max().unwrap();

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let coord = Coordinate(x, y);
                let c: char = if coord == Coordinate(0, 0) {
                    'o'
                } else {
                    (*self.map.get(&coord).unwrap_or(&BlockType::Unknown)).into()
                };
                print!("{}", c);
            }
            println!();
        }
    }

    /// Actually solve the given task:
    /// - Find the smallest distance between the starting position and the oxygen station.
    /// - Find the distance from the oxygen station to the walkable coordinate that is furthest away.
    pub fn run(&mut self) {
        // This is the only part where the robot / the intcode program is actually used.
        self.reveal_map();
        self.print_map();

        // The position of the oxygen station.
        let oxygen_station = self
            .oxygen_station_position()
            .expect("Oxygen station was not found / is not reachable from origin!");

        let dists_from_origin = self.calculate_dists_from_position(Coordinate(0, 0));
        println!(
            "The distance of the oxygen station is {}",
            dists_from_origin.get(oxygen_station).unwrap()
        );

        let dists_from_station = self.calculate_dists_from_position(*oxygen_station);
        println!(
            "The furthest distance from the station is {}",
            dists_from_station.values().max().unwrap()
        );
    }
}
