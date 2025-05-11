#[test]
fn program_output_contains_correct_questions() {
    use assert_cmd::Command;
    use predicates::str::contains;

    let mut cmd = Command::cargo_bin("quizzer").unwrap();

    cmd.write_stdin("wrong_answer\n")
        .assert()
        .success()
        .stdout(contains("What year was Speedcoding first released?"))
        .stdout(contains("Incorrect! The correct answer is 1953"));
}

#[test]
fn program_output_contains_questions_in_random_order() {
    use assert_cmd::Command;

    let run_once = || {
        let mut cmd = Command::cargo_bin("quizzer").unwrap();
        cmd.write_stdin("dummy\n".repeat(10))
            .assert()
            .success()
            .get_output()
            .stdout
            .clone()
    };

    let output1 = run_once();
    let output2 = run_once();

    assert_ne!(
        output1, output2,
        "Question order should not be same on each run"
    );
}
