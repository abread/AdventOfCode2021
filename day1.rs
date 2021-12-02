use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    let mut prev = i64::MAX;
    let mut count = 0_usize;
    while io::stdin().read_line(&mut input)? != 0 {
        // drop the \n
        assert_eq!(input.pop(), Some('\n'));

        let num: i64 = input.parse().expect("parse error");
        if num > prev {
            count += 1;
        }

        prev = num;
        input.clear();
    }

    dbg!(count);
    Ok(())
}
