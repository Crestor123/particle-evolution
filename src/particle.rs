use bevy::prelude::*;
use rand::distr::{Distribution, Uniform};
use std::cmp;
use std::f32::consts::PI;
//use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use super::{Extents, Group, Particle, Velocity, Friction, Charge, Bond};

const PARTICLE_RADIUS: f32 = 3.0;
const BOND_LENGTH: f32 = 12.0;
const COLOR_RED: Color = Color::hsl(0.0, 1.0, 0.5);
const COLOR_BLUE: Color = Color::hsl(240.0, 1.0, 0.5);
const COLOR_GREEN: Color = Color::hsl(100.0, 1.0, 0.5);
const COLOR_YELLOW: Color = Color::hsl(60.0, 1.0, 0.5);

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (interact, update_bonds, update_particles).chain());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_group: Query<(&Group, &Charge)>,
    extents: ResMut<Extents>
) {
    commands.spawn(Camera2d);

    //Creating groups to hold particles
    let red_group = commands.spawn((
        Group{name: String::from("red"), radius: 50.0},
        Charge(-1),
        Transform::from_xyz(0.0, 0.0, 0.0)
    )).id();

    let blue_group = commands.spawn((
        Group{name: String::from("blue"), radius: 50.0},
        Charge(1),
        Transform::from_xyz(0.0, 0.0, 0.0)
    )).id();

    let green_group = commands.spawn((
        Group{name: String::from("green"), radius: 50.0},
        Charge(2),
        Transform::from_xyz(0.0, 0.0, 0.0)
    )).id();

    let yellow_group = commands.spawn((
        Group{name: String::from("yellow"), radius: 50.0},
        Charge(-6),
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
            -1,
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
            1,
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
            2,
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
            -5,
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
    charge: i8,
    particle_color: Color,
    x_position: f32,
    y_position: f32
) -> Entity {
    //println!("{}, {}", x_position, y_position);

    let particle = commands.spawn((
        Mesh2d(meshes.add(Circle::new(PARTICLE_RADIUS),)),
        MeshMaterial2d(materials.add(particle_color)),
        Transform::from_xyz(x_position, y_position, 0.0),
        Particle {
            mass: charge.abs(),
            group: group,
            vibration: 0, 
            positive: charge.is_positive(),
            rotation: 0, 
            bonds: Vec::new()
        },
        Charge(charge),
        Velocity(Vec2::ZERO),
    )).id();
    
    particle
}

fn create_bond(
    commands: &mut Commands,
    particle_a: Entity,
    particle_b: Entity,
    charge: i8,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let bond = commands.spawn((
        Bond {
            particle_a: particle_a, 
            particle_b: particle_b
        },
        Charge(charge),
        Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, -2.0),
    )).id();

    bond
}

//Processes all pairs of particles
//Calculates the force of one particle on the other
fn interact(
    mut q_particle: Query<(Entity, &mut Particle, &mut Charge, &mut Transform, &mut Velocity)>,
    q_group: Query<(&Group, &Charge), Without<Particle>>,
    friction: Res<Friction>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut iter = q_particle.iter_combinations_mut();
    while let Some([(id_a, mut particle_a, mut charge_a, mut pos_a, mut vel_a), 
        (id_b, mut particle_b, mut charge_b, mut pos_b, mut vel_b)])
        = iter.fetch_next() {

        let (group_a, group_charge_a) = q_group.get(particle_a.group).unwrap();
        let (group_b, group_charge_b) = q_group.get(particle_b.group).unwrap();

        //Calculate force
        let radius_a = group_a.radius;
        let radius_b = group_b.radius;

        //Force is |charge_a| + |charge_b|, mutliplied by the signs of each charge
        let force = -((charge_a.0.abs() + charge_a.0.abs()) * (charge_a.0.signum() * charge_b.0.signum()));

        //Formula is based on the distance
        //Force increases as distance decreases
        let distance_squared = Vec3::distance_squared(pos_a.translation, pos_b.translation);
        let falloff = 1.0 / (f32::sqrt(distance_squared) * 2.0);
        let dir_x = pos_a.translation.x - pos_b.translation.x;
        let dir_y = pos_a.translation.y - pos_b.translation.y;

        //Collision
        if distance_squared <= (PARTICLE_RADIUS * 2.0).powf(2.0) {
            //Make sure the particles aren't inside each other
            if distance_squared < (PARTICLE_RADIUS).powf(2.0) {
                pos_a.translation.x -= dir_x * PARTICLE_RADIUS;
                pos_a.translation.y -= dir_y * PARTICLE_RADIUS;
                pos_b.translation.x += dir_x * PARTICLE_RADIUS;
                pos_b.translation.y += dir_y * PARTICLE_RADIUS;
            }

            //Bounce
            vel_a.0 *= -1.0;
            vel_b.0 *= -1.0;

            //Check for bonding
            if !(charge_a.0 == 0 || charge_b.0 == 0) && (charge_a.0 ^ charge_b.0) < 0 {
                //If the particles have opposite charges, they can bond
                //Find which particle has the lowest charge
                println!("{}, {}", charge_a.0.abs(), charge_b.0.abs());
                let charge_bond = cmp::min(charge_a.0.abs(), charge_b.0.abs());
                println!("{}", charge_bond);
                if charge_a.0.is_positive() { charge_a.0 -= charge_bond } else { charge_a.0 += charge_bond }
                if charge_b.0.is_positive() { charge_b.0 -= charge_bond } else { charge_b.0 += charge_bond }
                println!("{}, {}\n", charge_a.0, charge_b.0);

                let bond = create_bond(&mut commands, id_a, id_b, charge_bond, &mut meshes, &mut materials);
                particle_a.bonds.push(bond);
                particle_b.bonds.push(bond);
            }  
        }

        if particle_a.bonds.contains(&id_b) {
            //No forces should be applied if the two particles are bonded
            return;
        }

        //Force of A on B
        if distance_squared > (PARTICLE_RADIUS * 2.0).powf(2.0) && distance_squared < radius_a.powf(2.0) {
            vel_b.0.x += (dir_x * falloff * f32::from(force) * f32::from(particle_a.mass));
            vel_b.0.y += (dir_y * falloff * f32::from(force) * f32::from(particle_a.mass));
        }

        //Force of B on A
        if distance_squared > (PARTICLE_RADIUS * 2.0).powf(2.0) && distance_squared < radius_b.powf(2.0) {
            vel_a.0.x += (-dir_x * falloff * f32::from(force) * f32::from(particle_a.mass));
            vel_a.0.y += (-dir_y * falloff * f32::from(force) * f32::from(particle_a.mass));
        }
    }
}

fn update_bonds (
    mut q_bond: Query<(&Bond, &mut Transform, &Charge, &mut Mesh2d)>,
    mut q_particle: Query<(&Particle, &mut Transform, &mut Velocity), Without<Bond>>,
    friction: Res<Friction>,
) {
    for (bond, mut bond_pos, bond_charge, mut bond_mesh) in q_bond.iter_mut() {
        let [(p_a, mut pos_a, mut vel_a), (p_b, mut pos_b, mut vel_b)] = q_particle.get_many_mut([bond.particle_a, bond.particle_b]).unwrap();

        //Apply forces to each particle based on the bond's charge
        let dir = (pos_a.translation.xy() - pos_b.translation.xy()).normalize();

        //let distance_squared = Vec3::distance_squared(pos_a.translation, pos_b.translation);
        let distance = Vec3::distance(pos_a.translation, pos_b.translation);
        
        //Spring force
        let force = f32::from(bond_charge.0 + 1).powf(1.6) * (distance - BOND_LENGTH);

        //vel_a.0.x -= (force * f32::from(p_b.mass).powf(2.0)) * dir.x - ((1.0 - friction.0 * 2.0) * force);
        //vel_b.0.x += (force * f32::from(p_a.mass).powf(2.0)) * dir.x - ((1.0 - friction.0 * 2.0) * force);
        //vel_a.0.y -= (force * f32::from(p_b.mass).powf(2.0)) * dir.y - ((1.0 - friction.0 * 2.0) * force);
        //vel_b.0.y += (force * f32::from(p_a.mass).powf(2.0)) * dir.y - ((1.0 - friction.0 * 2.0) * force);

        //vel_a.0.x -= (force * f32::from(p_b.mass).powf(2.0)) * dir.x * friction.0;
        //vel_b.0.x += (force * f32::from(p_a.mass).powf(2.0)) * dir.x * friction.0;
        //vel_a.0.y -= (force * f32::from(p_b.mass).powf(2.0)) * dir.y * friction.0;
        //vel_b.0.y += (force * f32::from(p_a.mass).powf(2.0)) * dir.y * friction.0;

        if p_a.mass > p_b.mass {
            pos_b.translation = pos_a.translation + (Vec3::new(dir.x, dir.y, 0.0) * BOND_LENGTH);
        }
        else { 
            pos_a.translation = pos_b.translation + (Vec3::new(dir.x, dir.y, 0.0) * BOND_LENGTH);
        }

        //Update the position of the bond
        bond_pos.translation = pos_a.translation.midpoint(pos_b.translation);
        bond_pos.scale = Vec3::new(distance, 1.0, 1.0);
        bond_pos.rotation = Quat::from_rotation_z(dir.to_angle());
    }
}

fn update_particles (
    mut q_particles: Query<(&Particle, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
    extents: Res<Extents>,
    friction: Res<Friction>,
) {
    let dt = time.delta_secs();
    //Move particles based on their velocity
    //Apply friction
    for (p, mut pos, mut vel) in q_particles.iter_mut() {
        //Check for walls, bounce off
        //Might change to give some force away from wall
        if pos.translation.x < -(extents.0.x / 2.0) {
            pos.translation.x = -(extents.0.x / 2.0) + PARTICLE_RADIUS;
            vel.0.x *= -1.0;
        }
        if pos.translation.x > (extents.0.x / 2.0) {
            pos.translation.x = (extents.0.x / 2.0) - PARTICLE_RADIUS;
            vel.0.x *= -1.0;
        }
        if pos.translation.y > (extents.0.y / 2.0) {
            pos.translation.y = (extents.0.y / 2.0) - PARTICLE_RADIUS;
            vel.0.y *= -1.0;
        }
        if pos.translation.y < -(extents.0.y / 2.0) {
            pos.translation.y = -(extents.0.y / 2.0) + PARTICLE_RADIUS;
            vel.0.y *= -1.0;
        }

        //Prevents 'particle acceleration'
        //Will definitely change this to be a variable later
        vel.0 = vel.0.clamp_length_max(100.0);
        let velocity = vel.0;
        vel.0 -= velocity * (1.0 - friction.0) * dt;

        pos.translation.x += (vel.0.x) * dt;
        pos.translation.y += (vel.0.y) * dt;
    }
}