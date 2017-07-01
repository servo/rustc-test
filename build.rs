extern crate rustc_version;

use rustc_version::version_meta;
use std::cmp::Ordering;

fn main() {
    if rustc_emits_allow_fail() {
        println!("cargo:rustc-cfg=rustc_emits_allow_fail");
    }
}

fn rustc_emits_allow_fail() -> bool {
    let version = version_meta().unwrap();
    match (version.semver.major, version.semver.minor).cmp(&(1, 20)) {
        Ordering::Less => false,
        Ordering::Greater => true,
        Ordering::Equal => match version.commit_date {
            None => true,  // Assume 1.20 Release
            Some(ref date) => &**date >= "2017-06-29",
        }
    }
}
