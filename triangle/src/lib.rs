extern crate num;
pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Copy + PartialOrd + std::ops::Add + num::Zero,
{
    pub fn build(sides: [T; 3]) -> Result<Triangle<T>, &'static str> {
        if sides[0] == num::zero() || sides[1] == num::zero() || sides[2] == num::zero() {
            return Result::Err("some side equals zero");
        }

        let max_side = if sides[0] > sides[1] {
            if sides[0] > sides[2] {
                sides[0]
            } else {
                sides[2]
            }
        } else {
            if sides[1] > sides[2] {
                sides[1]
            } else {
                sides[2]
            }
        };

        let sum = sides[0] + sides[1] + sides[2];
        if max_side + max_side >= sum {
            return Result::Err("Max side >= max sum");
        }

        Result::Ok(Triangle { sides })
    }
    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles() && !self.is_equilateral()
    }

    pub fn is_isosceles(&self) -> bool {
        if self.is_equilateral() {
            return false;
        }

        self.sides[0] == self.sides[1] || self.sides[0] == self.sides[2] ||
            self.sides[1] == self.sides[2]
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[0] == self.sides[2]
    }
}
