use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// NOTE: Box を使うとデータをスタックではなくヒープに格納できる。
// スタックではサイズが既知のデータに決められた順序でアクセスし、
// ヒープでは値のサイズが時間とともに変わる可能性がある。
// dyn は std::error::Error トレイトに対するメソッド呼び出しが動的にディスパッチされることを示す。
// まとめは p.51 を参照。
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    // NOTE: fs::read_to_string は大きなファイルを読み込むと、メモリが足りなくなって
    // プログラムがクラッシュすることがあるらしい。
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
