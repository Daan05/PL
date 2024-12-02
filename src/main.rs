use std::process;

fn emit_ln(str: String) {
    println!("{}", str);
}

fn error(str: String) {
    println!("{} {}.", "Error: ", str);
}

fn abort(str: String) {
    error(str);
    process::exit(1);
}

fn expected() {}

fn read() {}

fn is_digit() -> bool {
    return false;
}

fn is_alpha() {}

fn get_number() -> char {
    if !is_digit() {
        expected();
    }

    get_char();
    return '1';
}

fn get_char() {
    read();
}

fn init() {
    get_char();
}

fn expression() {
    emit_ln(format!("{}{}{}", "mov ", "EAX", get_number()));
}

fn main() {
    init();
    expression();
}
