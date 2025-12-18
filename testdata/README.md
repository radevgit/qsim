# Test Data

IEEE standard test cases and simple test networks for validation.

## IEEE Test Cases

| File          | Buses | Generators | Branches | Source                |
|---------------|-------|------------|----------|-----------------------|
| `ieee9.json`  | 9     | 3          | 9        | MATPOWER case9.m      |
| `ieee14.json` | 14    | 5          | 20       | MATPOWER case14.m     |

## Simple Test Cases

| File        | Description                     |
|-------------|---------------------------------|
| `3bus.json` | Minimal network for unit tests  |

## Data Format

All files use JSON with the following structure:

```json
{
  "name": "Network name",
  "baseMVA": 100,
  "buses": [...],
  "generators": [...],
  "branches": [...]
}
```

## Sources

- MATPOWER: https://github.com/MATPOWER/matpower
- IEEE PES PSTCA: https://labs.ece.uw.edu/pstca/

## Usage in Tests

```rust
const IEEE14: &str = include_str!("../../testdata/ieee/ieee14.json");

#[test]
fn test_load_ieee14() {
    let network: NetworkData = serde_json::from_str(IEEE14).unwrap();
    assert_eq!(network.buses.len(), 14);
}
```
