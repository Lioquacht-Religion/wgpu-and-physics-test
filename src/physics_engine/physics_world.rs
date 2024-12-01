//physics world
//
//

use cgmath::{InnerSpace, num_traits::clamp};

use crate::{gen_vec::GenVec, primitives_2d::utils::Radians};

use super::{circle_body::Circle, convex_body::Convex2D};
use crate::physics_engine::physic_obj_traits::*;

//TODO: separating shape information from Physicsbody

type Vec2 = cgmath::Vector2<f32>;
type PhyObjPointer = Box<dyn PhysicsObject>;


pub struct TempPhyObjData{
    pub pos: Vec2,
    pub vel: Option<Vec2>,
    pub torque: f32,
}

static GRAVITY_FACTOR : f32 = 0.0009;

pub struct World{
    pub global_gravity_dir: cgmath::Vector2<f32>,
    obj_count : usize,
    width_bound: (f32, f32),
    height_bound: (f32, f32),
    pub physics_objects: GenVec<Box<dyn PhysicsObject>>,//Vec<Box<dyn PhysicsObject>>,
    pub static_objects: GenVec<Box<dyn PhysicsObject>>,
}

impl World{
    pub fn new() -> Self{
        let global_gravity_dir = Vec2::new(0., -0.01);
        //let Circle { id, nodes, pos, r, m, force, vel }
        let physics_objects = GenVec::new();
        let static_objects = GenVec::new();
        Self{
            global_gravity_dir,
            obj_count: 0,
            width_bound: (-1., 1.),
            height_bound: (-1., 1.),
            physics_objects,
            static_objects,
        }
    }

