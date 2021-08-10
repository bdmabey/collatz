pub mod collatz {
    pub struct Branch {
        pub spawn_number: u64,
        pub branch_number: i32,
        pub deviation: i32,
        pub limit: i32,
        pub numbers: Vec<u64>,
        pub linked_branches: Vec<Branch>
    }

    impl Branch {
        pub fn create_branch_numbers(limit: i32, coefficient: i32) -> Vec<u64> {
            let mut result: Vec<u64> = Vec::new();
            let start: u64 = 2;
            for i in 0..limit {
                result.push((coefficient as u64)*(start.pow(i as u32)));
            }
            return result;
        }

        pub fn check_for_branch(numbers: Vec<u64>) -> Vec<Branch>{
            let mut new_branches: Vec<Branch> = Vec::new();
            for x in 0..numbers.len() {
                let result = ((numbers[x] as f64) - 1.0) / 3.0;
                let result_str = result.to_string();
                if result == 0 as f64 { continue; }
                else if numbers[x] == 4 { continue; }
                else if result_str.contains(".") { continue; }
                else {
                    let branch = Branch {
                        spawn_number: numbers[x],
                        branch_number: 0,
                        deviation: 0,
                        limit: 0,
                        numbers: vec![],
                        linked_branches: vec![]
                    };
                    new_branches.push(branch);
                }
            }
            return new_branches;
        }
    }
}

pub mod arg {
    pub fn get_arg(x: Option<&str>) -> i32 {
        return match x {
            None => panic!("Nothing given"),
            Some(s) => {
                match s.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => panic!("Not a number")
                }
            }
        }
    }
}