//Collision Relations
//


use std::ops::Neg;

use cgmath::InnerSpace;

use crate::primitives_2d::utils::{Vec2, Mat2};

use super::{line_body::{Intersects, self},
    circle_body::Circle,
    physic_obj_traits::*,
    convex_body::Convex2D, physics_world::TempPhyObjData};


impl CollisionRelation<Circle, Circle> for Circle{
    fn check_col(object: &Circle, other: &Circle) -> bool {
        //let dist = (other.pos.x-object.pos.x).hypot(other.pos.y-object.pos.y);
        //dist <= object.r + other.r
        let dist_sq = (other.pos.x-object.pos.x).powi(2)
            + (other.pos.y-object.pos.y).powi(2);
        dist_sq <= (object.r + other.r).powi(2)
    }
    fn pos_reset_to(object: &Circle, other: &Circle) -> Option<(TempPhyObjData, TempPhyObjData)> {

        let t_circle1 = TempPhyObjData{
            pos: Vec2::new(object.pos.x-object.get_vel().x, object.pos.y-object.get_vel().y),
            vel: None,
            torque: 0.,
        };



        let t_circle2 = TempPhyObjData{
            pos: Vec2::new(other.pos.x-other.get_vel().x, other.pos.y-other.get_vel().y),
            vel: None,
            torque: 0.,
        };


        return Some((t_circle1, t_circle2));


           /*
        let a = object.get_pos() - object.get_vel();
        let b = other.get_pos() - other.get_vel();
        let r_dist = object.r + other.r;
        let mut a_to_b = b - a;
        a_to_b = r_dist.powi(2)/a_to_b.magnitude2() * a_to_b;
        match line_body::get_lines_intersection_point(
            [a.x + a_to_b.x, a.y + a_to_b.y],
            [object.get_pos().x + a_to_b.x, object.get_pos().y + a_to_b.y],
            [b.x, b.y],
            [other.get_pos().x, other.get_pos().y]
        ){
            Intersects::Single(x, y) => {
                Some(((x-a_to_b.x, y-a_to_b.y), (x, y)))
            },
            Intersects::Infinite => {
                let new_pos1 = object.get_pos() + (object.get_vel()
                    - (object.r.powi(2)/object.get_vel().magnitude2()
                        * object.get_vel()));
                let new_pos2 = other.get_pos() + (other.get_vel()
                    - (other.r.powi(2)/other.get_vel().magnitude2()
                        * other.get_vel()));
                println!("Infintite collisions: {:?}; {:?}", new_pos1, new_pos2);

                Some(((new_pos1.x, new_pos1.y), (new_pos1.x, new_pos2.y)))
            },
            Intersects::None => {
                None
            },
        }
        */
    }
}

fn get_closest_point_to_line(
    px: f32, py: f32,
    x1: f32, y1: f32,
    x2: f32, y2: f32
) -> (f32, f32){
    let dx = x1 - x2;
    let dy = y1 - y2;
    let len_sq = dx.powi(2) + dy.powi(2);
    let dot = ( ((px-x1)*(x2-x1)) + ((py-y1)*(y2-y1)) ) / len_sq;
    let closest_x = x1 + (dot * (x2-x1));
    let closest_y = y1 + (dot * (y2-y1));
    (closest_x, closest_y)
}

fn point_in_line_bounds(
    px: f32, py: f32,
    mut x1: f32, mut y1: f32,
    mut x2: f32, mut y2: f32
) -> bool{
    if x1 > x2 { std::mem::swap(&mut x1, &mut x2); }
    if y1 > y2 { std::mem::swap(&mut y1, &mut y2); }
    const ERR : f32 = 0.0001;
    (x1-ERR <= px && px <= x2+ERR) && (y1-ERR <= py && py <= y2+ERR)
}


//TODO: check for circle being inside of Convex
//maybe not needed, if velocity is bounded

