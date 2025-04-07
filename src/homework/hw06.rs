fn tree(levels: u32) {
    let total_height: u32 = (1..=levels).sum(); // загальна висота ялинки
    let max_width: u32 = 1 + 2 * (total_height - 1); // ширина останнього рядка

    let mut current_line = 0;

    for level in 1..=levels {
        for row in 0..level {
            let stars = 1 + 2 * row;
            let spaces = ((max_width - stars) / 2) as usize;
            let stars = stars as usize;

            let line: String = std::iter::repeat(' ')
                .take(spaces)
                .chain(std::iter::repeat('*').take(stars))
                .collect();

            println!("{line}");
            current_line += 1;
        }
    }
}

#[test]
fn test_tree() {
    tree(6); // кількість трикутників
}
