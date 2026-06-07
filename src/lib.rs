//! # cognitive-archaeology
//!
//! Layered cognitive history with archaeological excavation.
//!
//! Model cognitive history as geological strata: the oldest layers are at the
//! bottom, and you excavate downward to discover the origins of thoughts.

/// A cognitive layer (stratum) with a timestamp.
#[derive(Debug, Clone, PartialEq)]
pub struct Stratum {
    pub id: String,
    pub timestamp: f64,
    pub label: String,
    pub data: String,
    pub density: f64,
}

impl Stratum {
    pub fn new(
        id: impl Into<String>,
        timestamp: f64,
        label: impl Into<String>,
        data: impl Into<String>,
        density: f64,
    ) -> Self {
        Stratum {
            id: id.into(),
            timestamp,
            label: label.into(),
            data: data.into(),
            density: density.clamp(0.0, 1.0),
        }
    }

    /// Age of this stratum relative to "now".
    pub fn age(&self, now: f64) -> f64 {
        (now - self.timestamp).max(0.0)
    }
}

/// A stack of strata representing cognitive history (oldest at bottom / index 0).
#[derive(Debug, Clone)]
pub struct ArchaeologicalSite {
    strata: Vec<Stratum>,
}

impl ArchaeologicalSite {
    pub fn new() -> Self {
        ArchaeologicalSite { strata: Vec::new() }
    }

    /// Deposit a new stratum on top (most recent).
    pub fn deposit(&mut self, stratum: Stratum) {
        self.strata.push(stratum);
    }

    /// Number of strata.
    pub fn depth(&self) -> usize {
        self.strata.len()
    }

    /// Get the top (most recent) stratum.
    pub fn top(&self) -> Option<&Stratum> {
        self.strata.last()
    }

    /// Get the bottom (oldest) stratum.
    pub fn bottom(&self) -> Option<&Stratum> {
        self.strata.first()
    }

    /// Get all strata.
    pub fn strata(&self) -> &[Stratum] {
        &self.strata
    }

    /// Get a stratum at a specific depth (0 = bottom/oldest).
    pub fn at_depth(&self, depth: usize) -> Option<&Stratum> {
        self.strata.get(depth)
    }

    /// Check if site is empty.
    pub fn is_empty(&self) -> bool {
        self.strata.is_empty()
    }
}

impl Default for ArchaeologicalSite {
    fn default() -> Self {
        Self::new()
    }
}

/// A recovered cognitive artifact with its excavation context.
#[derive(Debug, Clone, PartialEq)]
pub struct Artifact {
    pub stratum_id: String,
    pub content: String,
    pub depth: usize,
    pub surrounding_context: Vec<String>,
}

impl Artifact {
    pub fn new(
        stratum_id: impl Into<String>,
        content: impl Into<String>,
        depth: usize,
        surrounding_context: Vec<String>,
    ) -> Self {
        Artifact {
            stratum_id: stratum_id.into(),
            content: content.into(),
            depth,
            surrounding_context,
        }
    }
}

/// An excavation through the layers to find origins.
#[derive(Debug)]
pub struct Excavation<'a> {
    site: &'a ArchaeologicalSite,
}

impl<'a> Excavation<'a> {
    pub fn new(site: &'a ArchaeologicalSite) -> Self {
        Excavation { site }
    }

    /// Excavate to a specific depth and return the stratum.
    pub fn dig_to(&self, depth: usize) -> Option<&Stratum> {
        self.site.at_depth(depth)
    }

    /// Find the earliest stratum matching a predicate.
    pub fn find_origin<F>(&self, predicate: F) -> Option<&Stratum>
    where
        F: Fn(&Stratum) -> bool,
    {
        // Search from bottom (oldest) to top
        self.site.strata().iter().find(|s| predicate(s))
    }

    /// Excavate all strata matching a label pattern.
    pub fn excavate_by_label(&self, label: &str) -> Vec<&Stratum> {
        self.site
            .strata()
            .iter()
            .filter(|s| s.label.contains(label))
            .collect()
    }

    /// Recover an artifact from a stratum with surrounding context.
    pub fn recover_artifact(&self, depth: usize) -> Option<Artifact> {
        let stratum = self.site.at_depth(depth)?;
        let mut context = Vec::new();
        // Get surrounding strata
        if depth > 0 {
            if let Some(s) = self.site.at_depth(depth - 1) {
                context.push(s.data.clone());
            }
        }
        if let Some(s) = self.site.at_depth(depth + 1) {
            context.push(s.data.clone());
        }
        Some(Artifact::new(&stratum.id, &stratum.data, depth, context))
    }

    /// Full excavation: return all strata from bottom to top.
    pub fn full(&self) -> Vec<&Stratum> {
        self.site.strata().iter().collect()
    }
}

/// Analysis of layer composition.
#[derive(Debug, Clone)]
pub struct StratigraphyReport {
    pub total_layers: usize,
    pub avg_density: f64,
    pub time_span: f64,
    pub labels: Vec<String>,
}

/// Analyze layer composition, density, and age.
pub struct Stratigraphy;

