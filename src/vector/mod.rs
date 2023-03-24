use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::vertex::VertexPosition;

pub trait VecComponent:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Clone + Copy + Div
{
    // // todo make this more then just hex
    // const SIN_LOOKUP: [Self; 6];
    // // todo make this more then just hex
    // const COS_LOOKUP: [Self; 6];
}

#[derive(Clone, Copy, PartialEq)]
pub struct RVec3<T: VecComponent> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: VecComponent> RVec3<T> {
    pub fn new(x: T, y: T, z: T) -> RVec3<T> {
        RVec3 { x, y, z }
    }
}

impl<T: VecComponent + Debug> Debug for RVec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "RVec3({:?}, {:?}, {:?})",
            self.x, self.y, self.z
        ))
    }
}

impl<T: VecComponent + Display> Display for RVec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("RVec3({}, {}, {})", self.x, self.y, self.z))
    }
}

impl<T: VecComponent + Default> Default for RVec3<T> {
    fn default() -> Self {
        RVec3 {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

impl<T: VecComponent> Add for RVec3<T> {
    type Output = RVec3<T>;
    fn add(self, rhs: RVec3<T>) -> Self::Output {
        RVec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: VecComponent + AddAssign<T>> AddAssign<RVec3<T>> for RVec3<T> {
    fn add_assign(&mut self, rhs: RVec3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: VecComponent + MulAssign> Mul<T> for RVec3<T> {
    type Output = RVec3<T>;
    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: VecComponent + MulAssign<T>> MulAssign<T> for RVec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: VecComponent + SubAssign<T>> SubAssign for RVec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: VecComponent + DivAssign<T>> DivAssign<T> for RVec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: VecComponent + Hash> Hash for RVec3<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl<T: VertexPosition> RVec3<T> {
    pub fn to_f32x3(&self) -> [f32; 3] {
        [self.x.to_f32(), self.y.to_f32(), self.z.to_f32()]
    }
}

impl<
        T: Clone
            + Copy
            + Sub<Output = Self>
            + Mul<Output = Self>
            + Div
            + Add<Output = Self>
            + PartialEq,
    > VecComponent for T
{
}
