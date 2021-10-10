pub mod collatz {
    pub struct Branch {
        pub spawn_number: u64,
        pub start_number: u64,
        pub counter: i32,
        pub deviation: i32,
        pub limit: i32,
        pub numbers: Vec<u64>,
    }

    impl Branch {

        pub fn create_branch_numbers(&self, limit: i32, start_number: i32) -> Vec<u64> {
            let mut result: Vec<u64> = Vec::new();
            let start: u64 = 2;
            for i in 0..limit {
                result.push((start_number as u64)*(start.pow(i as u32)));
            }
            return result;
        }

        pub fn check_for_branch(&self, numbers: &Vec<u64>, max_branches: i32) -> Vec<Branch>{
            let mut new_branches: Vec<Branch> = Vec::new();
            let mut counter = 1;
            while counter != max_branches {
                for x in 0..numbers.len() {
                    let result = ((numbers[x] as f64) - 1.0) / 3.0;
                    let result_str = result.to_string();
                    if result == 0 as f64 { continue; }
                    else if numbers[x] == 4 { continue; }
                    else if result_str.contains(".") { continue; }
                    else {
                        let branch = Branch {
                            spawn_number: numbers[x],
                            start_number: result as u64,
                            counter,
                            deviation: self.deviation,
                            limit: self.limit,
                            numbers: self.create_branch_numbers(self.limit, result as i32)
                        };
                        new_branches.push(branch);
                    }
                }
                counter = counter + 1;
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