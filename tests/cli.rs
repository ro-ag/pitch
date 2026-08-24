//! End-to-end tests against the built binary.

use std::process::Command;

fn pitch(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_pitch"))
        .args(args)
        .output()
        .expect("binary runs");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn note_to_frequency() {
    let (ok, stdout, _) = pitch(&["A4"]);
    assert!(ok);
    assert_eq!(stdout.trim(), "A4 -> 440.00 Hz (MIDI 69)");
}

#[test]
fn flats_and_low_octaves() {
    let (ok, stdout, _) = pitch(&["Bb3"]);
    assert!(ok);
    assert!(stdout.starts_with("A#3 -> 233.08 Hz"), "got: {stdout}");
}

#[test]
fn frequency_to_note_with_cents() {
    let (ok, stdout, _) = pitch(&["445"]);
    assert!(ok);
    assert_eq!(stdout.trim(), "445.00 Hz -> A4 (MIDI 69), +19.6 cents");
}

#[test]
fn custom_a4_reference() {
    let (ok, stdout, _) = pitch(&["--a4", "432", "A4"]);
    assert!(ok);
    assert_eq!(stdout.trim(), "A4 -> 432.00 Hz (MIDI 69)");
}

#[test]
fn version_and_help() {
    let (ok, stdout, _) = pitch(&["--version"]);
    assert!(ok);
    assert!(stdout.starts_with("pitch 0."), "got: {stdout}");

    let (ok, stdout, _) = pitch(&["--help"]);
    assert!(ok);
    assert!(stdout.contains("USAGE:"));
}

#[test]
fn rejects_bad_input() {
    for args in [&["H2"][..], &["0"][..], &["A4", "B4"][..], &[][..]] {
        let (ok, _, stderr) = pitch(args);
        assert!(!ok, "expected failure for {args:?}");
        assert!(!stderr.is_empty());
    }
}

#[test]
fn rejects_out_of_range_a4() {
    let (ok, _, stderr) = pitch(&["--a4", "300", "A4"]);
    assert!(!ok);
    assert!(stderr.contains("--a4"), "got: {stderr}");
}
