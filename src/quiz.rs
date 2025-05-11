use crate::util::read_input;
use rand::prelude::*;

pub fn ask_question(question: &str, correct_answer: &str) -> bool {
    println!("{}", question);

    let user_answer = read_input();

    if user_answer.is_empty() {
        println!("Incorrect! The correct answer is {}", correct_answer);
        return false;
    }

    if user_answer == correct_answer {
        println!("Correct!");
        true
    } else {
        println!("Incorrect! The correct answer is {}", correct_answer);
        false
    }
}

pub fn practice_problems(problems: &[(&str, &str)]) {
    let mut score = 0;
    for (question, correct_answer) in problems {
        let correct = ask_question(question, correct_answer);
        if correct {
            score += 1;
        }
    }
    println!("You got {} out of {} correct!", score, problems.len());
}

pub fn practice_problems_in_random_order(problems: &[(&str, &str)]) {
    let mut rng = thread_rng();
    let mut shuffled = problems.to_vec();
    shuffled.shuffle(&mut rng);
    practice_problems(&shuffled);
}
