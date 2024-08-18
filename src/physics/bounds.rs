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
    fn bounce_off(
        &self,
        placement: (Vec2, f32),
        rhs: ((&Self, &ShapeCache), Vec2, f32),
    ) -> Option<(Vec2, Vec2)> {
        let (my_pos, _my_rot) = placement;
        let (rhs_bounds, rhs_pos, rhs_rot) = rhs;
        match self {
            Self::Circle {
                center,
                radius: my_radius,
            } => {
                let my_pos = my_pos + *center;
                let (signed_dist, cp) = rhs_bounds.0.closest_point((rhs_pos, rhs_rot), my_pos);
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
                unimplemented!("Determining the bounce off for polygons is not yet supported");
            }
        }
    }

    /// Given my placement and another shape/placement combo, figure out if these things overlap.
    /// Returns None if they do not overlap. Otherwise, returns two things:
    /// 1. A diff which represents how much to move my placement by to get out of the shape
    /// 2. The exact collision point
    fn overlaps_with(
        &self,
        cache: &ShapeCache,
        placement: (Vec2, f32),
        rhs: ((&Self, &ShapeCache), Vec2, f32),
    ) -> bool {
        match (self, rhs.0 .0) {
            (
                Self::Circle {
                    center: my_center,
                    radius: my_radius,
                },
                Self::Circle {
                    center: other_center,
                    radius: other_radius,
                },
            ) => {
                return (*my_center + placement.0).distance(*other_center + rhs.1)
                    < my_radius + other_radius
            }
            (Self::Circle { center, radius }, Self::Polygon { .. }) => {
                let ShapeCache::Polygon { triangulation } = rhs.0 .1 else {
                    panic!("Shape cache doesn't match shape 0 in overlaps_with");
                };
                for triangle in triangulation {
                    let triangle = triangle.clone().my_rotated(rhs.2).shifted(rhs.1);
                    if triangle.signed_distance_to_point(*center + placement.0) < *radius {
                        return true;
                    }
                }
                return false;
            }
            (Self::Polygon { .. }, Self::Circle { center, radius }) => {
                let ShapeCache::Polygon { triangulation } = cache else {
                    panic!("Shape cache doesn't match shape 1 in overlaps_with");
                };
                for triangle in triangulation {
                    let triangle = triangle
                        .clone()
                        .my_rotated(placement.1)
                        .shifted(placement.0);
                    if triangle.signed_distance_to_point(*center + rhs.1) < *radius {
                        return true;
                    }
                }
                return false;
            }
            (Self::Polygon { .. }, Self::Polygon { .. }) => {
                let ShapeCache::Polygon { triangulation: t1 } = cache else {
                    panic!("Shape cache doesn't match shape 2 in overlaps_with");
                };
                let ShapeCache::Polygon { triangulation: t2 } = rhs.0 .1 else {
                    panic!("Shape cache doesn't match shape 3 in overlaps_with");
                };
                let t1 = t1
                    .iter()
                    .map(|t| t.clone().my_rotated(placement.1))
                    .map(|t| t.shifted(placement.0))
                    .collect::<Vec<_>>();
                let t2 = t2
                    .iter()
                    .map(|t| t.clone().my_rotated(rhs.2))
                    .map(|t| t.shifted(rhs.1))
                    .collect::<Vec<_>>();
                for ta in t1.iter() {
                    for tb in t2.iter() {
                        if are_triangles_colliding(ta, tb) {
                            return true;
                        }
                    }
                }
                return false;
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

/// Data about a shape that helps with collision detection
/// Calculated once when the shape is created.
#[derive(Debug, Clone, Reflect)]
enum ShapeCache {
    Circle,
    Polygon { triangulation: Vec<Triangle> },
}
impl ShapeCache {
    fn from_shape(shape: &Shape) -> Self {
        match shape {
            Shape::Circle { .. } => Self::Circle,
            Shape::Polygon { points } => Self::Polygon {
                triangulation: triangulate(points),
            },
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct Bounds {
    shapes_n_caches: Vec<(Shape, ShapeCache)>,
}
impl Bounds {
    pub fn from_shape(shape: Shape) -> Self {
        let cache = ShapeCache::from_shape(&shape);
        Self {
            shapes_n_caches: vec![(shape, cache)],
        }
    }

    pub fn from_shapes(shapes: Vec<Shape>) -> Self {
        let shapes_n_caches = shapes
            .into_iter()
            .map(|s| {
                let cache = ShapeCache::from_shape(&s);
                (s, cache)
            })
            .collect();
        Self { shapes_n_caches }
    }

    fn get_shapes_n_caches(&self) -> &[(Shape, ShapeCache)] {
        &self.shapes_n_caches
    }

    pub fn draw(&self, pos: Vec2, rot: f32, gz: &mut Gizmos, color: Color) {
        for (shape, _) in self.get_shapes_n_caches() {
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
        for (my_shape, _my_cache) in &self.shapes_n_caches {
            for (other_shape, other_cache) in other_bounds.get_shapes_n_caches() {
                let bounce = my_shape.bounce_off(
                    my_tran_n_angle,
                    ((other_shape, other_cache), other_tran, other_angle),
                );
                if bounce.is_some() {
                    return bounce;
                }
            }
        }
        None
    }

    pub fn overlaps_with(
        &self,
        my_tran_n_angle: (Vec2, f32),
        other_thing: (&Self, Vec2, f32),
    ) -> bool {
        let (other_bounds, other_tran, other_angle) = other_thing;
        for (my_shape, my_cache) in &self.shapes_n_caches {
            for (other_shape, other_cache) in other_bounds.get_shapes_n_caches() {
                if my_shape.overlaps_with(
                    my_cache,
                    my_tran_n_angle,
                    ((other_shape, other_cache), other_tran, other_angle),
                ) {
                    return true;
                }
            }
        }
        false
    }
}
