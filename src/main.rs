use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let json_filename = &args[1];
        let json_content = fs::read_to_string(&mut json_filename.trim_end()).unwrap();

        println!("{}", json_content);
    } else {
        println!("No json file was provided as argument!");
    }
}