impl CollisionRelation<Circle, Convex2D> for Circle{
    fn check_col(object: &Circle, other: &Convex2D) -> bool {
        let rot_mat = Mat2::from_angle(other.get_angle());
        let (mut p_vx, mut p_vy) =
            other.transformed_vertex(&rot_mat, 0);

        for i in 1..other.vertices.len(){
            let (vx, vy) =
                other.transformed_vertex(&rot_mat, i);
            let (cl_x, cl_y) = get_closest_point_to_line(
                object.pos.x, object.pos.y,
                p_vx, p_vy, vx, vy
            );
           if !point_in_line_bounds(cl_x, cl_y, p_vx, p_vy, vx, vy){
              p_vx = vx;
              p_vy = vy;
              continue;
            }

            let dx : f32 = cl_x - object.pos.x;
            let dy : f32 = cl_y - object.pos.y;

            let dist_sq = dx.powi(2) + dy.powi(2);

            if dist_sq <= object.r.powi(2){
                return true;
            }

            p_vx = vx;
            p_vy = vy;
        }
        let (vx, vy) =
            other.transformed_vertex(&rot_mat, 0);

            let (cl_x, cl_y) = get_closest_point_to_line(
                object.pos.x, object.pos.y,
                p_vx, p_vy, vx, vy
            );
           if !point_in_line_bounds(cl_x, cl_y, p_vx, p_vy, vx, vy){
              return false;
            }

            let dx : f32 = cl_x - object.pos.x;
            let dy : f32 = cl_y - object.pos.y;

            let dist_sq = dx.powi(2) + dy.powi(2);

            if dist_sq <= object.r.powi(2){
                return true;
            }

        false
    }

    fn pos_reset_to(object: &Circle, other: &Convex2D) -> Option<(TempPhyObjData, TempPhyObjData)> {



        let r = object.get_pos();
        let f = object.get_force();

        let convex_torque = other.calc_torque(r, f);

        let t_circle = TempPhyObjData{
            pos: Vec2::new(object.pos.x-object.get_vel().x, object.pos.y-object.get_vel().y),
            vel: None,
            torque: 0.,
        };



        let t_convex = TempPhyObjData{
            pos: Vec2::new(other.pos.x-other.get_vel().x, other.pos.y-other.get_vel().y),
            vel: None,
            torque: convex_torque,
        };

        return Some((t_circle, t_convex));
        /*

        let (mut cl_x, mut cl_y) = (0., 0.);
        let (mut p_vx, mut p_vy) = (other.vertices[0].0, other.vertices[0].1);

        for i in 1..other.vertices.len(){
            let (vx, vy) = other.vertices[i];
            let (l_cl_x, l_cl_y) = get_closest_point_to_line(
                object.pos.x, object.pos.y,
                p_vx, p_vy, vx, vy
            );
            if !point_in_line_bounds(cl_x, cl_y, p_vx, p_vy, vx, vy){
              p_vx = vx;
              p_vy = vy;
              continue;
            }

            let dx : f32 = cl_x - object.pos.x;
            let dy : f32 = cl_y - object.pos.y;

            let dist_sq = dx.powi(2) + dy.powi(2);

            if dist_sq <= object.r.powi(2){
                return Some(((cl_x, cl_y), (0., 0.)));
            }

            cl_x = l_cl_x;
            cl_y = l_cl_y;
            p_vx = vx;
            p_vy = vy;
        }*/
        //None
    }
}
//impl CollisionRelation<Convex2D, Circle> for Convex2D{}

impl CollisionRelation<Convex2D, Convex2D> for Convex2D{
    fn check_col(object: &Convex2D, other: &Convex2D) -> bool {
        convex_convex_collision(object, other)
    }

    fn pos_reset_to(object: &Convex2D, other: &Convex2D) -> Option<(TempPhyObjData, TempPhyObjData)> {
        let t_circle1 = TempPhyObjData{
            pos: object.pos - object.get_vel(),
            vel: None,
            torque: 0.,
        };

        let t_circle2 = TempPhyObjData{
            pos: other.pos - other.get_vel(),
            vel: None,
            torque: 0.,
        };
        return Some((t_circle1, t_circle2));
    }
}


//uses Separating Axis Theorem - SAT
//orthogenal normal vector of edge is used as axis direction
// a, b: vertices of edge
// n: normal vector of edge
// v: vertex to check for Collision
// (v-a) * n > 0
// -> if true, then v is in front of edge
fn sat_col_check(convex1 : &Convex2D, convex2 : &Convex2D) -> bool{
    if convex1.vertices.len() < 2 || convex2.vertices.len() < 2{
        return false;
    }
    let rot_mat1 = Mat2::from_angle(convex1.get_angle());
    let rot_mat2 = Mat2::from_angle(convex2.get_angle());

    for i in 0..convex1.vertices.len()-1{
        let a : Vec2 = convex1.transformed_vertex(&rot_mat1, i).into();
        let b : Vec2 = convex1.transformed_vertex(&rot_mat1, i+1).into();
        let n : Vec2 = Vec2::new((b.y-a.y)*-1., b.x-a.x).normalize();

        let mut verts_in_front_of_edge = true;

        for j in 0..convex2.vertices.len(){
            let v : Vec2 = convex2.transformed_vertex(&rot_mat2, j).into();
            if !((v-a).dot(n) > 0.){
                verts_in_front_of_edge = false;
            }
        }
        if verts_in_front_of_edge{
            return false;
        } // no collision possible
    }

    let a : Vec2 = convex1.transformed_vertex(&rot_mat1, convex1.vertices.len()-1).into();
    let b : Vec2 = convex1.transformed_vertex(&rot_mat1, 0).into();
    let n : Vec2 = Vec2::new((b.y-a.y)*-1., b.x-a.x).normalize();
    let mut verts_in_front_of_edge = true;
    for i in 0..convex2.vertices.len(){
        let v : Vec2 = convex2.transformed_vertex(&rot_mat2, i).into();
        if !((v-a).dot(n) > 0.){
            verts_in_front_of_edge = false;
        }
    }
    if verts_in_front_of_edge{
        return false;
    } // no collision possible
    true
}


