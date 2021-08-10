use clap::{App, load_yaml};
use collatz_cli::arg;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let  branch: i32 = arg::get_arg(matches.value_of("branches"));
    let  deviation: i32 = arg::get_arg(matches.value_of("deviation"));
    let  limit: i32 = arg::get_arg(matches.value_of("limit"));

    let mut test = collatz_cli::collatz::Branch {
        spawn_number: 1,
        branch_number: 1,
        deviation,
        limit,
        numbers: vec![],
        linked_branches: vec![]
    };

    test.numbers = collatz_cli::collatz::Branch::create_branch_numbers(limit, test.spawn_number as i32);

    let alternate = collatz_cli::collatz::check_for_branch(test.numbers);

}
