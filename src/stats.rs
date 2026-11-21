//! Statistics aggregation — what the website's sections display.

use std::collections::BTreeMap;

use crate::model::{BuildStatus, BuilderStatus, RebuilderdData};

/// Fleet-wide statistics.
#[derive(Debug, Default)]
pub struct FleetStats {
    /// Total builders.
    pub builders_total: usize,
    /// Online builders.
    pub builders_online: usize,
    /// Total package build results.
    pub packages_total: usize,
    /// Reproducible (good) results.
    pub packages_good: usize,
    /// Failed/different results.
    pub packages_bad: usize,
    /// Reproducibility percentage (0-100).
    pub reproducible_pct: f64,
}

impl FleetStats {
    /// Whether the fleet is healthy (all builders online, >0 good builds).
    pub fn healthy(&self) -> bool {
        self.builders_online == self.builders_total
            && self.builders_total > 0
            && self.packages_good > 0
    }
}

/// Compute fleet stats from rebuilderd data.
pub fn fleet_stats(data: &RebuilderdData) -> FleetStats {
    let builders_online = data
        .builders
        .iter()
        .filter(|b| b.status() == BuilderStatus::Online)
        .count();
    let packages_good = data
        .packages
        .iter()
        .filter(|p| p.status() == BuildStatus::Good)
        .count();
    let packages_bad = data
        .packages
        .iter()
        .filter(|p| p.status() == BuildStatus::Bad)
        .count();
    let total = data.packages.len();
    FleetStats {
        builders_total: data.builders.len(),
        builders_online,
        packages_total: total,
        packages_good,
        packages_bad,
        reproducible_pct: if total > 0 {
            packages_good as f64 / total as f64 * 100.0
        } else {
            0.0
        },
    }
}

/// Per-package stats: latest status across builders.
#[derive(Debug, Clone)]
pub struct PackageStat {
    /// Package name.
    pub pkgname: String,
    /// Latest version seen.
    pub version: String,
    /// Best status across builders.
    pub status: BuildStatus,
    /// Builders that built it.
    pub builders: usize,
}

/// Aggregate per-package stats.
pub fn package_stats(data: &RebuilderdData) -> Vec<PackageStat> {
    let mut map: BTreeMap<String, (String, BuildStatus, usize)> = BTreeMap::new();
    for p in &data.packages {
        let e = map
            .entry(p.pkgname.clone())
            .or_insert((p.pkgver.clone(), BuildStatus::Unknown, 0));
        e.0 = p.pkgver.clone(); // last seen wins
        e.1 = match (e.1, p.status()) {
            (BuildStatus::Good, _) | (_, BuildStatus::Good) => BuildStatus::Good,
            (BuildStatus::Bad, _) | (_, BuildStatus::Bad) => BuildStatus::Bad,
            _ => BuildStatus::Unknown,
        };
        e.2 += 1;
    }
    map.into_iter()
        .map(|(pkgname, (version, status, builders))| PackageStat {
            pkgname,
            version,
            status,
            builders,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Builder, PackageBuild};

    fn data() -> RebuilderdData {
        RebuilderdData {
            builders: vec![
                Builder { name: "b1".into(), host_arch: "x86_64".into(), packages: 100, status: "online".into() },
                Builder { name: "b2".into(), host_arch: "x86_64".into(), packages: 100, status: "offline".into() },
            ],
            packages: vec![
                PackageBuild { pkgname: "a".into(), pkgver: "1".into(), builder: "b1".into(), status: "good".into() },
                PackageBuild { pkgname: "a".into(), pkgver: "1".into(), builder: "b2".into(), status: "bad".into() },
                PackageBuild { pkgname: "b".into(), pkgver: "2".into(), builder: "b1".into(), status: "good".into() },
            ],
        }
    }

    #[test]
    fn test_fleet_stats() {
        let s = fleet_stats(&data());
        assert_eq!(s.builders_total, 2);
        assert_eq!(s.builders_online, 1);
        assert_eq!(s.packages_good, 2);
        assert_eq!(s.packages_bad, 1);
        assert!((s.reproducible_pct - 66.67).abs() < 0.1);
    }

    #[test]
    fn test_package_stats_best_status() {
        let stats = package_stats(&data());
        let a = stats.iter().find(|p| p.pkgname == "a").unwrap();
        assert_eq!(a.status, BuildStatus::Good); // good beats bad
        assert_eq!(a.builders, 2);
    }

    #[test]
    fn test_unhealthy_when_builder_down() {
        let s = fleet_stats(&data());
        assert!(!s.healthy()); // b2 offline
    }
}
