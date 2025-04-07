fn envelope(w: u32, h: u32) {
    let k = w as f32 / h as f32;
    for y in 1..=h {
        for x in 1..=w {
            let is_hor: bool = y == 1 || y == h;
            let is_ver: bool = x == 1 || x == w;
            let is_diag: bool = x == (y as f32 * k).round() as u32;
            let is_diag2: bool = w - x + 1 == (y as f32 * k).round() as u32;
            let show: bool = is_ver || is_hor || is_diag || is_diag2;
            let c: char = if show { '*' } else { ' ' };
            print!("{c}");
        }
        println!();
    }
}

#[test]
fn test_double() {
    let a: f64 = 0.1;
    let b: f64 = 0.2;
    let x: f64 = a + b;
    println!("{a} + {b} = {x}");
}

#[test]
fn test_envelope() {
    envelope(10, 10);
    envelope(20, 10);
    envelope(23, 10);
    envelope(25, 10);
    envelope(30, 10);
}
