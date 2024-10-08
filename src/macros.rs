/// Creates a transform from a translation.
/// Supports (x, y) (assume z = 0), (x, y, z), or Vec3
#[macro_export]
macro_rules! tran_tran {
    // Only x and y provided, z assumed to be zero
    ($x:expr, $y:expr $(,)?) => {{
        Transform::from_translation(Vec3::new($x, $y, 0.0))
    }};
    // All provided
    ($x:expr, $y:expr, $z:expr $(,)?) => {{
        Transform::from_translation(Vec3::new($x, $y, $z))
    }};
    // Vec3 provided
    ($v:expr $(,)?) => {{
        Transform::from_translation($v)
    }};
}
pub use tran_tran;

/// Creates a transform from a translation.
/// Supports (x, y) (assume z = 1), (x, y, z), or Vec3
#[macro_export]
macro_rules! scale_tran {
    // Only x and y provided, z assumed to be one
    ($x:expr, $y:expr) => {{
        Transform::from_scale(Vec3::new($x, $y, 1.0))
    }};
    // All provided
    ($x:expr, $y:expr, $z:expr) => {{
        Transform::from_scale(Vec3::new($x, $y, $z))
    }};
    // Vec3 provided
    ($v:expr) => {{
        Transform::from_scale($v)
    }};
}
pub use scale_tran;

/// Creates a spatial bundle from a translation
#[macro_export]
macro_rules! spat_tran {
    // Only x and y provided, z assumed to be zero
    ($x:expr, $y:expr) => {{
        SpatialBundle::from_transform(Transform::from_translation(Vec3::new($x, $y, 0.0)))
    }};
    // All provided
    ($x:expr, $y:expr, $z:expr) => {{
        SpatialBundle::from_transform(Transform::from_translation(Vec3::new($x, $y, $z)))
    }};
    // Vec3 provided
    ($v:expr) => {{
        SpatialBundle::from_transform(Transform::from_translation($v))
    }};
}
pub use spat_tran;

/// Implements `get` for a field
#[macro_export]
macro_rules! impl_get {
    ($field:ident, $type:ty) => {
        paste::paste! {
            #[allow(unused)]
            pub fn [<get_ $field>](&self) -> $type {
                self.$field
            }
        }
    };
}
pub use impl_get;

/// Implements `get` for a field that returns a reference
#[macro_export]
macro_rules! impl_get_ref {
    ($field:ident, $type:ty) => {
        paste::paste! {
            #[allow(unused)]
            pub fn [<get_ $field>](&self) -> &$type {
                &self.$field
            }
        }
    };
}
pub use impl_get_ref;

/// Implements `set` for a field
#[macro_export]
macro_rules! impl_set {
    ($field:ident, $type:ty) => {
        paste::paste! {
            #[allow(unused)]
            pub fn [<set_ $field>](&mut self, val: $type) {
                self.$field = val;
            }
        }
    };
}
pub use impl_set;

/// Implements `with` for a field
#[macro_export]
macro_rules! impl_with {
    ($field:ident, $type:ty) => {
        paste::paste! {
            #[allow(unused)]
            pub fn [<with_ $field>](mut self, val: $type) -> Self {
                self.$field = val;
                self
            }
        }
    };
}
pub use impl_with;

/// Implements `get`, `set` and `with` for a field
#[macro_export]
macro_rules! impl_get_set_with {
    ($field:ident, $type:ty) => {
        impl_get!($field, $type);
        impl_set!($field, $type);
        impl_with!($field, $type);
    };
}
pub use impl_get_set_with;

/// Implements `get`, `set` and `with` for a field that cannot be copied and must be cloned
#[macro_export]
macro_rules! impl_get_set_with_cloned {
    ($field:ident, $type:ty) => {
        paste::paste! {
            #[allow(unused)]
            pub fn [<get_ $field>](&self) -> $type {
                self.$field.clone()
            }

            #[allow(unused)]
            pub fn [<set_ $field>](&mut self, val: $type) {
                self.$field = val;
            }

            #[allow(unused)]
            pub fn [<with_ $field>](mut self, val: $type) -> Self {
                self.$field = val;
                self
            }
        }
    };
}
pub use impl_get_set_with_cloned;

/// For including a tab-activated debug panel for a give resource
#[macro_export]
macro_rules! debug_resource {
    ($app:expr, $resource:ty) => {{
        $app.add_plugins(
            ResourceInspectorPlugin::<$resource>::new()
                .run_if(input_toggle_active(false, KeyCode::Tab)),
        );
    }};
}
pub use debug_resource;
