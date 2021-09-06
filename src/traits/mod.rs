use std::fmt::{Debug, Display};

#[allow(dead_code)]
pub fn execute() {
    let news_article = NewsArticle {
        headline: "PlayStation Plus games for September 2021".to_string(),
        location: "Japan".to_string(),
        author: "Playstation Blog".to_string(),
        content: "Overcooked: All You Can Eat!, Hitman 2, Predator: Hunting Grounds".to_string(),
    };
    notify(&news_article);

    let tweet = Tweet {
        username: "bandrefilipe".to_string(),
        content: "Overcooked, Hitman 2 and Predator HG are the PS+ games of this month!"
            .to_string(),
        reply: false,
        retweet: false,
    };
    notify(&tweet);

    notify_allowing_different_types(&news_article, &tweet);
    notify_requiring_same_type(&news_article, &news_article);

    using_trait_bounds_to_conditionally_implement_methods();
}

// Defining a Trait:
trait Summary {
    fn summarize(&self) -> String {
        // Default implementation:
        format!("(Read more from {}...)", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

// Implementing a Trait on a Type:
#[allow(dead_code)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

#[allow(dead_code)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as Parameters:
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_allowing_different_types(item1: &impl Summary, item2: &impl Summary) {
    notify(item1);
    notify(item2);
}

fn notify_requiring_same_type<T: Summary>(item1: &T, item2: &T) {
    notify(item1);
    notify(item2);
}

#[allow(dead_code)]
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Clone + Display,
    U: Clone + Debug,
{
    println!("{}", t.clone());
    println!("{:?}", u.clone());
    42
}

#[allow(dead_code)]
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    }
}
fn using_trait_bounds_to_conditionally_implement_methods() {
    struct Pair<T> {
        first: T,
        second: T,
    }
    impl<T> Pair<T> {
        fn new(first: T, second: T) -> Self {
            Self { first, second }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.first >= self.second {
                println!("The largest member is first = {}", self.first);
            } else {
                println!("The largest member is second = {}", self.second);
            }
        }
    }

    let pair = Pair::new("a", "b");
    pair.cmp_display();
}
