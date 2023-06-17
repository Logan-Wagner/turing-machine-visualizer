use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    dbg!(file_path);
}
