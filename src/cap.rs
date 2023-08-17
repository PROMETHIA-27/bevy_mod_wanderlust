use bevy::math::*;

/// Sign independent capping.
pub trait Cap: Sized {
    /// Sign independent maximum.
    ///
    /// max > 0.0: [-inf, max]
    /// max < 0.0: [max, inf]
    fn signed_max(self, max: Self) -> Self;
    /// Sign independent minimum.
    ///
    /// min > 0.0: [min, inf]
    /// min < 0.0: [-inf, min]
    fn signed_min(self, min: Self) -> Self;
}

impl Cap for f32 {
    fn signed_max(self, cap: Self) -> Self {
        if cap.is_nan() || cap == 0.0 || cap == -0.0 {
            return cap;
        }

        if cap.is_sign_negative() {
            cap.max(self)
        } else {
            cap.min(self)
        }
    }
    fn signed_min(self, cap: Self) -> Self {
        if cap.is_nan() || cap == 0.0 || cap == -0.0 {
            return self;
        }

        if cap.is_sign_negative() {
            cap.max(self)
        } else {
            cap.min(self)
        }
    }
}

impl Cap for f64 {
    fn signed_max(self, cap: Self) -> Self {
        if cap.is_nan() || cap == 0.0 || cap == -0.0 {
            return cap;
        }

        if cap.is_sign_negative() {
            cap.max(self)
        } else {
            cap.min(self)
        }
    }
    fn signed_min(self, cap: Self) -> Self {
        if cap.is_nan() || cap == 0.0 || cap == -0.0 {
            return self;
        }

        if cap.is_sign_negative() {
            cap.max(self)
        } else {
            cap.min(self)
        }
    }
}

impl Cap for Vec2 {
    fn signed_max(self, cap: Self) -> Self {
        Vec2::new(self.x.signed_max(cap.x), self.y.signed_max(cap.y))
    }
    fn signed_min(self, cap: Self) -> Self {
        Vec2::new(self.x.signed_min(cap.x), self.y.signed_min(cap.y))
    }
}

impl Cap for DVec2 {
    fn signed_max(self, cap: Self) -> Self {
        DVec2::new(self.x.signed_max(cap.x), self.y.signed_max(cap.y))
    }
    fn signed_min(self, cap: Self) -> Self {
        DVec2::new(self.x.signed_min(cap.x), self.y.signed_min(cap.y))
    }
}

impl Cap for Vec3 {
    fn signed_max(self, cap: Self) -> Self {
        Vec3::new(
            self.x.signed_max(cap.x),
            self.y.signed_max(cap.y),
            self.z.signed_max(cap.z),
        )
    }
    fn signed_min(self, cap: Self) -> Self {
        Vec3::new(
            self.x.signed_min(cap.x),
            self.y.signed_min(cap.y),
            self.z.signed_min(cap.z),
        )
    }
}

impl Cap for Vec3A {
    fn signed_max(self, cap: Self) -> Self {
        Vec3A::new(
            self.x.signed_max(cap.x),
            self.y.signed_max(cap.y),
            self.z.signed_max(cap.z),
        )
    }
    fn signed_min(self, cap: Self) -> Self {
        Vec3A::new(
            self.x.signed_min(cap.x),
            self.y.signed_min(cap.y),
            self.z.signed_min(cap.z),
        )
    }
}

impl Cap for DVec3 {
    fn signed_max(self, cap: Self) -> Self {
        DVec3::new(
            self.x.signed_max(cap.x),
            self.y.signed_max(cap.y),
            self.z.signed_max(cap.z),
        )
    }
    fn signed_min(self, cap: Self) -> Self {
        DVec3::new(
            self.x.signed_min(cap.x),
            self.y.signed_min(cap.y),
            self.z.signed_min(cap.z),
        )
    }
}

impl Cap for Vec4 {
    fn signed_max(self, cap: Self) -> Self {
        Vec4::new(
            self.x.signed_max(cap.x),
            self.y.signed_max(cap.y),
            self.z.signed_max(cap.z),
            self.w.signed_max(cap.w),
        )
    }
    fn signed_min(self, cap: Self) -> Self {
        Vec4::new(
            self.x.signed_min(cap.x),
            self.y.signed_min(cap.y),
            self.z.signed_min(cap.z),
            self.w.signed_min(cap.w),
        )
    }
}

impl Cap for DVec4 {
    fn signed_max(self, cap: Self) -> Self {
        DVec4::new(
            self.x.signed_max(cap.x),
            self.y.signed_max(cap.y),
            self.z.signed_max(cap.z),
            self.w.signed_max(cap.w),
        )
    }
    fn signed_min(self, cap: Self) -> Self {
        DVec4::new(
            self.x.signed_min(cap.x),
            self.y.signed_min(cap.y),
            self.z.signed_min(cap.z),
            self.w.signed_min(cap.w),
        )
    }
}
