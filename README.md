# Cognitive Archaeology

[![crates.io](https://img.shields.io/crates/v/cognitive-archaeology.svg)](https://crates.io/crates/cognitive-archaeology)
[![docs.rs](https://docs.rs/cognitive-archaeology/badge.svg)](https://docs.rs/cognitive-archaeology)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Layered cognitive history with archaeological excavation — dig through the strata of an agent's mind.**

---

## The Problem

Agent cognition evolves over time. An agent that started with basic reflexes later developed planning, then self-awareness. But when debugging or auditing an agent, there's no way to "dig down" through these layers to understand how a particular behavior originated. The history is either flat (all events in one list) or lost entirely.

## Why This Exists

Cognitive Archaeology models an agent's history as **geological strata** — layered deposits where the oldest layers are at the bottom and the most recent are on top. You can excavate through these layers to discover the origins of thoughts, analyze density patterns, and recover artifacts with surrounding context.

## Architecture

```
  ╔═══════════════════════════════════╗
  ║  Reflective (density: 0.9)       ║ ← TOP (most recent)
  ║  "Self-awareness, metacognition"  ║
  ╠═══════════════════════════════════╣
  ║  Deliberative (density: 0.7)     ║
  ║  "Planning, reasoning"           ║
  ╠═══════════════════════════════════╣
  ║  Reactive (density: 0.4)         ║
  ║  "Reflexive behavior"            ║
  ╠═══════════════════════════════════╣
  ║  Primitive (density: 0.2)        ║ ← BOTTOM (oldest)
  ║  "Basic perception"              ║
  ╚═══════════════════════════════════╝
  
  Excavation: dig_to(depth) → find_origin(predicate) → recover_artifact
  Stratigraphy: analyze density gradients, find densest layers
```

## Installation

```toml
[dependencies]
cognitive-archaeology = "0.1"
```

## API Reference

### `Stratum`

A cognitive layer with timestamp, label, data, and density:

```rust
use cognitive_archaeology::Stratum;

let s = Stratum::new("s1", 100.0, "primitive", "basic perception", 0.2);
assert_eq!(s.age(150.0), 50.0);
```

### `ArchaeologicalSite`

A stack of strata representing cognitive history:

```rust
use cognitive_archaeology::*;

let mut site = ArchaeologicalSite::new();
site.deposit(Stratum::new("s1", 100.0, "primitive", "basic perception", 0.2));
site.deposit(Stratum::new("s2", 200.0, "reactive", "reflexive behavior", 0.4));
site.deposit(Stratum::new("s3", 300.0, "deliberative", "planning layer", 0.7));
site.deposit(Stratum::new("s4", 400.0, "reflective", "self-awareness", 0.9));

assert_eq!(site.depth(), 4);
assert_eq!(site.bottom().unwrap().label, "primitive");
assert_eq!(site.top().unwrap().label, "reflective");
```

### `Excavation`

Dig through layers to find origins:

```rust
use cognitive_archaeology::*;

let site = /* ... build site ... */;
let excavation = Excavation::new(&site);

// Dig to specific depth
let layer = excavation.dig_to(0); // oldest layer

// Find origin matching a predicate
let dense = excavation.find_origin(|s| s.density > 0.5);

// Excavate by label pattern
let results = excavation.excavate_by_label("tive"); // primitive, reactive, etc.

// Recover artifact with surrounding context
let artifact = excavation.recover_artifact(1).unwrap();
// artifact has surrounding_context from adjacent layers
```

### `Stratigraphy`

Analysis of layer composition:

```rust
use cognitive_archaeology::*;

let report = Stratigraphy::analyze(&site);
println!("{} layers, avg density {:.2}, span {:.0}s",
    report.total_layers, report.avg_density, report.time_span);

let gradient = Stratigraphy::density_gradient(&site);
let densest = Stratigraphy::densest_layer(&site);
```

## Usage Examples

### Example 1: Full Cognitive Excavation

```rust
use cognitive_archaeology::*;

let mut site = ArchaeologicalSite::new();
site.deposit(Stratum::new("s1", 100.0, "primitive", "basic perception", 0.2));
site.deposit(Stratum::new("s2", 200.0, "reactive", "reflexive behavior", 0.4));
site.deposit(Stratum::new("s3", 300.0, "deliberative", "planning layer", 0.7));
site.deposit(Stratum::new("s4", 400.0, "reflective", "self-awareness", 0.9));

let excavation = Excavation::new(&site);
for layer in excavation.full() {
    println!("[{}] {} (density: {:.1}): {}",
        layer.timestamp, layer.label, layer.density, layer.data);
}
```

### Example 2: Artifact Recovery with Context

```rust
use cognitive_archaeology::*;

// Build site...
let excavation = Excavation::new(&site);
let artifact = excavation.recover_artifact(2).unwrap();
// artifact includes data from layers above and below
println!("Found '{}' at depth {}", artifact.content, artifact.depth);
println!("Context: {:?}", artifact.surrounding_context);
```

### Example 3: Stratigraphy Analysis

```rust
use cognitive_archaeology::*;

let report = Stratigraphy::analyze(&site);
let gradient = Stratigraphy::density_gradient(&site);
let densest = Stratigraphy::densest_layer(&site);

println!("Cognitive evolution:");
println!("  Layers: {}", report.total_layers);
println!("  Time span: {:.0}s", report.time_span);
println!("  Avg density: {:.2}", report.avg_density);
println!("  Densest: {} ({:.1})",
    densest.unwrap().label, densest.unwrap().density);
```

## Performance

| Operation | Complexity |
|-----------|-----------|
| Deposit stratum | O(1) |
| Dig to depth | O(1) |
| Find origin | O(n) |
| Recover artifact | O(1) |
| Stratigraphy analysis | O(n) |
| Density gradient | O(n) |

## License

Licensed under the [MIT License](LICENSE).

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests
4. Push and open a Pull Request
