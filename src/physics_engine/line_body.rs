//line body
//

struct Line{}

impl Line{
}


pub fn get_lines_intersection_point(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], ) -> Intersects{
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

pub enum Intersects{
    Single(f32, f32),
    Infinite,
    None,
}
