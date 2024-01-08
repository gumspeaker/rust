use std::vec;

pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
    let (res1, res2) = (get_score(&player1), get_score(&player2));
    println!("{},{}", res1, res2);
    if res1 > res2 {
        1
    } else if res2 > res1 {
        2
    } else {
        0
    }
}
fn get_score(v: &Vec<i32>) -> i32 {
    let mut flag = 0;
    let res = v.into_iter().fold(0, |sum, current| {
        let current_scope = (if flag > 0 { 2 } else { 1 }) * current;
        if *current == 10 {
            flag = 2;
        } else {
            flag = flag - 1;
        }
        current_scope + sum
    });
    res
}
fn demo() {
    // Creating a week_days vector
    let week_days: Vec<&str> = vec![
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    // using filter() method with days (closure with condition)
    // passed as a parameter

    let filtered_week_days: Vec<_> = week_days
        .into_iter()
        .enumerate()
        .filter(|(index, days)| days.len() < 8)
        .collect();

    println!("{:?}", filtered_week_days);
}
