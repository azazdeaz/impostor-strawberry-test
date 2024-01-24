use std::ops::Deref;

use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

#[derive(Component, Debug, Default)]
pub struct ParticlePosition(pub Vec3);
impl Deref for ParticlePosition {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pub position: ParticlePosition,
    pub transform: Transform,
}
impl ParticleBundle {
    pub fn new(position: Vec3) -> Self {
        Self {
            position: ParticlePosition(position),
            transform: Transform::from_translation(position),
        }
    }
}

struct DragInfo {
    id: Entity,
    grab_distance: f32,
}

#[derive(Default)]
struct DragParticleState {
    info: Option<DragInfo>,
}



fn drag_particles(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut particles: Query<(Entity, &mut ParticlePosition)>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
    mut drag_state: Local<DragParticleState>,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
) {
    let Some(cursor_position) = windows.single().cursor_position() else { return; };
    let (camera, camera_transform) = camera_query.single();
    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return; };

    if buttons.just_pressed(MouseButton::Left) {
        info!("Clicked");
        let mut closest: Option<(Entity, f32)> = None;
        for (id, particle) in &particles {
                // calculate the particle distance from the ray
                let particle_from_origin = **particle - ray.origin;
                let closest_point_on_ray = ray.direction * particle_from_origin.dot(ray.direction);
                let distance = (particle_from_origin - closest_point_on_ray).length();
                info!("Distance: {}", distance);
                if distance < 0.1 && (closest.is_none() || distance < closest.unwrap().1) {
                    closest = Some((id, distance));
                }
            
        }
        if let Some((id, _)) = closest {
            drag_state.info = Some(DragInfo {
                id,
                grab_distance: particles.get(id).unwrap().1.distance(ray.origin),
            });
        }
    } else if buttons.just_released(MouseButton::Left) {
        drag_state.info = None;
    }

    for mut pan_orbit in pan_orbit_query.iter_mut() {
        pan_orbit.enabled = drag_state.info.is_none();
    }

    if let Some(info) = &drag_state.info {
        info!("Dragging {:?}", info.id);
        let new_pos = ray.origin + ray.direction * info.grab_distance;
        let mut particle_position = particles.get_mut(info.id).unwrap().1;
        particle_position.0 = new_pos;
        gizmos.cuboid(Transform::from_translation(new_pos).with_scale(Vec3::splat(0.1)), Color::PINK);
        
    }
}

fn update_transforms(
    mut particles: Query<(&ParticlePosition, &mut Transform), Changed<ParticlePosition>>,
) {
    for (particle_position, mut transform) in &mut particles {
        transform.translation = **particle_position;
    }
}

pub struct PlantPhysicsPlugin;
impl Plugin for PlantPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (drag_particles, update_transforms));
    }
    
}