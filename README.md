# lld-rust

Low-Level Design problems solved in Rust.

## Structure

Each problem lives under `problems/` as its own Cargo crate inside a shared workspace.

```
problems/
└── <problem-name>/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Running a problem

```bash
cargo run -p <problem-name>
```

## Problems

| Problem | Description |
|---------|-------------|
| [delivery-cost-tracking-system](problems/delivery-cost-tracking-system) | Track delivery costs per driver with payment settlement |
| [lru-cache](problems/lru-cache) | LRU cache with O(1) get/put using doubly linked list + hashmap |
| [parking-lot](problems/parking-lot) | Parking lot with slot allocation strategy, ticket issuance, hourly pricing, and pluggable payment |
