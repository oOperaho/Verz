use std::env;
use roux::Subreddit;
use tokio;

#[tokio::main]
async fn main() {
    let s: Vec<String> = env::args().collect();

    if s.len() == 2 {
        let title = &s[1];

        let sub = Subreddit::new(title);

        let hot_posts = sub.hot(30, None).await;

        println!("{:?}", hot_posts);
    }   
}
