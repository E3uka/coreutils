use crate::common::util::*;
use std::io::{Seek, SeekFrom, Write};

static TFILE1: &'static str = "truncate_test_1";
static TFILE2: &'static str = "truncate_test_2";

#[test]
fn test_increase_file_size() {
    let (at, mut ucmd) = at_and_ucmd!();
    let mut file = at.make_file(TFILE1);
    ucmd.args(&["-s", "+5K", TFILE1]).succeeds();

    file.seek(SeekFrom::End(0)).unwrap();
    assert!(file.seek(SeekFrom::Current(0)).unwrap() == 5 * 1024);
}

#[test]
fn test_increase_file_size_kb() {
    let (at, mut ucmd) = at_and_ucmd!();
    let mut file = at.make_file(TFILE1);
    ucmd.args(&["-s", "+5KB", TFILE1]).succeeds();

    file.seek(SeekFrom::End(0)).unwrap();
    assert!(file.seek(SeekFrom::Current(0)).unwrap() == 5 * 1000);
}

#[test]
fn test_reference() {
    let scene = TestScenario::new(util_name!());
    let at = &scene.fixtures;
    let mut file = at.make_file(TFILE2);

    scene.ucmd().arg("-s").arg("+5KB").arg(TFILE1).run();

    scene
        .ucmd()
        .arg("--reference")
        .arg(TFILE1)
        .arg(TFILE2)
        .run();

    file.seek(SeekFrom::End(0)).unwrap();
    assert!(file.seek(SeekFrom::Current(0)).unwrap() == 5 * 1000);
}

#[test]
fn test_decrease_file_size() {
    let (at, mut ucmd) = at_and_ucmd!();
    let mut file = at.make_file(TFILE2);
    file.write_all(b"1234567890").unwrap();
    ucmd.args(&["--size=-4", TFILE2]).succeeds();
    file.seek(SeekFrom::End(0)).unwrap();
    assert!(file.seek(SeekFrom::Current(0)).unwrap() == 6);
}

#[test]
fn test_space_in_size() {
    let (at, mut ucmd) = at_and_ucmd!();
    let mut file = at.make_file(TFILE2);
    file.write_all(b"1234567890").unwrap();
    ucmd.args(&["--size", " 4", TFILE2]).succeeds();
    file.seek(SeekFrom::End(0)).unwrap();
    assert!(file.seek(SeekFrom::Current(0)).unwrap() == 4);
}

#[test]
fn test_failed() {
    new_ucmd!().fails();
}

#[test]
fn test_failed_2() {
    let (_at, mut ucmd) = at_and_ucmd!();
    ucmd.args(&[TFILE1]).fails();
}

#[test]
fn test_failed_incorrect_arg() {
    let (_at, mut ucmd) = at_and_ucmd!();
    ucmd.args(&["-s", "+5A", TFILE1]).fails();
}

#[test]
fn test_at_most_shrinks() {
    let (at, mut ucmd) = at_and_ucmd!();
    let mut file = at.make_file(TFILE2);
    file.write_all(b"1234567890").unwrap();
    ucmd.args(&["--size", "<4", TFILE2]).succeeds();
    file.seek(SeekFrom::End(0)).unwrap();
    assert!(file.seek(SeekFrom::Current(0)).unwrap() == 4);
}

#[test]
fn test_at_most_no_change() {
    let (at, mut ucmd) = at_and_ucmd!();
    let mut file = at.make_file(TFILE2);
    file.write_all(b"1234567890").unwrap();
    ucmd.args(&["--size", "<40", TFILE2]).succeeds();
    file.seek(SeekFrom::End(0)).unwrap();
    assert!(file.seek(SeekFrom::Current(0)).unwrap() == 10);
}

#[test]
fn test_at_least_grows() {
    let (at, mut ucmd) = at_and_ucmd!();
    let mut file = at.make_file(TFILE2);
    file.write_all(b"1234567890").unwrap();
    ucmd.args(&["--size", ">15", TFILE2]).succeeds();
    file.seek(SeekFrom::End(0)).unwrap();
    assert!(file.seek(SeekFrom::Current(0)).unwrap() == 15);
}

#[test]
fn test_at_least_no_change() {
    let (at, mut ucmd) = at_and_ucmd!();
    let mut file = at.make_file(TFILE2);
    file.write_all(b"1234567890").unwrap();
    ucmd.args(&["--size", ">4", TFILE2]).succeeds();
    file.seek(SeekFrom::End(0)).unwrap();
    assert!(file.seek(SeekFrom::Current(0)).unwrap() == 10);
}

#[test]
fn test_round_down() {
    let (at, mut ucmd) = at_and_ucmd!();
    let mut file = at.make_file(TFILE2);
    file.write_all(b"1234567890").unwrap();
    ucmd.args(&["--size", "/4", TFILE2]).succeeds();
    file.seek(SeekFrom::End(0)).unwrap();
    assert!(file.seek(SeekFrom::Current(0)).unwrap() == 8);
}

#[test]
fn test_round_up() {
    let (at, mut ucmd) = at_and_ucmd!();
    let mut file = at.make_file(TFILE2);
    file.write_all(b"1234567890").unwrap();
    ucmd.args(&["--size", "*4", TFILE2]).succeeds();
    file.seek(SeekFrom::End(0)).unwrap();
    assert!(file.seek(SeekFrom::Current(0)).unwrap() == 12);
}
