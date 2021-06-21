mod url;

use clap::{Arg, App};

fn main() {
    let matches = App::new("AmazonUrlBeautify")
        .version("1.0")
        .author("@shiomiyan")
        .about("Simple CLI tool for beautify Amazon URL.")
        .arg(
            Arg::with_name("URL")
                .help("Product URL on Amazon.co.jp.")
        )
        .get_matches();

        if let Some(url) = matches.value_of("URL") {
            let result = url::parser::get_asin(url);
            println!("https://www.amazon.co.jp/dp/{}", result);
        }
}
