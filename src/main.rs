use std::env;
use roux::Subreddit;
use tokio;

#[tokio::main]
async fn main() {
    let s: Vec<String> = env::args().collect();

    if s.len() == 3 {
        if &s[1] == "help" {
            let h = help();
            println!("{}", h);
        }

        let title = &s[1];
        let sub = Subreddit::new(title);
        if &s[1] == "hot" {
            let hot_posts = hot();
        }
    } else {
        println!("Error! Invalid number of arguments! Enter cargo run help to see the available commands!");
    }
}

fn help() -> String {
    String::from("-- List of Commands --
        Enter 'cargo run [name-of-subreddit] [sort-by-x]' to use Verz!
        help: You're seeing it right now!
        hot: Shows the hot posts of the subreddit within 30 limit range
        new: Displays the new posts of the subreddit within 15 limit range")
}

fn hot(&str) -> Result<String, Error> {
    // snip
}
