# Particle System

**Particle System** — A Rust library for Particle System.

## Why It Matters

This crate provides particle system functionality used across SuperInstance services. 

## How It Works

See the [source code](./src) for implementation details.

## Usage

```toml
[dependencies]
particle-system = "0.1.0"
```

```rust
use particle_system;

// See examples/ directory for detailed usage
```

## API

API documentation is generated from source doc-comments.

## Architecture

This crate is part of the **[SuperInstance](https://github.com/SuperInstance)** ecosystem — a conservation-law-based framework for fleet coordination, ternary computation, and distributed agent systems.

### Related Crates

- [`superinstance-core`](https://github.com/SuperInstance/superinstance-core) — Core conservation law (γ + η = C)
- [`superinstance-harness`](https://github.com/SuperInstance/superinstance-harness) — Build harness and self-improving loop
- [`fleet-coordinator`](https://github.com/SuperInstance/fleet-coordinator) — Fleet-level coordination

## References

- [SuperInstance Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)
- [Conservation Law Paper](https://github.com/SuperInstance/SuperInstance/blob/main/docs/conservation-law.md)

## License

MIT
