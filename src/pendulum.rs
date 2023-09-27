use nannou::{prelude::*, winit::event::Force};
use csv::Writer;

#[derive(Debug)]
pub struct SpringMass {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,

    pub stiffness: f32,

    pub bob_pos: Vec2,
    pub bob_mass: f32,
    pub length: f32,

    pub gravity: f32,
    // pub enitial_potential_energy: f32,
    pub ground: Vec2,
}

impl SpringMass {
    pub fn solver(&mut self, dt: f32) {
        // get the velocity using the direction that was moved
        // println!("acc: {}, vel: {}", self.acceleration, velocity);
        // self.acceleration = Vec2::ZERO;
        // self.acceleration = Self::rk4(dt, |x| x,self.spring_mass() / self.bob_mass + vec2(0.0, self.gravity));
        self.acceleration = Self::rk4(dt, |x| x,self.spring_mass() / self.bob_mass);
        // self.acceleration += Self::rk4(dt, |x| x, vec2(0.0, self.gravity) / self.bob_mass);

        self.velocity += Self::rk4(dt, |x| x, self.acceleration);
        self.bob_pos += Self::rk4(dt, |x| x,self.velocity);


        
    }

    pub fn calculate_energy(&mut self) -> (f32, f32){

        let kinetic_energy = 0.5 * self.bob_mass * ((self.velocity).length_squared()/0.1) ;
        let spring_potential_energy = 0.5*self.stiffness*(self.bob_pos.distance(self.pos) - self.length).powi(2);
        // println!("vel: {}, dispalcement: {}", self.velocity.length_squared(), (self.bob_pos.distance(self.pos) - self.length).powi(2));
        // let spring_potential_energy = 0.0;
        // let bob_mass_potential_energy = self.bob_mass * self.gravity.abs() * (self.bob_pos.y - self.ground.y);
        let bob_mass_potential_energy = 0.0;
        let potential_energy = spring_potential_energy + bob_mass_potential_energy;
        
        // if self.enitial_potential_energy < potential_energy {
        //     self.enitial_potential_energy = potential_energy;
        // }
        // let kinetic_energy = self.enitial_potential_energy - potential_energy;

        println!("PE: {}, KE: {}, TE: {}", potential_energy, kinetic_energy, potential_energy + kinetic_energy);
        (potential_energy, kinetic_energy)
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
    
    pub fn spring_mass(&self) -> Vec2 {
        let distance = self.bob_pos.distance(self.pos) - self.length;
        let normal = (self.bob_pos - self.pos).normalize_or_zero();

        let force = -(distance * normal) * self.stiffness;
        force
    }

    pub fn collision(&mut self) {
        if self.bob_pos.y - (self.bob_mass) <= self.ground.y {
            self.velocity.y *= -1.0;
        }
    }
}

impl Default for SpringMass {
    fn default() -> Self {
        let pos = Vec2::ZERO;
        let velocity = Vec2::ZERO;
        let acceleration = Vec2::ZERO;
        let bob_pos = vec2(0.01, 1.0);
        let bob_mass = 20.0;
        let length = 100.0;
        let stiffness = 6.0;
        let gravity = -100.0;
        let ground = vec2(0.0, -200.0);
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
            ground
        }
    }
}
