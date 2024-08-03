use crate::prelude::*;

#[derive(Debug, Clone, Reflect)]
pub enum Shape {
    Circle {
        radius: f32,
    },
    /// A polygonal shape. NOTE: Points on the exterior should be defined in CLOCKWISE order
    Polygon {
        points: Vec<Vec2>,
    },
}
impl Shape {
    /// Given my placement and a point, figure out the signed distance and the diff need to get to a point on MY border
    /// that is closest to this point. Also return the signed distance from this point to the provided point.
    /// NOTE: The returned point is in GLOBAL, UNROTATED SPACE, relative to MY POS
    pub fn closest_point(&self, placement: (Vec2, f32), rhs: Vec2) -> (f32, Vec2) {
        let (my_pos, my_rot) = placement;
        match self {
            Self::Circle { radius: my_radius } => {
                let diff = rhs - my_pos;
                let signed_dist = diff.length() - *my_radius;
                let norm = diff.normalize_or_zero();
                (signed_dist, my_pos + norm * *my_radius)
            }
            Self::Polygon { points: my_points } => {
                let mut signed_dist = f32::MAX;
                let mut closest_point = Vec2::ZERO;
                for unplaced_line in my_points.to_lines() {
                    let placed_line = [
                        my_pos + unplaced_line[0].my_rotate(my_rot),
                        my_pos + unplaced_line[1].my_rotate(my_rot),
                    ];
                    let (test_signed_dist, test_cp) = signed_distance_to_segment(rhs, placed_line);
                    if test_signed_dist.abs() < signed_dist.abs() {
                        signed_dist = test_signed_dist;
                        closest_point = test_cp;
                    }
                }
                (signed_dist, closest_point)
            }
        }
    }

    /// Given my placement and another shape/placement combo, figure out how to push this shape
    /// out of the other. Returns None if they do not overlap. Otherwise, returns two things:
    /// 1. A diff which represents how much to move my placement by to get out of the shape
    /// 2. The exact collision point
    pub fn bounce_off(
        &self,
        placement: (Vec2, f32),
        rhs: (&Self, Vec2, f32),
    ) -> Option<(Vec2, Vec2)> {
        let (my_pos, _my_rot) = placement;
        let (rhs_bounds, rhs_pos, rhs_rot) = rhs;
        match self {
            Self::Circle { radius: my_radius } => {
                let (signed_dist, cp) = rhs_bounds.closest_point((rhs_pos, rhs_rot), my_pos);
                // NOTE: This abs is maybe not correct? Maybe it is?
                // Basically it means we'll only bounce off if we're near the edge.
                // If we're way inside another bounds, we're fucked, and we'll stay there forever.
                // This is probably fine? Idk without it there were weird bugs on edges extending down (like mario 64).
                // To handle the way inside thing would probably be another function (move outside) or something
                // with more expensive logic for shape overlap calculations.
                if signed_dist.abs() > *my_radius {
                    return None;
                }
                let dir = (my_pos - cp).normalize_or_zero();
                Some((dir * (*my_radius - signed_dist), cp))
            }
            Self::Polygon { points: _my_points } => {
                unimplemented!("Determining the push point for polygons is not yet supported");
            }
        }
    }
}
impl Shape {
    pub fn to_points(&self) -> Vec<Vec2> {
        match self {
            Self::Circle { radius } => regular_polygon(radius.ceil() as u32 * 2, 0.0, *radius),
            Self::Polygon { points } => points.clone(),
        }
    }
}
#[derive(Debug, Clone, Reflect)]
pub struct Bounds {
    shape: Shape,
    // TODO IF NEEDED: Add additional info (like a bounding circle, max/min x/y) to speed up collision detection
}
impl Bounds {
    pub fn from_shape(shape: Shape) -> Self {
        Self { shape }
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    pub fn draw(&self, pos: Vec2, rot: f32, gz: &mut Gizmos, color: Color) {
        // First draw the shape
        match self.get_shape() {
            Shape::Circle { radius } => {
                gz.circle_2d(pos, *radius, color);
            }
            Shape::Polygon { points } => {
                for [p1, p2] in points.to_lines() {
                    gz.line_2d(pos + p1.my_rotate(rot), pos + p2.my_rotate(rot), color);
                }
            }
        }
        // Then draw a line to show rotation (useful for circles)
        let diff = Vec2::X.my_rotate(rot) * 4.0;
        gz.line_2d(pos, pos + diff, color);
    }
}
