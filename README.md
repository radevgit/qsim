# qsim

A Rust library for power grid modeling and simulation.

## Features

- **Power Flow Analysis** — DC and AC power flow solvers
- **Modular Architecture** — Pluggable solvers and elements
- **High Performance** — Pure Rust, parallel computation with Rayon
- **Graph-Based Topology** — Flexible network representation

## Crates

| Crate           | Description                              |
|-----------------|------------------------------------------|
| `qsim-core`     | Core engine, traits, state management    |
| `qsim-elements` | Grid elements (Bus, Branch, Generator)   |
| `qsim-solvers`  | Power flow solvers (DC, AC)              |
| `qsim-io`       | JSON I/O, serialization, persistence     |
| `qsim`          | Umbrella crate (re-exports all)          |

## Quick Start

```rust
use qsim::prelude::*;

// Create a simple 3-bus network
let mut network = Network::new();

// Add buses
let bus1 = network.add_bus(Bus::slack(1.0));
let bus2 = network.add_bus(Bus::pv(1.0, 50.0));
let bus3 = network.add_bus(Bus::pq(-100.0, -30.0));

// Add branches
network.add_branch(Branch::line(bus1, bus2, 0.01, 0.1));
network.add_branch(Branch::line(bus2, bus3, 0.02, 0.15));

// Solve power flow
let solver = DcPowerFlowSolver::new();
let result = solver.solve(&network)?;

println!("Bus voltages: {:?}", result.voltages());
```

## Installation

```toml
[dependencies]
qsim = "0.1"
```

## License

MIT
