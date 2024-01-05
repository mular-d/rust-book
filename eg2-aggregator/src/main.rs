use eg2_aggregator::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.loading());
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("{}", article.loading());
    println!("New article available! {}", article.summarize());

    let _r = returns_summarizable();
}

pub fn notify(item: &impl Summary) {
    // notify<T: Summary>(item: &T) // more verbose version
    // notify(item: &(impl Summary + Display)) // multiple trait bounds
    // notify<T: Summary + Display>(item: &T)
    println!("Breaking news! {}", item.summarize());
}

// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where
//         T: Display + Clone,
//         U: Clone + Debug,
// {
//     unimplemented!()
// }

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