impl Stratigraphy {
    /// Generate a stratigraphy report for a site.
    pub fn analyze(site: &ArchaeologicalSite) -> StratigraphyReport {
        if site.is_empty() {
            return StratigraphyReport {
                total_layers: 0,
                avg_density: 0.0,
                time_span: 0.0,
                labels: Vec::new(),
            };
        }
        let avg_density = site.strata().iter().map(|s| s.density).sum::<f64>() / site.depth() as f64;
        let earliest = site.bottom().unwrap().timestamp;
        let latest = site.top().unwrap().timestamp;
        let labels = site.strata().iter().map(|s| s.label.clone()).collect();
        StratigraphyReport {
            total_layers: site.depth(),
            avg_density,
            time_span: latest - earliest,
            labels,
        }
    }

    /// Compute the density gradient (how density changes from bottom to top).
    pub fn density_gradient(site: &ArchaeologicalSite) -> Vec<f64> {
        site.strata()
            .windows(2)
            .map(|w| w[1].density - w[0].density)
            .collect()
    }

    /// Find the densest layer.
    pub fn densest_layer(site: &ArchaeologicalSite) -> Option<&Stratum> {
        site.strata().iter().max_by(|a, b| a.density.partial_cmp(&b.density).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_site() -> ArchaeologicalSite {
        let mut site = ArchaeologicalSite::new();
        site.deposit(Stratum::new("s1", 100.0, "primitive", "basic perception", 0.2));
        site.deposit(Stratum::new("s2", 200.0, "reactive", "reflexive behavior", 0.4));
        site.deposit(Stratum::new("s3", 300.0, "deliberative", "planning layer", 0.7));
        site.deposit(Stratum::new("s4", 400.0, "reflective", "self-awareness", 0.9));
        site
    }

    #[test]
    fn test_site_depth() {
        let site = make_site();
        assert_eq!(site.depth(), 4);
    }

    #[test]
    fn test_site_top_bottom() {
        let site = make_site();
        assert_eq!(site.bottom().unwrap().id, "s1");
        assert_eq!(site.top().unwrap().id, "s4");
    }

    #[test]
    fn test_site_at_depth() {
        let site = make_site();
        assert_eq!(site.at_depth(0).unwrap().label, "primitive");
        assert_eq!(site.at_depth(3).unwrap().label, "reflective");
        assert!(site.at_depth(10).is_none());
    }

    #[test]
    fn test_empty_site() {
        let site = ArchaeologicalSite::new();
        assert!(site.is_empty());
        assert!(site.top().is_none());
    }

    #[test]
    fn test_stratum_age() {
        let s = Stratum::new("s", 100.0, "test", "data", 0.5);
        assert_eq!(s.age(150.0), 50.0);
        assert_eq!(s.age(50.0), 0.0);
    }

    #[test]
    fn test_excavation_dig_to() {
        let site = make_site();
        let excavation = Excavation::new(&site);
        assert_eq!(excavation.dig_to(0).unwrap().id, "s1");
        assert_eq!(excavation.dig_to(2).unwrap().id, "s3");
    }

    #[test]
    fn test_excavation_find_origin() {
        let site = make_site();
        let excavation = Excavation::new(&site);
        let found = excavation.find_origin(|s| s.density > 0.5);
        assert_eq!(found.unwrap().id, "s3");
    }

    #[test]
    fn test_excavation_by_label() {
        let site = make_site();
        let excavation = Excavation::new(&site);
        let results = excavation.excavate_by_label("tive");
        assert_eq!(results.len(), 4); // primitive, reactive, deliberative, reflective
    }

    #[test]
    fn test_recover_artifact() {
        let site = make_site();
        let excavation = Excavation::new(&site);
        let artifact = excavation.recover_artifact(1).unwrap();
        assert_eq!(artifact.depth, 1);
        assert_eq!(artifact.stratum_id, "s2");
        assert!(!artifact.surrounding_context.is_empty());
    }

    #[test]
    fn test_recover_artifact_boundary() {
        let site = make_site();
        let excavation = Excavation::new(&site);
        let bottom = excavation.recover_artifact(0).unwrap();
        assert_eq!(bottom.depth, 0);
        // Bottom should only have context from above
        assert_eq!(bottom.surrounding_context.len(), 1);
        let top = excavation.recover_artifact(3).unwrap();
        assert_eq!(top.surrounding_context.len(), 1); // only from below
    }

    #[test]
    fn test_recover_artifact_out_of_range() {
        let site = make_site();
        let excavation = Excavation::new(&site);
        assert!(excavation.recover_artifact(10).is_none());
    }

    #[test]
    fn test_stratigraphy_analyze() {
        let site = make_site();
        let report = Stratigraphy::analyze(&site);
        assert_eq!(report.total_layers, 4);
        assert!((report.avg_density - 0.55).abs() < 1e-9);
        assert_eq!(report.time_span, 300.0);
        assert_eq!(report.labels.len(), 4);
    }

    #[test]
    fn test_stratigraphy_empty() {
        let site = ArchaeologicalSite::new();
        let report = Stratigraphy::analyze(&site);
        assert_eq!(report.total_layers, 0);
    }

    #[test]
    fn test_density_gradient() {
        let site = make_site();
        let gradient = Stratigraphy::density_gradient(&site);
        assert_eq!(gradient.len(), 3);
        // All gradients should be positive (density increases from bottom to top)
        for g in &gradient {
            assert!(g > &0.0);
        }
    }

    #[test]
    fn test_densest_layer() {
        let site = make_site();
        let densest = Stratigraphy::densest_layer(&site);
        assert_eq!(densest.unwrap().id, "s4");
    }

    #[test]
    fn test_full_excavation() {
        let site = make_site();
        let excavation = Excavation::new(&site);
        let all = excavation.full();
        assert_eq!(all.len(), 4);
    }
}
