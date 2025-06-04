use bevy::prelude::*;
use rand::distr::{Distribution, Uniform};
//use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use super::{Extents, Group, Particle, Velocity, Friction};

const PARTICLE_RADIUS: f32 = 3.0;
const COLOR_RED: Color = Color::hsl(0.0, 1.0, 0.5);
const COLOR_BLUE: Color = Color::hsl(240.0, 1.0, 0.5);
const COLOR_GREEN: Color = Color::hsl(100.0, 1.0, 0.5);
const COLOR_YELLOW: Color = Color::hsl(60.0, 1.0, 0.5);

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (interact, update_particles).chain());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    extents: ResMut<Extents>
) {
    commands.spawn(Camera2d);

    //Creating groups to hold particles
    let red_group = commands.spawn((
        Group{name: String::from("red"), charge: -1, radius: 50.0},
        Transform::from_xyz(0.0, 0.0, 0.0)
    )).id();

    let blue_group = commands.spawn((
        Group{name: String::from("blue"), charge: 1, radius: 50.0},
        Transform::from_xyz(0.0, 0.0, 0.0)
    )).id();

    let green_group = commands.spawn((
        Group{name: String::from("green"), charge: 2, radius: 50.0},
        Transform::from_xyz(0.0, 0.0, 0.0)
    )).id();

    let yellow_group = commands.spawn((
        Group{name: String::from("yellow"), charge: -5, radius: 50.0},
        Transform::from_xyz(0.0, 0.0, 0.0)
    )).id();

    //Randomly placing particles
    let mut rng = rand::rng();
    let x_range = Uniform::try_from(
        -(extents.0.x / 2.0 - PARTICLE_RADIUS)..(extents.0.x / 2.0 - PARTICLE_RADIUS))
        .unwrap();
    let y_range = Uniform::try_from(
        -(extents.0.y / 2.0 - PARTICLE_RADIUS)..(extents.0.y / 2.0 - PARTICLE_RADIUS))
        .unwrap();

    //Need to make this code generic
    for _i in 0..100 {
        let posx : f32 = x_range.sample(&mut rng);
        let posy : f32 = y_range.sample(&mut rng);

        let _p = create_particle(
            &mut commands, 
            &mut meshes, 
            &mut materials,
            red_group,
            COLOR_RED,
            posx,
            posy
        );
    }
    for _i in 0..100 {
        let posx : f32 = x_range.sample(&mut rng);
        let posy : f32 = y_range.sample(&mut rng);

        let _p = create_particle(
            &mut commands,
            &mut meshes,
            &mut materials,
            blue_group,
            COLOR_BLUE,
            posx,
            posy
        );
    }
    for _i in 0..100 {
        let posx : f32 = x_range.sample(&mut rng);
        let posy : f32 = y_range.sample(&mut rng);

        let _p = create_particle(
            &mut commands,
            &mut meshes,
            &mut materials,
            green_group,
            COLOR_GREEN,
            posx,
            posy
        );
    }
    for _i in 0..100 {
        let posx : f32 = x_range.sample(&mut rng);
        let posy : f32 = y_range.sample(&mut rng);

        let _p = create_particle(
            &mut commands,
            &mut meshes,
            &mut materials,
            yellow_group,
            COLOR_YELLOW,
            posx,
            posy
        );
    }
    
}

fn create_particle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    group: Entity,
    particle_color: Color,
    x_position: f32,
    y_position: f32
) -> Entity {
    //println!("{}, {}", x_position, y_position);

    let particle = commands.spawn((
        Mesh2d(meshes.add(Circle::new(PARTICLE_RADIUS),)),
        MeshMaterial2d(materials.add(particle_color)),
        Transform::from_xyz(x_position, y_position, 0.0),
        Particle {group: group, vibration: 0, rotation: 0},
        Velocity(Vec2::ZERO),
    )).id();
    
    particle
}

//Processes all pairs of particles
//Calculates the force of one particle on the other
fn interact(
    mut q_particle: Query<(&Particle, &Transform, &mut Velocity)>,
    q_group: Query<&Group>,
    friction: Res<Friction>
) {
    let mut iter = q_particle.iter_combinations_mut();
    while let Some([(particle_a, pos_a, mut vel_a), 
        (particle_b, pos_b, mut vel_b)])
        = iter.fetch_next() {

        let group_a = q_group.get(particle_a.group).unwrap();
        let group_b = q_group.get(particle_b.group).unwrap();

        //Calculate force
        let charge_a = group_a.charge;
        let charge_b = group_b.charge;
        let radius_a = group_a.radius;
        let radius_b = group_b.radius;

        //Force is |charge_a| + |charge_b|, mutliplied by the signs of each charge
        let force = -((charge_a.abs() + charge_a.abs()) * (charge_a.signum() * charge_b.signum()));

        //Formula is based on the distance
        //Force increases as distance decreases
        let distance_squared = Vec3::distance_squared(pos_a.translation, pos_b.translation);
        let falloff = 1.0 / f32::sqrt(distance_squared);
        let dir_x = pos_a.translation.x - pos_b.translation.x;
        let dir_y = pos_a.translation.y - pos_b.translation.y;

        //Force of A on B
        if distance_squared > 0.0 && distance_squared < radius_a.powf(2.0) {
            vel_b.0.x += (dir_x * falloff * f32::from(force) * 2.0) * (1.0 / friction.0);
            vel_b.0.y += (dir_y * falloff * f32::from(force) * 2.0) * (1.0 / friction.0);
        }

        //Force of B on A
        if distance_squared > 0.0 && distance_squared < radius_b.powf(2.0) {
            vel_a.0.x += (-dir_x * falloff * f32::from(force) * 2.0) * (1.0 / friction.0);
            vel_a.0.y += (-dir_y * falloff * f32::from(force) * 2.0) * (1.0 / friction.0);
        }
    }
}

fn update_particles (
    mut q_particles: Query<(&Particle, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
    extents: Res<Extents>
) {
    let dt = time.delta_secs();
    //Move particles based on their velocity
    //Apply friction
    for (_p, mut pos, mut vel) in q_particles.iter_mut() {
        //Check for walls, bounce off
        //Might change to give some force away from wall
        if pos.translation.x < -(extents.0.x / 2.0) {
            vel.0.x *= -1.0;
        }
        if pos.translation.x > (extents.0.x / 2.0) {
            vel.0.x *= -1.0;
        }
        if pos.translation.y > (extents.0.y / 2.0) {
            vel.0.y *= -1.0;
        }
        if pos.translation.y < -(extents.0.y / 2.0) {
            vel.0.y *= -1.0;
        }

        //Prevents 'particle acceleration'
        //Will definitely change this to be a variable later
        vel.0 = vel.0.clamp_length_max(100.0);

        pos.translation.x += (vel.0.x) * dt;
        pos.translation.y += (vel.0.y) * dt;
    }
}