    pub fn add_phy_obj<T: PhysicsObject + 'static>(&mut self, mut add: T) -> usize{
        add.set_id(self.obj_count);
        self.physics_objects.insert(Box::new(add));
        self.obj_count += 1;
        //self.obj_count = self.physics_objects.len();
        self.obj_count-1
    }

    pub fn add_circles(&mut self){
        for i in 1..2{
            let add = Circle::new(0, 0.2*i as f32 -1.0, 0.8, 0.07, 6.);
            self.add_phy_obj(add);

            /*
            let add = Circle::new(0, 0.2*i as f32 -1.0, 0.3, 0.06, 6.5);
            self.add_phy_obj(add);

            let add = Circle::new(0, 0.1*i as f32 -1.0, 0.0, 0.02 as f32, 0.5);
            self.add_phy_obj(add);
            let add = Circle::new(0, 0.1*i as f32 -1.0, -0.3, 0.02 as f32, 0.5);
            self.add_phy_obj(add);
            let add = Circle::new(0, 0.1*i as f32 -1.0, -0.5, 0.02 as f32, 0.5);
            self.add_phy_obj(add);
            let add = Circle::new(0, 0.1*i as f32 -1.0, -0.7, 0.02 as f32, 0.5);
            self.add_phy_obj(add);
            */
        }
    }
    pub fn simulation_step(&mut self){
        self.set_forces_to_zero();
        self.simulate_force();
        self.simulate_velocity();
        self.simulate_movement();
        self.collision_detection();
    }

    fn set_forces_to_zero(&mut self){
        for p in self.physics_objects.iter_mut(){
            p.get_force_mut().x = 0.0;
            p.get_force_mut().y = 0.0;
        }
    }

    fn simulate_force(&mut self){
        for p in self.physics_objects.iter_mut(){
            if p.is_static(){continue;}

            let g_force = p.get_mass()*self.global_gravity_dir*GRAVITY_FACTOR;
            *p.get_force_mut() += g_force;
        }
    }

    fn simulate_velocity(&mut self){
        for p in self.physics_objects.iter_mut(){
            p.get_vel_mut().x = clamp(p.get_vel().x, -0.01, 0.01);
            p.get_vel_mut().y = clamp(p.get_vel().y, -0.01, 0.01);
            if p.is_static(){continue;}

            p.set_angular_accel(
                cgmath::Rad(p.get_torque()/p.get_inertia())
            );


            *p.get_vel_mut() *= 0.99; //applying air resistance

            *p.get_vel_mut() = *p.get_vel() + (*p.get_force() / p.get_mass());
        }
    }

    fn simulate_movement(&mut self){
        for p in self.physics_objects.iter_mut(){
            if p.is_static(){continue;}

            p.set_angle(p.get_angle() + p.get_angular_accel());
            println!("angle in rad: {}", p.get_angle().0);
            println!("inertia: {}", p.get_inertia());

            *p.get_pos_mut() = *p.get_pos() + *p.get_vel();
        }
    }

    fn collision_detection(&mut self){
        for p in self.physics_objects.iter_mut(){
            if p.get_pos().y + p.get_vel().y < -0.99 || 0.99 < p.get_pos().y + p.get_vel().y {
                p.get_force_mut().y *= -0.4;
                p.get_vel_mut().y *= -1.0;
                p.get_pos_mut().y = if p.get_pos().y < 0. {-0.98} else {0.98};
            }
            else
            if p.get_pos().x + p.get_vel().x < -0.99 || 0.99 < p.get_pos().x + p.get_vel().x {
                p.get_force_mut().x *= -0.4;
                p.get_vel_mut().x *= -1.0;
                p.get_pos_mut().x = if p.get_pos().x < 0. {-0.98} else {0.98};
            }
        }

        //TODO: create struct to cache all changes to be made during Collisionrestricition

        let mut temp_reset_obj_data: Vec<TempPhyObjData> = Vec::with_capacity(self.physics_objects.len());
        self.physics_objects.iter().for_each(
            |obj|{
                temp_reset_obj_data.push(
                    TempPhyObjData { pos: obj.get_pos().clone(), vel: None, torque: obj.get_torque() }
                );
            }
        );


        let mut pobjs : Vec<&mut PhyObjPointer> = self.physics_objects.iter_mut().collect();
        for i in 0..pobjs.len(){//self.physics_objects.len(){
            for j in i+1..pobjs.len(){//self.physics_objects.len(){
                if let CollisionType::Circle(p_c) = pobjs[i].get_col_type(){//self.physics_objects[i].get_col_type(){
                    if let CollisionType::Circle(p_c_other) = pobjs[j].get_col_type(){//self.physics_objects[j].get_col_type(){
                        if Circle::check_col(p_c, p_c_other){
                            if let Some((t_circle1, t_circle2)) = Circle::pos_reset_to(p_c, p_c_other){
                                reset_pos(&mut pobjs, i, j,
                                          &mut temp_reset_obj_data,
                                          t_circle1, t_circle2
                                          );
                           }


                        }
                    }
                    else
                    if let CollisionType::Convex(p_c_other) = pobjs[j].get_col_type(){
                        if Circle::check_col(p_c, p_c_other){
                            if let Some((t_circle, t_convex)) = Circle::pos_reset_to(p_c, p_c_other){
                                reset_pos(&mut pobjs, i, j,
                                          &mut temp_reset_obj_data,
                                          t_circle, t_convex
                                          );
                           }

                        }
                    }
                }

                else
                if let CollisionType::Convex(p_c) = pobjs[i].get_col_type(){//self.physics_objects[i].get_col_type(){
                    if let CollisionType::Circle(p_c_other) = pobjs[j].get_col_type(){//self.physics_objects[j].get_col_type(){
                        if Circle::check_col(p_c_other, p_c){
                            if let Some((t_circle, t_convex)) = Circle::pos_reset_to(p_c_other, p_c){
                                reset_pos(&mut pobjs, i, j,
                                          &mut temp_reset_obj_data,
                                          t_convex, t_circle
                                          );
                            }
                        }
                    }
                    else
                    if let CollisionType::Convex(p_c_other) = pobjs[j].get_col_type(){
                        if Convex2D::check_col(p_c, p_c_other){
                            if let Some((t_convex1, t_convex2)) = Convex2D::pos_reset_to(p_c, p_c_other){
                                reset_pos(&mut pobjs, i, j,
                                          &mut temp_reset_obj_data,
                                          t_convex1, t_convex2
                                          );
                            }

                        }
                    }
                }



            }
        }
        for i in 0..pobjs.len(){
            if pobjs[i].is_static(){continue;}
            *pobjs[i].get_pos_mut() = temp_reset_obj_data[i].pos;
            if let Some(t_vel) = temp_reset_obj_data[i].vel{
                *pobjs[i].get_vel_mut() = t_vel;
            }
            pobjs[i].set_torque(temp_reset_obj_data[i].torque)
        }

    }

}

