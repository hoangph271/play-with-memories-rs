use std::mem;

#[allow(dead_code)]
enum SmolEnum {
    Empty,
    Text(String),
}

#[allow(dead_code)]
enum BigEnum {
    Empty,
    Int(isize),
    Text(String),
}

#[allow(dead_code)]
enum BiggerEnum {
    Empty,
    Int(f64),
    Text(String),
}


#[allow(dead_code)]
enum BiggestEnum {
    Empty,
    Text(String),
    TextAgain(String),
}

fn main() {
    println!("Size of String: {}", mem::size_of::<String>());
    println!("Size of SmolEnum:  {}", mem::size_of::<SmolEnum>());
    println!("Size of BigEnum:  {}", mem::size_of::<BigEnum>());
    println!("Size of BiggerEnum:  {}", mem::size_of::<BiggerEnum>());
    println!("Size of BiggestEnum:  {}", mem::size_of::<BiggestEnum>());
}
