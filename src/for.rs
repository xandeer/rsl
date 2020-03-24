fn main() {
    implied();
    iter();
    into_iter();
    iter_mut();
}

fn implied() {
    println!("Iter impliedly (same as into_iter):");

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello, {}", name),
        }
    }

    println!("value names borrowed here are moved.");
    // println!("names: {:?}", names);
}

fn iter() {
    println!("Iter:");

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello, {}", name),
        }
    }

    println!("names: {:?}", names);
}

fn into_iter() {
    println!("Into iter:");

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello, {}", name),
        }
    }

    println!("value names borrowed here are moved.");
    // println!("names: {:?}", names);
}

fn iter_mut() {
    println!("Iter mut:");

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
