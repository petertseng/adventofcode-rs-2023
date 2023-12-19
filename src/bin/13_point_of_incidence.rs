fn reflect(xs: &[Vec<bool>], mult: usize) -> Vec<(usize, usize)> {
    use std::collections::VecDeque;

    let mut ls: VecDeque<&Vec<bool>> = VecDeque::with_capacity(xs.len());
    let mut rs = xs.iter().collect::<VecDeque<&Vec<bool>>>();

    (1..xs.len()).map(|i| {
        ls.push_front(rs.pop_front().unwrap());
        (mult * i, ls.iter().zip(rs.iter()).map(|(l, r)| {
            l.iter().zip(r.iter()).filter(|(a, b)| a != b).count()
        }).sum())
    }).collect()
}

fn main() {
    let lines = adventofcode::read_input_lines(|l| {
        l.chars().map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("invalid char {c}"),
        }).collect::<Vec<_>>()
    });
    let pats = lines.split(|n| n.is_empty()).collect::<Vec<_>>();

    let mut p1 = 0;
    let mut p2 = 0;

    for pat in pats {
        let mut errors = reflect(&pat, 100);
        let transpose = (0..pat[0].len()).map(|x| {
            pat.iter().map(|row| row[x]).collect::<Vec<bool>>()
        }).collect::<Vec<_>>();
        errors.append(&mut reflect(&transpose, 1));

        let mut found0 = false;
        let mut found1 = false;

        for (i, err) in errors {
            if err == 0 {
                assert!(!found0);
                p1 += i;
                found0 = true;
            } else if err == 1 {
                assert!(!found1);
                p2 += i;
                found1 = true;
            }
        }

        assert!(found0);
        assert!(found1);
    }

    println!("{p1}");
    println!("{p2}");
}
