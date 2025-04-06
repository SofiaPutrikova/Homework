fn main() {
    const HEIGHT: usize = 6;
    const WIDTH: usize = HEIGHT * 2 - 1;

    let mut lines = Vec::new();

    for i in 0..HEIGHT {
        let stars = 2 * i + 1;
        let spaces = (WIDTH - stars) / 2;
        let line = " ".repeat(spaces) + &"*".repeat(stars);
        lines.push(line);
    }

    for i in (0..HEIGHT - 1).rev() {
        let stars = 2 * i + 1;
        let spaces = (WIDTH - stars) / 2;
        let line = " ".repeat(spaces) + &"*".repeat(stars);
        lines.push(line);
    }

    print!("{}", lines.join("\n"));
    println!();
}

