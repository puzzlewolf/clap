mod utils;

use std::str;

use clap::{App, AppSettings, ErrorKind};

static VERSION: &str = "clap-test v1.4.8\n";

#[test]
fn version_short() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .try_get_matches_from(vec!["myprog", "-V"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");
}

#[test]
fn version_long() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .try_get_matches_from(vec!["myprog", "--version"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");
}

#[test]
fn complex_version_output() {
    let mut a = App::new("clap-test").version("v1.4.8");
    let _ = a.try_get_matches_from_mut(vec![""]);

    // Now we check the output of print_version()
    assert_eq!(a.render_version(), VERSION);
}

#[test]
fn override_version_short() {
    let app = App::new("test")
        .version("1.3")
        .mut_arg("version", |a| a.short('v'));

    let m = app.clone().try_get_matches_from(vec!["test", "-v"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");

    let m = app.try_get_matches_from(vec!["test", "--version"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");
}

#[test]
fn override_version_long() {
    let app = App::new("test")
        .version("1.3")
        .mut_arg("version", |a| a.long("vers"));

    let m = app.clone().try_get_matches_from(vec!["test", "-V"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");

    let m = app.try_get_matches_from(vec!["test", "--vers"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");
}

static OVERRIDE_VERSION_ABOUT: &str = "test 1.3

USAGE:
    test

FLAGS:
    -H, --help       Prints help information
    -v, --version    version info";

#[test]
fn override_version_about() {
    let app = App::new("test")
        .version("1.3")
        .mut_arg("version", |a| a.about("version info"));

    assert!(utils::compare_output(
        app,
        "test -h",
        OVERRIDE_VERSION_ABOUT,
        false
    ));
}
