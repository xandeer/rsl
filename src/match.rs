fn main() {
    tuples();
    enums();
    structs();
    guards();
    binding();
}

fn tuples() {
    let pair = (0, 3);

    match pair {
        (0, y) => println!("First is `0` and `y` is `{}`", y),
        (x, 0) => println!("`x` is `{}` and last is `0`", x),
        _ => println!("It doesn't matter what they are.")
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
}

fn enums() {
    let color = Color::RGB(12, 34, 44);

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Green => println!("The color is Green!"),
        Color::Blue => println!("The color is Blue!"),
        Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
    }
}

struct Foo {
    x: (u32, u32),
    y: u32
}

fn structs() {
    let foo = Foo { x: (1, 2), y: 9 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1 and b is {}, y is {}", b, y),
        Foo { x, y: 9 } => println!("y is `9` and x is `{:?}`", x),
        Foo { y, .. } => println!("y is {}, we don't care about x.", y),
    }
}

fn guards() {
    let pair = (1, -1);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation...")
    }
}

fn binding() {
    match age() {
        0 => println!("I'm not born yet I guess."),
        n @ 1 ..= 12 => println!("I'm a child of age {}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {}", n),
        n => println!("I'm a old person of age {}", n),
    }

    match some_number() {
        Some(n @ 23) => println!("The answer is {}", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(23)
}
