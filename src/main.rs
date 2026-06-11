/// Simple 2D particle system with gravity, drag, and bouncing
use std::f64::consts::PI;

#[derive(Clone)]
struct Particle {
    x: f64, y: f64,
    vx: f64, vy: f64,
    life: f64,
    radius: f64,
}

struct ParticleSystem {
    particles: Vec<Particle>,
    gravity: f64,
    drag: f64,
    dt: f64,
}

impl ParticleSystem {
    fn new(gravity: f64, drag: f64, dt: f64) -> Self {
        ParticleSystem { particles: Vec::new(), gravity, drag, dt }
    }

    fn emit(&mut self, count: usize, cx: f64, cy: f64) {
        for i in 0..count {
            let angle = 2.0 * PI * i as f64 / count as f64;
            let speed = 20.0 + (i as f64 % 5.0) * 4.0;
            self.particles.push(Particle {
                x: cx, y: cy,
                vx: speed * angle.cos(),
                vy: speed * angle.sin(),
                life: 3.0,
                radius: 0.5,
            });
        }
    }

    fn step(&mut self) {
        let dt = self.dt;
        let floor = 0.0;
        for p in &mut self.particles {
            p.vy -= self.gravity * dt;
            p.vx *= 1.0 - self.drag * dt;
            p.vy *= 1.0 - self.drag * dt;
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            if p.y < floor + p.radius {
                p.y = floor + p.radius;
                p.vy *= -0.6;
            }
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
    }

    fn count(&self) -> usize { self.particles.len() }
}

fn main() {
    let mut sys = ParticleSystem::new(9.81, 0.3, 0.05);
    sys.emit(20, 50.0, 80.0);

    for frame in 0..60 {
        sys.step();
        if frame % 10 == 0 {
            println!("Frame {frame:3}: {} particles alive", sys.count());
        }
    }

    // Second burst
    sys.emit(15, 30.0, 60.0);
    for frame in 60..120 {
        sys.step();
        if frame % 10 == 0 {
            println!("Frame {frame:3}: {} particles alive", sys.count());
        }
    }
    println!("Simulation complete. Final count: {}", sys.count());
}
