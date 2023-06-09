
fn solution(mut p: &char) {
    let the_key: char = 'A';
    p = &the_key;
    println!("{}", p);
}

fn main() {
    let ch: char = '\0';
    let p: &char = &ch;
    solution(p);
    println!("{}", p);
}

