use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;


fn main() {
    usage();
    println!("{}", get_title());
    print_long_banner();

    let args: Vec<String> = std::env::args().collect();
    match args.len() { //match is like switch case
        2 => parse_markdown_file(&args[1]), // for 2 do this
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            usage();
        } // otherwise do usage()
    }
}

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    return the_title;
}

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    print_short_banner();
    println!("[ INFO ] Starting parser!");

    // Create a path variable from the filename
    let input_filename = Path::new(_filename);

    // Try to open the file
    let file = File::open(&input_filename)
        .expect("[ ERROR ] Failed to open file!");

    let mut _ptag: bool = false;
    let mut _htag: bool = false;

    // Create a place to store all our tokens
    let mut tokens: Vec<String> = Vec::new();

    // Read the file line-by-line
    let reader = BufReader::new(file);

}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    let mut written_by = String::from("Written by: ");
    written_by.push_str(env!("CARGO_PKG_AUTHORS"));
    let mut homepage = String::from("Homepage: ");
    homepage.push_str(env!("CARGO_PKG_HOMEPAGE"));
    let mut usage = String::from("Usage: tinymd <somefile>.md");
    println!("{}", written_by);
    println!("{}", homepage);
    println!("{}", usage);
}

fn usage() {}

fn get_version() -> u16 {
    1000
}

