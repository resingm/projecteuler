mod problems;

fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <problem_number>", args[0]);
        std::process::exit(1);
    }

    let problem_number = &args[1];

    match problem_number.as_str() {
        "1" => problems::problem_001::solve(),
        "2" => problems::problem_002::solve(),
        "3" => problems::problem_003::solve(),
        "4" => problems::problem_004::solve(),
        "5" => problems::problem_005::solve(),
        "6" => problems::problem_006::solve(),
        "7" => problems::problem_007::solve(),
        "8" => problems::problem_008::solve(),
        "9" => problems::problem_009::solve(),
        "10" => problems::problem_010::solve(),
        "11" => problems::problem_011::solve(),
        _ => eprintln!("Problem not implemented."),
    }
}
