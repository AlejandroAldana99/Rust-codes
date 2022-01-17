pub fn add(a: i32, b: i32) -> i32 {
    private_adder(a, b)
}

fn private_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn rest(a: i32, b: i32) -> i32 {
    private_rester(a, b)
}

fn private_rester(a: i32, b: i32) -> i32 {
    a - b
}

pub fn div(a: i32, b: i32) -> i32 {
    private_divider(a, b)
}

fn private_divider(a: i32, b: i32) -> i32 {
    a / b
}

pub fn multi(a: i32, b: i32) -> i32 {
    private_multipler(a, b)
}

fn private_multipler(a: i32, b: i32) -> i32 {
    a * b
}