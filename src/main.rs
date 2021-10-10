use clap::{App, load_yaml};
use collatz_cli::arg;
use collatz_cli::collatz::Branch;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let branch: i32 = arg::get_arg(matches.value_of("branches"));
    let deviation: i32 = arg::get_arg(matches.value_of("deviation"));
    let limit: i32 = arg::get_arg(matches.value_of("limit"));

    let mut initial_branch = collatz_cli::collatz::Branch {
        spawn_number: 1,
        start_number: 1,
        deviation,
        limit,
        numbers: vec![],
    };

    initial_branch.numbers = initial_branch.create_branch_numbers(limit, 1);

    let alternate: Vec<Branch> = initial_branch.check_for_branch(&initial_branch.numbers);
    println!("{}", alternate.len())
}
