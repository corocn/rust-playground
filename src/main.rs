pub trait Summary {
    fn summarize_author(&self) -> String;

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
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub trait Hoge1 {
    fn say1(&self) -> String;
}

impl Hoge1 for NewsArticle {
    fn say1(&self) -> String {
        return String::from("ほげわん");
    }
}

pub trait Hoge2 {
    fn say2(&self) -> String;
}

impl Hoge2 for NewsArticle {
    fn say2(&self) -> String {
        return String::from("ほげつー");
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn test<T: Hoge1 + Hoge2>(item: &T) {
    println!("{} {}", item.say1(), item.say2())
}

fn returns_hoge() -> impl Hoge1 + Hoge2 + Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
    )}
}

fn main() {
    let article = returns_hoge();
    println!("New article available! {}", article.summarize());

    test(&article);

    notify(&article);
}