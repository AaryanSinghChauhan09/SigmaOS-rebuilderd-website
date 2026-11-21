//! Integration tests for sigmaos-rebuilderd-website.

use rebuilderd_website::{BuildStatus, BuilderStatus, fleet_stats, package_stats, RebuilderdData};
use rebuilderd_website::model::{Builder, PackageBuild};

fn sample() -> RebuilderdData {
    RebuilderdData {
        builders: vec![Builder { name: "b1".into(), host_arch: "x86_64".into(), packages: 50, status: "online".into() }],
        packages: vec![
            PackageBuild { pkgname: "linux".into(), pkgver: "6.1-1".into(), builder: "b1".into(), status: "good".into() },
            PackageBuild { pkgname: "firefox".into(), pkgver: "120-1".into(), builder: "b1".into(), status: "bad".into() },
        ],
    }
}

#[test]
fn test_status_normalization() {
    let b = Builder { name: "x".into(), host_arch: String::new(), packages: 0, status: "online".into() };
    assert_eq!(b.status(), BuilderStatus::Online);
    let p = PackageBuild { pkgname: "x".into(), pkgver: "1".into(), builder: "b".into(), status: "good".into() };
    assert_eq!(p.status(), BuildStatus::Good);
}

#[test]
fn test_stats_and_sections() {
    let d = sample();
    let s = fleet_stats(&d);
    assert_eq!(s.builders_online, 1);
    assert_eq!(s.packages_good, 1);
    assert_eq!(s.packages_bad, 1);
    assert!(s.healthy());
    let stats = package_stats(&d);
    assert_eq!(stats.len(), 2);
    let linux = stats.iter().find(|p| p.pkgname == "linux").unwrap();
    assert_eq!(linux.status, BuildStatus::Good);
}
