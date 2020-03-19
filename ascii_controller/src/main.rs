use ascii_controller::AsciiController;
use intcode_computer::read_program_from_file;

fn main() {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap_or(String::from("input.txt"));
    let program = read_program_from_file(&filename);
    let mut controller = AsciiController::new(program);
    controller.run_cameras();
    println!("{} moves calcualted", controller.moves_needed().len());
    println!("{} scaffoldings", controller.scaffolding_coords().len());
    // let collected_dust = controller.walk_scaffolding();
    // println!("Dust collected: {}", collected_dust);
}
