use std::io;

fn main() {
    // Read lab score from the user
    println!("Enter lab score:");
    let mut lab_score = String::new();
    io::stdin().read_line(&mut lab_score).expect("Failed to read line");
    let lab_score: f64 = lab_score.trim().parse().expect("Invalid input");

    // Read lecture score from the user
    println!("Enter lecture score:");
    let mut lecture_score = String::new();
    io::stdin().read_line(&mut lecture_score).expect("Failed to read line");
    let lecture_score: f64 = lecture_score.trim().parse().expect("Invalid input");

    // Calculate total score (40% lab + 60% lecture)
    let total_score = (0.4 * lab_score) + (0.6 * lecture_score);

    // Determine the grade using a match statement
    let grade = match total_score {
        90.0..=100.0 => "A",
        85.0..=89.9 => "B+",
        80.0..=84.9 => "B",
        75.0..=79.9 => "C+",
        70.0..=74.9 => "C",
        65.0..=69.9 => "D+",
        60.0..=64.9 => "D",
        _ => "F",
    };

    // Print the calculated grade
    println!("Your grade is: {}", grade);
}

