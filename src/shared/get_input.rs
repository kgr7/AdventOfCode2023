use std::error::Error;
use getopts::Options;
use std::env;

pub async fn get_input_with_cookie(problem_url: &str) -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("s", "session", "Set a session cookie", "SESSION");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    let session_cookie = matches.opt_str("s").expect("Session not provided").to_string();

    let client = reqwest::Client::new();
    let res = client.get(problem_url)
        .header("Cookie", session_cookie)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}
