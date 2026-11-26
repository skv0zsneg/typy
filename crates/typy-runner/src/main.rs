fn main() {
    let num = 100;
    match typy_vm::add(num, num) {
        Some(result) => println!("{num} plus {num} is {}!", result),
        None => eprintln!("Arithmetic error"),
    }
}