fn reset_pos(
    pobjs: &mut Vec<&mut PhyObjPointer>,
    i: usize,
    j: usize,
    temp_reset_pos: &mut Vec<TempPhyObjData>,
    t_obj1 : TempPhyObjData,
    t_obj2: TempPhyObjData,
    //x1: f32, y1: f32,
    //x2: f32, y2: f32
){

    let (x1, y1) = t_obj1.pos.into();
    let (x2, y2) = t_obj2.pos.into();

                                /*let pos1 = pobjs[i].get_pos();
                                if !pobjs[i].is_static()
                                && (temp_reset_pos[i].pos - pos1).magnitude2()
                                < (Vec2::new(x1, y1) - pos1).magnitude2() {*/
                                    temp_reset_pos[i].pos.x = x1;
                                    temp_reset_pos[i].pos.y = y1;
                                    println!("reset pos1: {x1}, {y1}");
                                //}
                                /*let pos2 = pobjs[j].get_pos();
                                if !pobjs[j].is_static()
                                && (temp_reset_pos[j].pos - pos2).magnitude2()
                                < (Vec2::new(x2, y2) - pos2).magnitude2() {*/
                                    temp_reset_pos[j].pos.x = x2;
                                    temp_reset_pos[j].pos.y = y2;
                                    println!("reset pos2: {x2}, {y2}");
                                //}

                                /*pobjs[i].get_pos_mut().x = temp_reset_pos[i].x;
                                pobjs[i].get_pos_mut().y = temp_reset_pos[i].y;

                                pobjs[j].get_pos_mut().x = temp_reset_pos[j].x;
                                pobjs[j].get_pos_mut().y = temp_reset_pos[j].x;*/

                                //simple impuls calc after col

                                let (pv1, pv2) = //(*pobjs[i].get_vel(), *pobjs[j].get_vel());
                                calc_vel_after_col(
                                    pobjs[i].get_pos(), pobjs[i],
                                    pobjs[j].get_pos(), pobjs[j]
                                );

                                if !pobjs[i].is_static() {
                                    if let Some(t_vel) = temp_reset_pos[i].vel{
                                        temp_reset_pos[i].vel = Some(t_vel + pv1);
                                    }
                                    else{

                                        temp_reset_pos[i].vel = Some(pv1);
                                    }
                                        temp_reset_pos[i].torque = t_obj1.torque;
                                        let reset_angle = pobjs[i].get_angle()-pobjs[i].get_angular_accel();
                                        pobjs[i].set_angle(reset_angle);
                                }
                                if !pobjs[j].is_static() {
                                    if let Some(t_vel) = temp_reset_pos[j].vel{
                                        temp_reset_pos[j].vel = Some(t_vel + pv2);
                                    }
                                    else{
                                        temp_reset_pos[j].vel = Some(pv2);
                                    }
                                        temp_reset_pos[j].torque = t_obj2.torque;
                                        let reset_angle = pobjs[j].get_angle()-pobjs[j].get_angular_accel();
                                        pobjs[j].set_angle(reset_angle);

                                }

}

fn calc_vel_after_col(pos1: &Vec2, obj1: &PhyObjPointer, pos2: &Vec2, obj2: &PhyObjPointer) -> (Vec2, Vec2){
    //formula for centric push
    // pv2 = (m2 * v2 + m1 * (2 * v1 - v2)) / (m1 + m2)
    //
    //non_centric
    // v1 = par_v1 + orth_v1
    // v2 = par_v2 + orth_v2
    // u1 = par_v2 + orth_v1
    // u2 = par_v1 + orth_v2
    //
    // doesnt work for now; rework of displacement of objects probably needed
    // doesnt work without exact positions during collision

    /*
    let par_v1_dir = pos2 - pos1;
    let orth_v1 : Vec2 = obj1.get_vel().project_on(par_v1_dir);
    //println!("par_v1_dir: {}, {}", par_v1_dir.x, par_v1_dir.y);
    //println!("orth_v1: {}, {}", orth_v1.x, orth_v1.y);
    let par_v1 = obj1.get_vel() - orth_v1;

    let par_v2_dir = pos1 - pos2;
    let orth_v2 : Vec2 = obj2.get_vel().project_on(par_v2_dir);
    //println!("par_v2_dir: {}, {}", par_v2_dir.x, par_v2_dir.y);
    //println!("orth_v2: {}, {}", orth_v2.x, orth_v2.y);
    let par_v2 = obj2.get_vel() - orth_v2;

    let u1 = par_v2 + orth_v1;
    let u2 = par_v1 + orth_v2;

    println!("u1, u2: {:?}, {:?}", u1.normalize(), u2.normalize());

    (u1, u2)
*/

    let v1 = obj1.get_vel();
    let m1 = obj1.get_mass();
    let v2 = obj2.get_vel();
    let m2 = obj2.get_mass();

    let pv1 : Vec2 = (m1 * v1 + m2 * (2. * v2 - v1)) / (m2 + m1);
    let pv2 : Vec2 = (m2 * v2 + m1 * (2. * v1 - v2)) / (m1 + m2);

    (pv1, pv2)
}






pub struct Rect{
    pub pos: Vec2,
    force: Vec2,
    vel: Vec2,
    pub w: f32,
    pub h: f32,
    pub rot: f32,
    rot_mat: cgmath::Matrix2<f32>,
}