//finds farthest vertex in vertices in direction of d
fn get_support_point(convex: &Convex2D, d: &Vec2) -> Vec2{
    let mut highest = -f32::MAX;
    let mut support = (0., 0.);
    let rot_mat = Mat2::from_angle(convex.get_angle());
    for i in 0..convex.vertices.len(){
        let v = convex.transformed_vertex(&rot_mat, i);
        let dot = v.0*d.x + v.1*d.y;
        if dot > highest{
            highest = dot;
            support = v;
        }
    }
    support.into()
}

//searchest for the farthest vertex in direction d for convex1
//and -d for convex
//then calculates the resulting minkowski difference from the to points
fn get_support_in_minkowski_diff(
    convex1: &Convex2D, convex2 : &Convex2D, d: &Vec2
) -> Vec2{
    let c1_fp = get_support_point(&convex1, d);
    let c2_fp = get_support_point(&convex2, &(d * -1.));
    c1_fp - c2_fp
}


//is not really needed, you dont want to calculate the whole minkowski difference
fn minkowski_difference(convex1: &Convex2D, convex2 : &Convex2D) -> Vec<(f32, f32)>{
    let mut resulting_shape : Vec<(f32, f32)> =
        Vec::with_capacity(convex1.vertices.len()*convex2.vertices.len());

    let rot_mat1 = Mat2::from_angle(convex1.get_angle());
    let rot_mat2 = Mat2::from_angle(convex2.get_angle());

    for i in 0..convex1.vertices.len(){
        let (c1_vert_x, c1_vert_y) = convex1.transformed_vertex(&rot_mat1, i);

        for j in 0..convex2.vertices.len(){
            let (c2_vert_x, c2_vert_y) = convex2.transformed_vertex(&rot_mat2, j);
            resulting_shape.push((c1_vert_x - c2_vert_x, c1_vert_y - c2_vert_y));
        }
    }

    resulting_shape
}


fn triple_product(a: Vec2, b: Vec2, c: Vec2) -> Vec2{
    b*c.dot(a) - a*c.dot(b)
}

fn simplex_contains_origin(simplex: &mut Vec<Vec2>, d: &mut Vec2) -> bool{

    let a = simplex.last().unwrap();
    let ao = a*(-1.);
    if simplex.len() == 3{
        let b = simplex[0];
        let c = simplex[1];

        let ab = b - a;
        let ac = c - a;

        let ab_perp = triple_product(ac, ab, ab);
        let ac_perp = triple_product(ab, ac, ac);

        if ab_perp.dot(ao) > 0.{
            simplex.remove(1);
            *d = ab_perp;
        }
        else{
            if ac_perp.dot(ao) > 0.{
                simplex.remove(0);
                *d = ac_perp;
            }
            else{return true;}
        }
    }
    else{
        let b = simplex[0];
        let ab = b - a;
        let ab_perp = triple_product(ab, ao, ab);
        *d = ab_perp;
    }
    false
}

fn gjk_col_check(convex1: &Convex2D, convex2 : &Convex2D) -> bool{
    let mut simplex : Vec<Vec2> = Vec::with_capacity(3);
    let mut d = convex2.pos - convex1.pos;
    simplex.push(get_support_in_minkowski_diff(convex1, convex2, &d));
    d = d * -1.;

    loop{
        simplex.push(get_support_in_minkowski_diff(convex1, convex2, &d));

        if let Some(last_v) = simplex.last(){
            if last_v.dot(d) <= 0.{
                return false;
            }
            else{
                if simplex_contains_origin(&mut simplex, &mut d) {
                    return true;
                }
            }
        }
    }
}

fn convex_convex_collision(convex1 : &Convex2D, convex2 : &Convex2D) -> bool{
    //println!("farthest points: {:?}, {:?}", c1_fp, c2_fp);
    //sat_col_check(convex1, convex2)
    gjk_col_check(convex1, convex2)
}



