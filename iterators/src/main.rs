use iterators::Counter;

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2 = v1.iter().map(|x| 2 * x - 1);
    let v2_col: Vec<_> = v2.clone().collect();

    println!("{:?}", v2_col);

    let mut c = 0;

    let vv = vec!['a', 'b', 'c'];

    let v: Vec<_> = vv
        .iter()
        .map(|letter| {
            c += 1;
            (letter, c)
        })
        .rev()
        .collect();

    // let v: Vec<_> = v.into_iter().rev().collect();

    println!("{:?}", v);

    let mut counter = Counter::new();
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter);
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter);
}
