fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    let n = n.rem_euclid(len as isize) as usize;
    let (left, right) = s.split_at(len - n);
    format!("{}{}", right, left)
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(
                rotate(s.to_string(), *n),
                exp.to_string()
            );
            println!("rotate({}, {}) = {}", s, n, rotate(s.to_string(), *n));
        });
}
