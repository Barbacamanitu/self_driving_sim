use bevy::prelude::*;

const TURN_SPEED: f32 = 2.8;
const FRICTION: f32 = 250.0;
const ACCELERATION: f32 = 1200.0;
const MAX_SPEED: f32 = 500.0;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, car_setup_system)
            .add_system(keyboard_input_system)
            .add_system(car_movement_system)
            .add_system(car_camera_follow_system);
    }
}

fn car_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/car_red.png"),
            transform: Transform::from_xyz(100., 0., 100.),
            ..default()
        })
        .insert(CarDirection::default())
        .insert(CarMovement::default())
        .insert(Car);
}

#[derive(Component)]
struct CarDirection {
    pub forward: bool,
    pub reverse: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Component)]
struct CarMovement {
    pub speed: f32,
    pub acceleration: f32,
    pub max_speed: f32,
    pub angle: f32,
}

#[derive(Component)]
pub struct Car;

impl CarMovement {
    pub fn update(&mut self, dir: &CarDirection, time: &Time) {
        if dir.forward {
            self.speed += self.acceleration * time.delta_seconds();
        }
        if dir.reverse {
            self.speed -= self.acceleration * time.delta_seconds();
        }

        //Change angle
        let flip = if self.speed > 0.0 { 1.0 } else { -1.0 };
        if self.speed.abs() > 0.1 {
            if dir.left {
                self.angle += TURN_SPEED * time.delta_seconds() * flip;
            }

            if dir.right {
                self.angle -= TURN_SPEED * time.delta_seconds() * flip;
            }
        }

        self.speed = self.speed.clamp(-self.max_speed / 2.0, self.max_speed);

        let frict = FRICTION * time.delta_seconds();
        //Apply friction
        if self.speed.abs() > (frict * 1.1) {
            self.speed = self.speed - (frict * self.speed.signum());
        } else {
            self.speed = 0.0;
        }
    }

    pub fn apply(&self, transform: &mut Transform, time: &Time) {
        transform.translation.x -= self.angle.sin() * self.speed * time.delta_seconds();
        transform.translation.y += self.angle.cos() * self.speed * time.delta_seconds();

        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, self.angle);
    }
}

impl Default for CarMovement {
    fn default() -> Self {
        Self {
            speed: 0.0,
            acceleration: ACCELERATION,
            max_speed: MAX_SPEED,
            angle: 0.0,
        }
    }
}

impl Default for CarDirection {
    fn default() -> Self {
        Self {
            forward: false,
            reverse: false,
            left: false,
            right: false,
        }
    }
}

fn car_camera_follow_system(
    cars: Query<(&Transform, &CarMovement), Without<Camera2d>>,
    mut cameras: Query<&mut Transform, With<Camera2d>>,
) {
    for (car_transform, movement) in &cars {
        for mut cam_trans in &mut cameras {
            cam_trans.translation.x = car_transform.translation.x;
            cam_trans.translation.y = car_transform.translation.y;
        }
    }
}

fn car_movement_system(
    time: Res<Time>,
    mut cars: Query<(&mut CarDirection, &mut Transform, &mut CarMovement)>,
) {
    for (mut dir, mut transform, mut movement) in &mut cars {
        movement.update(&dir, &time);
        movement.apply(&mut transform, &time);
    }
}

fn keyboard_input_system(keys: Res<Input<KeyCode>>, mut car: Query<&mut CarDirection>) {
    let (w, w_r) = (
        keys.just_pressed(KeyCode::W),
        keys.just_released(KeyCode::W),
    );
    let (s, s_r) = (
        keys.just_pressed(KeyCode::S),
        keys.just_released(KeyCode::S),
    );
    let (a, a_r) = (
        keys.just_pressed(KeyCode::A),
        keys.just_released(KeyCode::A),
    );
    let (d, d_r) = (
        keys.just_pressed(KeyCode::D),
        keys.just_released(KeyCode::D),
    );

    for mut dir in &mut car {
        if w {
            dir.forward = true;
        }
        if w_r {
            dir.forward = false;
        }
        if s {
            dir.reverse = true;
        }
        if s_r {
            dir.reverse = false;
        }
        if a {
            dir.left = true;
        }
        if a_r {
            dir.left = false;
        }
        if d {
            dir.right = true;
        }
        if d_r {
            dir.right = false;
        }
    }
}
