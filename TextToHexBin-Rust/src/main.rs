use std::io::Write;

fn main() {
    let mut buffer = String::new();
    
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    print!("Enter text: ");
    stdout.flush().expect("Failed to flush");
    stdin.read_line(&mut buffer).expect("Failed to read a line");

    let buffer = buffer.trim();

    print!("Binary representation: ");
    for element in buffer.as_bytes() {
        print!("{:b} ", element);
    }
    println!();
    stdout.flush().expect("Failed to flush");

    print!("Hexadecimal representation: ");
    for element in buffer.as_bytes() {
        print!("{:x} ", element);
    }
    stdout.flush().expect("Failed to flush");
}
