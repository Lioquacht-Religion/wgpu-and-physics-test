//convex_body.rs
//


use crate::primitives_2d::utils::{Vec2, Radians, Mat2};

use super::physic_obj_traits::*;


//ATTENTION: Vertices do not get translated
//and will keep their initial value at the origin point
//this needs to be of than handling collisions

pub struct Convex2D{
    pub static_body : bool,
    pub pos: Vec2,
    m : f32,
    force: Vec2,
    torque: f32,
    vel: Vec2,
    angle: cgmath::Rad<f32>,
    ang_vel: cgmath::Rad<f32>,
    pub vertices: Vec<(f32, f32)>,
    inertia: f32,
    id : usize,
    nodes : Vec<usize>,
}

impl Convex2D{
    pub fn new(x: f32, y: f32, vertices: Vec<(f32, f32)>, m : f32) -> Self{
        let mut convex = Self {
            static_body: false,
            pos: Vec2::new(x, y),
            force: Vec2::new(0., 0.),
            vel: Vec2::new(0., 0.),
            vertices,
            m,
            torque: 0.,
            angle: cgmath::Rad(0.),
            ang_vel: cgmath::Rad(0.),
            inertia: 1.,
            id: 0,
            nodes : vec![],
        };

        convex.inertia = convex.calc_inertia();

        convex
    }

    pub fn calc_inertia(&self) -> f32{
        /*
         *

         probably meant for 3D Polygons planes:

         I =
         m * (
         sig(||P[n+1] x P[n]|| * ( (P[n]*P[n]) + (P[n]*P[n+1]) + (P[n+1]*P[n+1]) ))
         / 6 * sig(||P[n+1] X P[n]||)
         )

         oder anscheinend:

         I = sig(m*r.pow(2))

         r: distance of each element from its axis of rotation

         */

        let sigma_dist_sq : f32 = self.vertices.iter().map(
            |v| {
                let (x, y) = v;
                x.powi(2) + y.powi(2)
            }
        ).sum();

        self.m*sigma_dist_sq
    }

    pub fn calc_torque(&self, r: &Vec2, f: &Vec2) -> f32 {
        r.x*f.y - r.y*f.x
    }

    pub fn calc_angular_accel(&self) -> cgmath::Rad<f32>{
        cgmath::Rad(self.torque / self.inertia)
    }

    //TODO: think about if rotation matrix should be stored in struct

    pub fn transformed_vertex(&self, rot: &Mat2, vertex_index: usize) -> (f32, f32){
        let (x, y) = self.vertices[vertex_index];
        let v = Vec2::new(x, y);
        (rot * v + self.pos).into()
    }
}

impl PhysicsObject for Convex2D{
    fn is_static(&self)-> bool {
        self.static_body
    }
}

impl ForceObject for Convex2D{
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
        self.ang_vel.clone()
    }
    fn set_angular_accel(&mut self, angle : Radians) {
        self.ang_vel = angle;
    }

}

impl TransposeObject for Convex2D{
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

impl NodeObject for Convex2D{
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

impl CollisionObject for Convex2D{
    fn get_col_type(&self) -> CollisionType {
        CollisionType::Convex(&self)
    }
}
