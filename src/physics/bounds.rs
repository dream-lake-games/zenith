use crate::prelude::*;

#[derive(Debug, Clone, Reflect)]
pub enum Shape {
    Circle {
        center: Vec2,
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
            Self::Circle {
                center,
                radius: my_radius,
            } => {
                let my_pos = my_pos + *center;
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
            Self::Circle {
                center,
                radius: my_radius,
            } => {
                let my_pos = my_pos + *center;
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
            Self::Circle { center, radius } => {
                let non_centered_points = regular_polygon(radius.ceil() as u32 * 2, 0.0, *radius);
                non_centered_points
                    .into_iter()
                    .map(|p| p + *center)
                    .collect()
            }
            Self::Polygon { points } => points.clone(),
        }
    }

    pub fn with_offset(self, offset: Vec2) -> Shape {
        match self {
            Self::Circle { center, radius } => Self::Circle {
                center: center + offset,
                radius,
            },
            Self::Polygon { points } => Self::Polygon {
                points: points.into_iter().map(|p| p + offset).collect(),
            },
        }
    }
}
#[derive(Debug, Clone, Reflect)]
pub struct Bounds {
    shapes: Vec<Shape>,
    // TODO IF NEEDED: Add additional info (like a bounding circle, max/min x/y) to speed up collision detection
}
impl Bounds {
    pub fn from_shape(shape: Shape) -> Self {
        Self {
            shapes: vec![shape],
        }
    }

    pub fn from_shapes(shapes: Vec<Shape>) -> Self {
        Self { shapes }
    }

    pub fn get_shapes(&self) -> &[Shape] {
        &self.shapes
    }

    pub fn draw(&self, pos: Vec2, rot: f32, gz: &mut Gizmos, color: Color) {
        for shape in self.get_shapes() {
            // First draw the shape
            match shape {
                Shape::Circle { center, radius } => {
                    gz.circle_2d(pos + *center, *radius, color);
                }
                Shape::Polygon { points } => {
                    for [p1, p2] in points.to_lines() {
                        gz.line_2d(pos + p1.my_rotate(rot), pos + p2.my_rotate(rot), color);
                    }
                }
            }
            // Then draw a line to show rotation (useful for shapes where rotation is not obvious)
            let diff = Vec2::X.my_rotate(rot) * 4.0;
            gz.line_2d(pos, pos + diff, color);
        }
    }

    pub fn bounce_off(
        &self,
        my_tran_n_angle: (Vec2, f32),
        other_thing: (&Self, Vec2, f32),
    ) -> Option<(Vec2, Vec2)> {
        let (other_bounds, other_tran, other_angle) = other_thing;
        for my_shape in &self.shapes {
            for other_shape in other_bounds.get_shapes() {
                let bounce =
                    my_shape.bounce_off(my_tran_n_angle, (other_shape, other_tran, other_angle));
                if bounce.is_some() {
                    return bounce;
                }
            }
        }
        None
    }
}
