use std::env::args;

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let output = if args.is_empty() {
        "y"
    } else {
        &args.join(" ")
    };
    loop {
        println!("{output}");
    }
}
