mod batter_stats;
use batter_stats::Statistics;

fn main() {
    let response = reqwest::blocking::get("https://statsapi.mlb.com/api/v1/people/630105/stats?stats=career,careerAdvanced&group=hitting").unwrap();
    let data: Statistics = response.json().unwrap();
    println!("{:?}", data);
}