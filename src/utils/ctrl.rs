use std::io::stdin;
use failure::Fail;

pub fn print_x_errors(err: &dyn Fail, x: i32) {
    eprintln!("{}", err);

    if let Some(cause) = err.cause() {
        if x > 0 {
            print_x_errors(cause, x - 1);
        } else {
            eprintln!("(Rest of trace excluded)");
        }
    }
}

pub fn enter_then_quit(exit_code: i32) -> ! {
    let mut dump = String::with_capacity(1);
    print!("Press enter to close the program");
    stdin().read_line(&mut dump);
    std::process::exit(exit_code)
}
