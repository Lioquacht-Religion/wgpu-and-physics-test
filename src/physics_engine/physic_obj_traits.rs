//phyobj traits
//

use super::{circle_body::Circle, convex_body::Convex2D, physics_world::TempPhyObjData};

type Vec2 = cgmath::Vector2<f32>;
type Radians = cgmath::Rad<f32>;

pub trait PhysicsObject : NodeObject + TransposeObject + ForceObject + CollisionObject{
    fn is_static(&self)-> bool {false}
}

pub trait TransposeObject{
    fn get_pos(&self) -> &cgmath::Vector2<f32>;
    fn get_pos_mut(&mut self) -> &mut cgmath::Vector2<f32>;
    fn add_pos(&mut self, add: &Vec2){
        self.get_pos_mut().x += add.x;
        self.get_pos_mut().y += add.y;
    }

    fn get_angle(&self) -> Radians;
    fn set_angle(&mut self, angle: Radians);
}

pub trait ForceObject{
    fn get_mass(&self) -> f32;
    fn get_force(&self) -> &Vec2;
    fn get_force_mut(&mut self) -> &mut Vec2;
    fn get_vel(&self) -> &Vec2;
    fn get_vel_mut(&mut self) -> &mut Vec2;
    fn get_torque(&self) -> f32;
    fn set_torque(&mut self, torque: f32);
    fn get_angular_accel(&self) -> Radians;
    fn set_angular_accel(&mut self, angle : Radians);
    fn get_inertia(&self) -> f32;
    fn set_inertia(&mut self, inertia: f32);
}

pub enum CollisionType<'a>{
    NoCollision,
    Circle(&'a Circle),
    //{x: f32, y: f32, r: f32},
    Line{x1: f32, y1: f32, x2: f32, y2: f32},
    Rectangle,
    Convex(&'a Convex2D),
}

pub trait CollisionObject{
    fn get_col_type(&self) -> CollisionType{
        CollisionType::NoCollision
    }
}

pub trait CollisionRelation<T, O>{
    fn check_col(object: &T, other: &O) -> bool;
    fn pos_reset_to(object: &T, other: &O) -> Option<(TempPhyObjData, TempPhyObjData)>;
}

pub trait NodeObject{
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id : usize);
    fn get_connections(&self) -> &[usize];
}
