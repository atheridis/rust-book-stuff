pub trait Summary {
    // Without logic
    // fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    // Having a default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    // If we were importing
    // You could also use it for your own type
    //use crate::Summary;

    let tweet = Tweet {
        username: String::from("Yorugo"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    println!("1 new tweet from {}", tweet.summarize());

    notify(&tweet);
}

// This function and the next mean the same thing
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// We can force both items to be of the same type T
// pub fn notify(item1: &impl Summary, item2 &impl Summary)
// pub fn notify<T: Summary>(item1: &T, item2: &T)

// item must implement both Display and Summary
// pub fn notify(item: &(impl Summary + Display))
// pub fn notify<T: Summary + Display>(item: &T)

// The second way is less cluttered
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
//     0
// }

// fn some_other_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     0
// }

// We can also return types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
