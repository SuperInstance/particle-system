# Particle System

**particle-system** is a Rust library for 2D particle simulations with Newtonian physics. It models particles under gravity, air drag, and elastic collision with surfaces, providing the computational foundation for visual effects, game engines, and physics demonstrations.

## Why It Matters

Particle systems are everywhere: explosion effects in games, smoke simulations in film VFX, rain and snow in weather rendering, and spray patterns in fluid dynamics. Every modern game engine (Unity, Unreal, Godot) ships a particle system as a core component. This implementation provides the physics integration layer — the part that decides where each particle moves next based on forces — which is the mathematical heart of any particle simulation.

## How It Works

### Newtonian Integration

Each particle has position (x, y), velocity (vx, vy), and life remaining. Per time step Δt, the system applies forces using **semi-implicit (symplectic) Euler** integration:

```
1. Apply gravity:     vy += -g · Δt
2. Apply drag:        vx *= (1 - drag · Δt)
                       vy *= (1 - drag · Δt)
3. Update position:   x += vx · Δt
                       y += vy · Δt
4. Check collisions:  if y < floor + radius:
                       y = floor + radius
                       vy *= -restitution  (bounce)
5. Age:               life -= Δt
6. Cull:              remove if life ≤ 0
```

### Force Model

**Gravity** accelerates particles downward at *g = 9.81 m/s²* (configurable). This is the same constant used in Earth-surface physics.

**Drag** models air resistance as an exponential decay of velocity per time step. The drag coefficient (0.0–1.0) controls how quickly particles slow down:
- `drag = 0.0`: vacuum (no air resistance, particles travel forever)
- `drag = 0.3`: light air resistance (default, realistic for small particles)
- `drag = 1.0`: thick medium (particles stop almost immediately)

**Restitution** (bounce coefficient) controls energy retention on collision:
- `0.0`: perfectly inelastic (stops dead)
- `0.6`: default (loses 64% of kinetic energy per bounce)
- `1.0`: perfectly elastic (bounces forever)

### Emission

`emit(count, cx, cy)` creates `count` particles distributed radially from center (cx, cy) with equal angular spacing:

```
angle_i = 2π · i / count    for i = 0..count
speed_i  = 20 + (i mod 5) · 4
vx = speed · cos(angle)
vy = speed · sin(angle)
```

This creates a circular burst pattern. Unequal speeds add visual variation.

### Complexity

| Operation | Time |
|-----------|------|
| `emit(n)` | O(n) |
| `step()` | O(N) where N = active particles |
| `count()` | O(1) |

Each `step()` iteration is O(N): every particle gets one force update, one position update, one collision check, and one life decrement. Dead particles are removed via `Vec::retain`, which is O(N).

## Quick Start

```rust
// Binary crate. Run with: cargo run

use particle_system::ParticleSystem;

fn main() {
    let mut sys = ParticleSystem::new(9.81, 0.3, 0.05);

    // Emit a burst of 20 particles from (50, 80)
    sys.emit(20, 50.0, 80.0);

    // Simulate 120 frames (6 seconds at Δt=0.05)
    for frame in 0..120 {
        sys.step();
        if frame % 10 == 0 {
            println!("Frame {:3}: {} particles alive", frame, sys.count());
        }
    }

    // Second burst from a different location
    sys.emit(15, 30.0, 60.0);
    for frame in 120..200 {
        sys.step();
    }
    println!("Final count: {}", sys.count());
}
```

## API

### `ParticleSystem`

| Method | Description |
|--------|-------------|
| `new(gravity: f64, drag: f64, dt: f64)` | Create with physics parameters |
| `emit(count: usize, cx: f64, cy: f64)` | Spawn a radial burst |
| `step()` | Advance one time step (applies forces, moves, bounces, culls) |
| `count() → usize` | Active particle count |

### `Particle` (internal)

| Field | Description |
|-------|-------------|
| `x, y` | Position in 2D space |
| `vx, vy` | Velocity components |
| `life` | Remaining lifetime in seconds |
| `radius` | Collision radius |

## Architecture Notes

This crate provides simulation capabilities for SuperInstance's physics-based modeling. Particle systems simulate agent diffusion in distributed systems: each particle represents a request propagating through the network, and the physics parameters model network latency and load. This visualizes the γ + η = C framework's behavior under stress.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for the full design.

## References

- Reeves, W. T. (1983). *Particle Systems — A Technique for Modeling a Class of Fuzzy Objects*. ACM Transactions on Graphics, 2(2), 91–108.
- Hairer, E., & Wanner, G. (1996). *Solving Ordinary Differential Equations I: Nonstiff Problems* (2nd ed.). Springer. §VI.1 on symplectic methods.
- Bridson, R. (2008). *Fluid Simulation for Computer Graphics*. A K Peters.

## License

MIT
