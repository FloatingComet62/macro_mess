use std::{fs::{File, OpenOptions}, io::{prelude::*, ErrorKind}};

static mut DATA: String = String::new();

macro_rules! log {
    (@core $x: expr, $($e: expr),*) => {
        if DATA.len() == 0 {
            println!("[{}] {}", $x, format!($($e),*));
        } else {
            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(DATA.clone());
            match file {
                Err(e) => {
                    if e.kind() == ErrorKind::NotFound {
                        match File::create(DATA.clone()) {
                            Ok(mut f) => if let Err(e) = f.write(format!("[{}] {}\n", $x, format!($($e),*)).as_bytes()) {
                                println!("[ERROR] Failed to write to file\n{}", e);
                            },
                            Err(e) => println!("[ERROR] Failed to create the file\n{}", e),
                        }
                    } else { println!("[ERROR] Failed to open the file\n{}", e) }
                },
                Ok(mut file) => if let Err(e) = writeln!(file, "{}", format!("[{}] {}", $x, format!($($e),*))) {
                    println!("[ERROR] Failed to open the file\n{}", e)
                }
            }
        }
    };
    (info $($e: expr),*) => {
        log!(@core "INFO", $($e),*)
    };
    (warn $($e: expr),*) => {
        log!(@core "WARN", $($e),*)
    };
    (err $($e: expr),*) => {
        log!(@core "ERROR", $($e),*)
    };
    // make sure this is before (file $x: expr)
    (file clear) => {
        DATA = String::new();
    };
    (file $x: expr) => {
        DATA = $x.to_string();
    };
}

fn main() {
    let mut x = 1;
    unsafe {
        log!(info "Hello, World!");
        log!(file "D:\\macro_mess\\file.txt");
        log!(warn "Hello, World! {}", x);
        log!(file clear);
        x += 1;
        log!(err "Hello, World! {}", x);
    }
}
