use std::io;
use std::fs;
use std::collections::HashMap;

pub fn run(file_name: &str) -> std::io::Result<()> {
    match fs::read_to_string(file_name) {
        Ok(_) =>  {
            let file = fs::read_to_string(file_name)?;
            parse(&file);
        }
        
        Err(_) => {
            println!("[EXCEPTION] Could not find file '{}'", file_name);
        }
    }
    Ok(())
}

fn parse(code_str: &str) {
    let code: Vec<_> = code_str.split('\n').map(ToOwned::to_owned).collect();
    let mut vars: HashMap<String, String> = HashMap::new();
    for (i, line) in code.iter().enumerate() {
        let line_vec: Vec<_> = line.split(' ').map(ToOwned::to_owned).collect();
        let line_number = i + 1;

        // if the line is empty or is a comment, ignore it
        if line_vec[0].is_empty() || line.starts_with("//") {
            continue;
        }

        let mut args = line_vec.clone();
        args.remove(0);

        match line_vec[0].as_ref() {
            // println <Text to print>
            "println" => {
                args = args
                .into_iter()
                .map(|arg|
                    if arg.starts_with('{') && arg.ends_with('}') {
                        vars.get(&arg).unwrap().clone()
                    } else {
                     arg
                    }
                )
                .collect();

                let printed_str = args.join(" ");

                println!("{}", printed_str);
            }

            // getln {<varToStore>}
            "getln" => {
                let mut new_var = String::new();
                
                io::stdin()
                    .read_line(&mut new_var)           
                    .expect("[ERROR] Could not read line");
                
                new_var = String::from(new_var.trim());

                vars.insert(args[0].clone(), new_var);
            }

            // throw <Custom error message>
            "throw" => {
                let exception = args.join(" ");

                println!("[ERROR] {}", exception);
                return;
            }

            // let {varName} <value>
            "let" => {
                let mut args = args.clone();
                let var_name = args[0].clone();
                args.remove(0);

                let val = args.join(" ");

                vars.insert(var_name, val);
            }

            &_ => {
                println!("[ERROR] Line {}: Unexpected '{}'", line_number, line);
                return;
            }
        }
    }
}
