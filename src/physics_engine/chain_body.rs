//chain body
//


use cgmath::InnerSpace;


static GRAVITY_FACTOR : f32 = 0.0001;

pub struct Chain{
    pub points : Vec<Point>,
    pub point_mass : f32,
    pub segment_length : f32,
    pub width : f32,
    pub gravity_dir : cgmath::Vector2<f32>,
}

pub struct Point{
    pub x: f32,
    pub y: f32,
    vel: cgmath::Vector2<f32>,
    force: cgmath::Vector2<f32>,
}

impl Point {
    fn new(x: f32, y: f32) -> Self{
        Self { x, y,
            vel: cgmath::Vector2::new(0., 0.),
            force: cgmath::Vector2::new(0., 0.)
        }
    }

    fn as_f32_arr(&self) -> [f32; 2]{
        [self.x, self.y]
    }
}

impl From<[f32; 2]> for Point{
    fn from(value: [f32; 2]) -> Self {
        Self::new(value[0], value[1])
    }
}

impl Chain {

    pub fn new() -> Self {
        todo!();
    }

    pub fn from_coords(
        points : Vec<[f32; 2]>,
        point_mass: f32,
        segment_length: f32,
        width: f32
    ) -> Self {
        let points : Vec<Point> = points.iter().map(
            |p| {
                Point::new(p[0], p[1])
            }
        ).collect();

        Self{
            points,
            point_mass,
            segment_length,
            width,
            gravity_dir: cgmath::Vector2::new(0., -1.),
        }
    }

    pub fn simulation_step(&mut self){
        self.simulate_forces();
        //self.collision_check();
        self.simulate_velocity();
        self.simulate_movement();
    }

    pub fn simulate_forces(&mut self){
        self.apply_gravity();
        self.pull_points_together();
    }

    pub fn simulate_velocity(&mut self){
        for p in self.points.iter_mut(){
            if p.y + p.vel.y < -0.99 || 0.99 < p.y + p.vel.y {
                p.force.y *= 0.0;
                p.vel.y *= 0.0;
                p.y = if p.y < 0. {-0.98} else {0.98};
            }
            if p.x + p.vel.x < -0.99 || 0.99 < p.x + p.vel.x {
                p.force.x *= 0.0;
                p.vel.x *= 0.0;
                p.x = if p.x < 0. {-0.98} else {0.98};
            }
            p.vel += p.force;
        }
    }

    pub fn simulate_movement(&mut self){
        for p in self.points.iter_mut(){
            p.x += p.vel.x;
            p.y += p.vel.y;
        }
    }

    fn apply_gravity(&mut self){
        for p in self.points.iter_mut(){
            self.gravity_dir.normalize();
            p.force = self.gravity_dir * (self.point_mass*GRAVITY_FACTOR);
        }
    }

    fn pull_points_together(&mut self){
        if self.points.len() >= 2{
            self.set_point_pull_forces(0, 1);
        for i in 1..self.points.len()-1{
            self.set_point_pull_forces(i, i+1)
        }
        let len = self.points.len();
        self.set_point_pull_forces(len-2, len-1);
        }
    }

    fn set_point_pull_forces(&mut self, p1_id: usize, p2_id: usize){
            let p1 = &self.points[p1_id];
            let p2 = &self.points[p2_id];
            let (dx, dy) = (p2.x-p1.x, p2.y-p1.y);
            let dist = (dx*dx + dy*dy).sqrt();
            let h_delta_dist = (dist - self.segment_length)*0.5;
            let p1_to_p2 =
                cgmath::Vector2::new(dx, dy).normalize() * h_delta_dist;
            let p2_to_p1 =
                cgmath::Vector2::new(dx*-1., dy*-1.).normalize() * h_delta_dist;
            self.points[p1_id].force += p1_to_p2;
            self.points[p2_id].force += p2_to_p1;
    }

