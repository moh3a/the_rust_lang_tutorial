#[allow(dead_code)]
pub fn run() {
    println!("hello from traits");

    let tweet = Tweet {
        username: String::from("moh3a"),
        content: String::from("hello world!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("moh aaa"),
        content: String::from("lorem ipsum and other shit"),
        headline: String::from("major hit"),
    };

    println!("Tweet summary: {}", tweet.summerize());
    println!("Article summary: {}", article.summerize());
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    // we can leave it like this
    // fn summerize(&self) -> String;
    // or we can add a default implementation
    fn summerize(&self) -> String {
        String::from("(Read more...)")
    }
}

// if we dont implement a custom summerize the default one will be used
impl Summary for NewsArticle {}

// to implement the summary trait for our tweet
impl Summary for Tweet {
    fn summerize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
