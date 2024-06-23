use std::io::{self, Write, Read, BufReader, BufRead};
use std::fs::{self, File};
use std::error::Error;

enum d_type {
    number,
    string,
    boolean
}

fn read_file_to_string(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_file_to_vector(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(&path)?;
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}

fn write_vec_to_file(data: &Vec<Vec<String>>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for row in data {
        let line = row.join(",") + "\n";
        file.write_all(line.as_bytes())?;
    }

    Ok(())
}

fn input() -> Result<String, Box<dyn Error>> {
    print!(": ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return Err("read error".into());
    }
    Ok(input.trim().to_string())
}

fn validate_d_type_number(x: &String) -> bool {
    x.parse::<f64>().is_ok()
}

fn validate_d_type_string(x: &String) -> bool {
    true
}

fn validate_d_type_boolean(x: &String) -> bool {
    if x == "true" || x == "false" {
        true
    } else {
        false
    }
}

fn main() {
    let mut active_table_name = String::new();
    let mut active_table_header: Vec<d_type> = Vec::new();
    let mut active_table: Vec<Vec<String>> = Vec::new();
    let d_types = ["number", "string", "boolean"]; 
    loop{
        let input = match input() {
            Ok(x) => x,
            Err(e) =>  {
                eprintln!("{e}");
                continue;
            }
        };

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
                let tokens: Vec<String> = cmd.split_whitespace().map(String::from).collect();

                let mut error_occurred = false;

                //if active table isnt the same as the one mentioned in the operation
                if active_table_name != tokens[1] {
                    let path = format!("data/{}.data", active_table_name);
                    write_vec_to_file(&active_table, &path).unwrap();
                    //clear old data
                    active_table.clear();
                    active_table_header.clear();
                    active_table_name = tokens[1].clone();

                    //read contents from header file
                    let path = format!("data/{}.h", tokens[1]);
                    let header_contents = match read_file_to_string(&path) {
                        Ok(x) => x,
                        Err(e) => {
                            eprintln!("{e}");
                            continue;
                        }
                    };

                    //push the data types of each column into the active_table_header vector
                    let attributes: Vec<String> = header_contents.split(',').map(String::from).collect();
                    for i in 0..attributes.len() {
                        let Some((_, d_type)) = attributes[i].split_once(':') else {todo!()};
                        match d_type {
                            "number" => active_table_header.push(d_type::number),
                            "string" => active_table_header.push(d_type::string),
                            "boolean" => active_table_header.push(d_type::boolean),
                            _ => unreachable!(),
                        }
                    }

                    //load all the data from the .data file
                    let path = format!("data/{}.data", tokens[1]);
                    let contents = match read_file_to_vector(&path) {
                        Ok(x) => x,
                        Err(e) => {
                            eprintln!("{e}");
                            continue;
                        }
                    };
                    for i in 0..contents.len() {
                        active_table.push(contents[i].split(',').map(String::from).collect());
                    }
                    
                }
                //this is the argument in the operation that we want to add to the active table
                let values: Vec<String> = tokens[2].split(',').map(String::from).collect();

                //if the columns dont match on the values to add and the active table
                if values.len() != active_table_header.len() {
                    eprintln!("parser error: incorrect number of arguments in operation");
                }

                //validating each value with its data type
                for i in 0..values.len() {
                    match active_table_header[i] {
                        d_type::number => {
                            if !validate_d_type_number(&values[i]) {
                                eprintln!("parser error: incorrect data type");
                                error_occurred = true;
                                break;
                            }
                        },
                        d_type::string => {
                            if !validate_d_type_string(&values[i]) {
                                eprintln!("parser error: incorrect data type");
                                error_occurred = true;
                                break;
                            }
                        },
                        d_type::boolean => {
                            if !validate_d_type_boolean(&values[i]) {
                                eprintln!("parser error: incorrect data type");
                                error_occurred = true;
                                break;
                            }
                        }
                    }
                }

                if error_occurred {
                    continue;
                }

                active_table.push(values);
            },
            cmd if cmd.starts_with("remove ") => {
                //TODO: remove row from active table
            },
            _ => todo!(),
        }
    }
}
