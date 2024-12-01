//circle body
//
//

use crate::{physics_engine::physic_obj_traits::*, primitives_2d::utils::Radians};

type Vec2 = cgmath::Vector2<f32>;

pub struct Circle{
    id: usize,
    nodes: Vec<usize>,
    pub pos: Vec2,
    pub r: f32,
    m: f32,
    force: Vec2,
    vel: Vec2,
    torque: f32,
    angle: cgmath::Rad<f32>,
    ang_vel: cgmath::Rad<f32>,
    inertia: f32,
}

impl Circle {

    pub fn new(id: usize, x: f32, y: f32, r: f32, m: f32) -> Self{
        Self {
            id, nodes: vec![],
            pos: Vec2::new(x, y),
            r, m,
            force: Vec2::new(0., 0.), vel: Vec2::new(0., 0.),
            torque : 0.,
            angle : cgmath::Rad(0.),
            ang_vel: cgmath::Rad(0.),
            inertia: 1.,
        }
    }

    pub fn check_col_with_aabb_line(&self, line_coord1: f32, line_coord2: f32, horizontal: bool) -> bool {
        if !horizontal{
            self.pos.x + self.vel.x - self.r <= line_coord1
                || self.pos.x + self.vel.x + self.r >= line_coord2
        }
        else{
            self.pos.y + self.vel.y - self.r <= line_coord1
                || self.pos.y + self.vel.y + self.r >= line_coord2
        }
    }

}

impl PhysicsObject for Circle{}

impl ForceObject for Circle{
    fn get_mass(&self) -> f32 {
        self.m
    }
    fn get_force(&self) -> &Vec2 {
        &self.force
    }
    fn get_force_mut(&mut self) -> &mut Vec2 {
        &mut self.force
    }
    fn get_vel(&self) -> &Vec2 {
        &self.vel
    }
    fn get_vel_mut(&mut self) -> &mut Vec2 {
        &mut self.vel
    }


    fn get_torque(&self) -> f32 {
        self.torque
    }
    fn set_torque(&mut self, torque: f32) {
        self.torque = torque;
    }
    fn get_inertia(&self) -> f32 {
        self.inertia
    }
    fn set_inertia(&mut self, inertia: f32) {
        self.inertia = inertia;
    }
    fn get_angular_accel(&self) -> Radians {
        self.ang_vel
    }
    fn set_angular_accel(&mut self, angle : Radians) {
        self.ang_vel = angle;
    }

}

impl TransposeObject for Circle{
    fn get_pos(&self) -> &cgmath::Vector2<f32> {
        &self.pos
    }
    fn get_pos_mut(&mut self) -> &mut cgmath::Vector2<f32> {
        &mut self.pos
    }
    fn get_angle(&self) -> Radians {
        self.angle
    }
    fn set_angle(&mut self, angle: Radians) {
        self.angle = angle;
    }
}

impl NodeObject for Circle{
    fn get_id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn get_connections(&self) -> &[usize] {
        &self.nodes[..]
    }
}

impl CollisionObject for Circle{
    fn get_col_type(&self) -> CollisionType {
        CollisionType::Circle(self)
    }
}


