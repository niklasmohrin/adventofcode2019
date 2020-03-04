use arcade_cabinet::{ArcadeCabinet, Tile};
use intcode_computer::read_program_from_file;

fn main() {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap_or(String::from("input.txt"));

    let program = read_program_from_file(&filename);
    let cabinet = ArcadeCabinet::new();
    cabinet.run(program, 2);
    println!("Block tiles: {}", cabinet.count_tile(Tile::Block));
}