    fn collision_check(&mut self){
        for i in 0..self.points.len()-2{
            for j in i+2..self.points.len()-1{
                let mut intersection = Intersects::None;
                {
                let p1 = &self.points[i];
                let p1 = [p1.x+p1.force.x, p1.y+p1.force.y];
                let p2 = &self.points[i+1];
                let p2 = [p2.x+p2.force.x , p2.x+p2.force.y ];
                let p3 = &self.points[j];
                let p3 = [p3.x+p3.force.x, p3.y+p3.force.y ];
                let p4 = &self.points[j+1];
                let p4 = [p4.x+p4.force.x , p4.y+p4.force.y ];
                intersection = Self::get_lines_intersection_point(
                    p1, p2,
                    p3, p4
                );
                }

                match intersection{
                    Intersects::None => {
                        println!("No Collision");
                    },
                    Intersects::Infinite => {
                        println!("Infinite Points of collision");
                    },
                    Intersects::Single(inter_x, inter_y) => {
                        println!("iter-i: {i}; iter-j: {j}; collision point: x: {inter_x}; y {inter_y}");
                     //   if !(inter_x == self.points[i+1].x && inter_y == self.points[i+1].y
                       //   || inter_x == self.points[j].x && inter_y == self.points[j].y){
                        self.points[i].force.x -= self.points[i].vel.x;
                        self.points[i].force.y -= self.points[i].vel.y;
                        self.points[i+1].force.x -= self.points[i].vel.x;
                        self.points[i+1].force.y -= self.points[i].vel.y;
                        self.points[j].force.x -= self.points[i].vel.x;
                        self.points[j].force.y -= self.points[i].vel.y;
                        self.points[j+1].force.x -= self.points[i].vel.x;
                        self.points[j+1].force.y -= self.points[i].vel.y;


                        //self.points[i+1].vel.x = 0.0;
                        //self.points[i+1].vel.y = 0.0;
                        /*self.points[j].vel *= 0.0;
                        self.points[j+1].vel *= 0.0;*/
                        /*self.points[i].force *= 0.0;
                        self.points[i+1].force *= 0.0;
                        self.points[j].force *= 0.0;
                        self.points[j+1].force *= 0.0;*/
                         // }

                    }
                }

            }

        }

        match Self::get_lines_intersection_point([-0.9, 0.9], [0.9, -0.9], [-0.9, -0.9], [0.9, 0.9]){
            Intersects::Single(x, y) => {
            println!("test col at: x: {x}; y: {y}");
            },
            Intersects::Infinite => {println!("Infinite Collisions")},
            Intersects::None => {println!("None")},
        }

    }


    fn get_lines_intersection_point(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], ) -> Intersects{
        // f(x) = y = m*x + b
        // b = y - m*x
        // m = (y - b)/x
        // y = m*x + (y - m)
        // m = (y2 - y1) / (x2 - x1)

        let mut m1 = (p2[1] - p1[1]) / (p2[0] - p1[0]);
        m1 = if m1.is_normal(){m1}else{0.};
        let b1 = p1[1] - m1 * p1[0];
        let mut m2 = (p4[1] - p3[1]) / (p4[0] - p3[0]);
        m2 = if m2.is_normal(){m2}else{0.};
        let b2 = p3[1] - m2 * p3[0];
        //calculate intersection of the two lines
        let mut corner_x = (b2 - b1) / (m1 - m2);
        corner_x = if corner_x.is_normal(){corner_x}else{0.};
        let corner_y = m1*corner_x + b1;
        if (
            (p1[0] < p2[0] && (corner_x <= p1[0] && p2[0] <= corner_x))
            || (p1[0] > p2[0] && (corner_x >= p1[0] && p2[0] >= corner_x))
           &&
           (p1[1] < p2[1] && (corner_y <= p1[1] && p2[1] <= corner_y))
          || (p1[1] > p1[1] && (corner_y >= p1[1] && p2[1] >= corner_y))
          )
            ||
            (b1 != b2 && m1 == m2)
        {
                return Intersects::None;
        }
        else if b1 == b2 && m1 ==  m2{
            return Intersects::Infinite;
        }
        Intersects::Single(corner_x, corner_y)
    }

}

pub enum Intersects{
    Single(f32, f32),
    Infinite,
    None,
}
