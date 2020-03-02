use intcode_computer::run_program_from_file;

fn main() {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap();
    run_program_from_file(&filename);
}
