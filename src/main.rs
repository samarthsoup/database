use std::io::{self, Write};
use std::fs::{self, File};

fn main() {
    let d_types = ["number", "string", "boolean"]; 
    loop{
        print!(": ");
        io::stdout().flush().unwrap();


        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("read error");
            continue; 
        }
        let input = input.trim(); 

        match input {
            cmd if cmd.starts_with("new ") => {
                let tokens: Vec<String> = cmd.split_whitespace().map(String::from).collect();

                //checking if cmd is of right format:
                //new name attribute_name1:data_type, attribute_name2:data_type
                if tokens.len() != 3 {
                    eprintln!("parser error: incorrect number of arguments in operation");
                    continue;
                }

                let mut error_occurred = false;
                let attributes: Vec<String> = tokens[2].split(',').map(String::from).collect();

                for attr in attributes {
                    let Some((_, d_type)) = attr.split_once(':') else {todo!()};
                    if !d_types.contains(&d_type) {
                        eprintln!("parser error: incorrect data type");
                        error_occurred = true;
                        break;
                    }
                }

                if error_occurred {
                    continue;
                }

                let header_filename = format!("data/{}.h", tokens[1]);
                let mut header_file = File::create(header_filename).unwrap();
                header_file.write_all(tokens[2].as_bytes()).unwrap();

                let data_filename = format!("data/{}.data", tokens[1]);
                File::create(data_filename).unwrap();
            },
            cmd if cmd.starts_with("delete ") => {
                let Some((_, filename)) = cmd.split_once(' ') else {todo!()};
                let filename = filename.trim();
                let header_filename = format!("data/{}.h", filename);
                let data_filename = format!("data/{}.data", filename);

                fs::remove_file(header_filename).unwrap();
                fs::remove_file(data_filename).unwrap();
            },
            cmd if cmd.starts_with("add ") => {
                //TODO: add row into active table
            },
            cmd if cmd.starts_with("remove ") => {
                //TODO: remove row from active table
            },
            _ => todo!(),
        }
    }
}
