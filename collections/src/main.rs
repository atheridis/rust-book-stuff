use std::collections::HashMap;
use std::ops::Add;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(5);

    let mut v = vec![1, 2, 5];

    v.push(50);
    v.push(100);

    let third: i32 = v[2];
    v.push(2);
    println!("The third element is {}", third);

    for i in &mut v {
        *i += 50;
        println!("{}", *i);
    }

    match v.get(7) {
        Some(third) => println!("The third elment is {}", third),
        None => println!("There is no third element"),
    }

    println!("{:?}", &27.to_string());

    let a = Test { x: 5.3, y: 2.7 };

    let b = Test { x: 7.8, y: 4.3 };

    let c = a + b;
    let s = format!("{:?}", c);
    println!("{}", s);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Replace the old Yellow value
    scores.insert(String::from("Yellow"), 22);

    // Insert only if the key has no value
    scores.entry(String::from("Yellow")).or_insert(33);

    println!("{:?}", scores.get("NNN"));

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Update a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    //

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, field_value.clone());
    println!("{:?}", map);
    println!("{}", field_name);
}

#[derive(Debug)]
struct Test {
    x: f64,
    y: f64,
}

impl Add for Test {
    type Output = Self;

    fn add(self, other: Test) -> Test {
        Test {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
