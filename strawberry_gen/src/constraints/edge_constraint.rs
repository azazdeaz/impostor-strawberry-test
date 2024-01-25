use bevy::prelude::*;

use crate::ParticlePosition;

#[derive(Component, Debug, Reflect)]
pub struct EdgeConstraint {
    pub rest_length: f32,
    pub compliance: f32,
}
impl EdgeConstraint {
    pub fn from_particles(a: &ParticlePosition, b: &ParticlePosition) -> Self {
        Self {
            rest_length: (*a).distance(**b),
            compliance: 0.1,
        }
    }
    pub fn from_rest_length(rest_length: f32) -> Self {
        Self {
            rest_length,
            compliance: 0.1,
        }
    }
}

impl EdgeConstraint {
    pub fn get_compliance(&self) -> f32 {
        self.compliance
    }
    pub fn compute_stress(&self, a: &ParticlePosition, b: &ParticlePosition) -> f32 {
        let distance = (*a).distance(**b);
        let strain = (distance - self.rest_length) / self.rest_length;
        strain * self.get_compliance()
    }
    pub fn distance_with_stress(&self, target_stress: f32) -> f32 {
        self.rest_length * (1.0 + target_stress / self.get_compliance())
    }
    pub fn solve(&self, a: &mut ParticlePosition, b: &mut ParticlePosition, target_stress: f32) {
        let distance = (*a).distance(**b);
        if distance == 0.0 {
            return;
        }
        let target_distance = self.distance_with_stress(target_stress);
        let difference = target_distance - distance;
        let direction = (**b - **a) / distance;
        a.0 -= direction * difference * 0.5;
        b.0 += direction * difference * 0.5;   
    }
}