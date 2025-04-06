fn draw_tree(levels: usize) {
    let max_width = 2 * levels + 1;

    for i in 0..levels {
        let height = i + 1;

        for row in 0..height {
            let stars = 2 * row + 1;
            let spaces = (max_width - stars) / 2;

            println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
        }
    }
}

fn main() {
    let triangles = 5; // Кількість трикутників
    draw_tree(triangles);
}
