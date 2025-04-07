const W: u32 = 11; // ширина
const H: u32 = 11; // висота

fn diamond(w: u32, h: u32) {
    let mut result = String::new();
    let mid = h as i32 / 2;

    for y in 0..h as i32 {
        let dy = (mid - y).abs();
        let stars = w as i32 - 2 * dy;
        let spaces = (w as i32 - stars) / 2;

        for _ in 0..spaces {
            result.push(' ');
        }
        for _ in 0..stars {
            result.push('*');
        }
        result.push('\n');
    }

    print!("{result}");
}

#[test]
fn test_diamond() {
    diamond(W, H);
}


