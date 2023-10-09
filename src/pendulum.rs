use nannou::{prelude::*, winit::event::Force};
use csv::Writer;

#[derive(Debug)]
pub struct SpringMass {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,

    pub stiffness: f32,
    pub damping: f32,

    pub bob_pos: Vec2,
    pub bob_mass: f32,
    pub length: f32,

    pub gravity: f32,
    // pub enitial_potential_energy: f32,
    pub ground: Vec2,
}

impl SpringMass {
    /// Updates the pendulum's position, velocity, and acceleration using the fourth-order Runge-Kutta method.
    ///
    /// # Arguments
    ///
    /// * `dt` - The time step to use for the update.
    ///
    /// # Returns
    ///
    /// None
    pub fn solver(&mut self, dt: f32) {
        self.acceleration = self.spring_mass(dt) + vec2(0.0, self.gravity);

        self.velocity += Self::rk4(dt, |x| x, self.acceleration);
        self.bob_pos += Self::rk4(dt, |x| x,self.velocity);

    }

    pub fn calculate_energy(&mut self, dt:f32) -> (f32, f32, f32){

        let kinetic_energy = 0.5 * self.bob_mass * ((self.velocity).length_squared());
        
        let spring_potential_energy = 0.5*self.stiffness*(self.bob_pos.distance(self.pos) - self.length).powi(2);
        let gravitational_potential_energy = self.bob_mass * self.gravity.abs() * (self.bob_pos.y - self.ground.y);
        let potential_energy = spring_potential_energy + gravitational_potential_energy;
        // println!("dispalcement: {}", (self.bob_pos.distance(self.pos) - self.length).powi(2));
        // println!("dispalcement from ground: {}", (self.bob_pos.y - self.ground.y));
        println!("PE: {}, KE: {}, TE: {}", potential_energy, kinetic_energy, potential_energy + kinetic_energy);
        (spring_potential_energy, gravitational_potential_energy, kinetic_energy)
    }

    fn rk4<F>(dt: f32, f: F, value: Vec2) -> Vec2
    where
        F: Fn(Vec2) -> Vec2,
    {
        let k1 = f(value);
        let k2 = f(value + dt * (k1 * 0.5));
        let k3 = f(value + dt * (k2 * 0.5));
        let k4 = f(value + k3 * dt);
        (k1 + 2.0 * k2 + 2.0 * k3 + k4) * (dt / 6.0)
    }
    
    pub fn spring_mass(&self, dt: f32) -> Vec2 {
        let distance = self.bob_pos.distance(self.pos) - self.length;
        let normal = (self.bob_pos - self.pos).normalize_or_zero();

        let force = -(distance * normal) * (self.stiffness/self.bob_mass) - ((self.damping/self.bob_mass) * self.velocity);
        force
    }

    pub fn collision(&mut self, size_function: &dyn Fn(f32) -> f32) {
        if self.bob_pos.y - (size_function(self.bob_mass)) <= self.ground.y {
            self.velocity.y *= -1.0;
        }
    }
}

impl Default for SpringMass {
    fn default() -> Self {
        let pos = Vec2::ZERO;
        let velocity = Vec2::ZERO;
        let acceleration = Vec2::ZERO;
        let bob_pos = vec2(0.0, -1.0);
        let bob_mass = 2.0;
        let length = 10.0;
        let stiffness = 6.0;
        let gravity = -1.0;
        let ground = vec2(0.0, -35.0);
        // let spring_potential_energy = 0.5*stiffness*(length).powi(2);
        // let bob_mass_potential_energy = bob_mass * gravity.abs() * length;
        // let enitial_potential_energy = spring_potential_energy + bob_mass_potential_energy;
        Self {
            pos,
            velocity,
            acceleration,
            bob_pos,
            bob_mass,
            length,
            stiffness,
            // enitial_potential_energy,
            gravity,
            ground,
            damping: 0.0,
        }
    }
}
