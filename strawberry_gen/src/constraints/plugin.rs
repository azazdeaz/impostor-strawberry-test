use aery::prelude::*;
use bevy::{prelude::*, utils::HashSet};

use crate::{EdgeConstraint, ParticlePosition};

pub struct ConstrainsPlugin;
impl Plugin for ConstrainsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EdgeConstraint>()
            .add_systems(Update, relax_constraints);
    }
}

#[derive(Relation)]
#[aery(Symmetric, Poly)]
pub struct ConstraintToConstraint;

#[derive(Relation)]
pub struct P0;

#[derive(Relation)]
pub struct P1;

fn relax_constraints(
    constraints: Query<(
        (Entity, &EdgeConstraint),
        Relations<(P0, P1, ConstraintToConstraint)>,
    )>,
    mut particles: Query<&mut ParticlePosition>,
    particle_entities: Query<Entity, With<ParticlePosition>>,
) {
    println!("Relaxing constraints");
    let mut max_stress_constraint = (Entity::PLACEHOLDER, 0.0);
    for (constraint, edges) in &constraints {
        println!("Constraint: {:?}", constraint);
        edges
            .join::<Up<P0>>(&particles)
            .join::<Up<P1>>(&particles)
            .for_each(|(a, b)| {
                let stress = constraint.1.compute_stress(a, b);
                println!("Stress ({:?}->{:?}): {}", a, b, stress);
                if max_stress_constraint.0 == Entity::PLACEHOLDER
                    || stress > max_stress_constraint.1
                {
                    max_stress_constraint = (constraint.0, stress);
                }
            });
    }
    if max_stress_constraint.0 == Entity::PLACEHOLDER {
        return;
    }
    // Traverse the graph and relax the constraints strating from the most stressed one
    let mut solved_particles = HashSet::<Entity>::new();
    let mut solved_constrains = HashSet::<Entity>::new();

    {
        // TODO: move this into a function or something
        let ((constraint_id, constraint), edges) =
            constraints.get(max_stress_constraint.0).unwrap();
        edges
            .join::<Up<P0>>(&particle_entities)
            .join::<Up<P1>>(&particle_entities)
            .for_each(|(a, b)| {
                solved_particles.insert(a);
                solved_particles.insert(b);
                let [mut a, mut b] = particles.get_many_mut([a, b]).unwrap();
                println!("Solving constraint {:?}", constraint);
                constraint.solve(&mut a, &mut b, 0.0);
            });
        solved_constrains.insert(constraint_id);
    }
    constraints
        .traverse::<ConstraintToConstraint>([max_stress_constraint.0])
        .for_each(|(constraint_id, constraint), edges| {
            if solved_constrains.contains(constraint_id) {
                println!("Constraint {:?} already solved", constraint_id);
                TCF::Close
            } else {
                println!("Solving constraint {:?}", constraint_id);
                edges
                    .join::<Up<P0>>(&particle_entities)
                    .join::<Up<P1>>(&particle_entities)
                    .for_each(|(a, b)| {
                        // TODO lock solved particles
                        solved_particles.insert(a);
                        solved_particles.insert(b);
                        let [mut a, mut b] = particles.get_many_mut([a, b]).unwrap();
                        println!("Solving constraint {:?}", constraint);
                        constraint.solve(&mut a, &mut b, 0.0);
                    });
                solved_constrains.insert(*constraint_id);
                TCF::Continue
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn test_edge_constraint() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, Aery, ConstrainsPlugin));

        let a = app
            .world
            .spawn(ParticlePosition((0.0, 0.0, 0.0).into()))
            .id();
        let b = app
            .world
            .spawn(ParticlePosition((0.0, 1.0, 0.0).into()))
            .id();
        let c = app
            .world
            .spawn(ParticlePosition((0.0, 2.3, 0.0).into()))
            .id();
        let constraint1 = app
            .world
            .spawn(EdgeConstraint::from_rest_length(1.0))
            .set::<P0>(a)
            .set::<P1>(b)
            .id();
        app.world
            .spawn(EdgeConstraint::from_rest_length(1.0))
            .set::<P0>(b)
            .set::<P1>(c)
            .set::<ConstraintToConstraint>(constraint1);
        app.world.run_system_once(relax_constraints);
        // print all particles
        app.world
            .run_system_once(|particles: Query<&ParticlePosition>| {
                particles.for_each(|p| println!("{:?}", p));
            });
        app.world.run_system_once(relax_constraints);
        // print all particles
        app.world
            .run_system_once(|particles: Query<&ParticlePosition>| {
                particles.for_each(|p| println!("{:?}", p));
            });
        assert!(1 == 2);
    }
}
