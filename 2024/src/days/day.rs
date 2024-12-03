use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub trait Day {

    fn day_number(&self) -> &str;
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;

    fn load_input(&self) -> String {
        let day = self.day_number();
        let cache_path = format!("input/day{}.txt", day);
        let cache_file = Path::new(&cache_path);

        // If the cache file exists, read the input from it
        if cache_file.exists() {
            fs::read_to_string(cache_file).expect("Failed to read cache file")
        } else {
            // Otherwise, fetch the input from the server
            let input = self.fetch_input_from_server();
            // Save the input to the cache file
            fs::create_dir_all("input").expect("Failed to create input directory");
            let mut file = File::create(cache_file).unwrap_or_else(|_| panic!("Failed to create cache file at {}", cache_path));
            file.write_all(input.as_bytes()).expect("Failed to write cache file");
            input
        }
    }

    fn fetch_input_from_server(&self) -> String {

        // Get the day number
        let day = self.day_number();
        // Construct the URL for the input
        let url = format!("https://adventofcode.com/2024/day/{}/input", day);

        // Retrieve the session token from the environment
        let session = read_aoc_session_token_from_file(".aoc_session_token").expect("Failed to read AOC_SESSION token from file");

        // Initialize the HTTP client with a custom User-Agent
        let client = Client::builder()
            .user_agent("Rust Advent of Code Solution")
            .build()
            .expect("Failed to build HTTP client");

        // Send the GET request with the session cookie
        let response = client
            .get(&url)
            .header(COOKIE, format!("session={}", session))
            .send()
            .expect("Failed to send request")
            .error_for_status()
            .expect("Received error status from server");

        // Return the response body as a String
        response.text().expect("Failed to read response text")
    }
}


fn read_aoc_session_token_from_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut session = String::new();
    file.read_to_string(&mut session)?;
    Ok(session.trim().to_string())
}
