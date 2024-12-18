#![allow(dead_code)]
#![warn(clippy::explicit_reinitialization)]

fn test_copy() {
    let mut x = 1;
    println!("{x}");
    x = 2;
    println!("{x}");
}

fn test_move() {
    let mut x = String::new();
    println!("{x}");
    x = String::new();
    println!("{x}");
}

#[allow(unused_assignments, clippy::misrefactored_assign_op)]
fn should_lint() {
    let mut a = 1;
    a = a * a * a;
}

#[allow(clippy::ptr_arg)]
fn should_lint_2() {
    fn foo(_x: &mut String) {}
    let mut s = String::new();
    s = s.replacen('o', "a", 3);
    foo(&mut s);
    drop(s);
}

// require liveness analysis
#[allow(clippy::ptr_arg, clippy::unnecessary_literal_unwrap, clippy::useless_format)]
fn should_lint_3() {
    fn foo(_x: &mut String) {}
    if true {
        let mut s = Result::<String, ()>::Ok(String::new()).unwrap_or_else(|_| panic!());
        let mod_decl = format!("\nmod");
        s = s.replacen(&mod_decl, "a", 3);
        foo(&mut s);
    }
    println!("123");
}

fn should_not_lint_1() {
    let mut x = 1;
    loop {
        println!("{x}");
        x = 2;
    }
}

fn should_not_lint_2() {
    let mut x = String::new();
    loop {
        println!("{x}");
        x = String::new();
    }
}

fn should_not_lint_3() {
    let mut x = String::new();
    if true {
        x = String::new();
    } else {
        x = x.trim().to_owned();
    }
    drop(x)
}

fn should_not_lint_4() {
    let mut b = true;
    loop {
        b = !b;
    }
}

fn should_not_lint_5(mut v: Vec<String>) -> Vec<String> {
    for _ in 0..10 {
        if true {
            v = Vec::new();
            continue;
        }
        v.push(String::new());
    }
    v
}

fn known_false_negative() {
    let mut x = 1;
    println!("{x}");
    {
        x = 2;
    }
    println!("{x}");
}

fn false_positive() {
    let mut x = 1;
    println!("{x}");

    if true {
        x = 2;
        println!("{x}");
    } else {
        x = 3;
        println!("{x}");
    }
}

fn main() {}

