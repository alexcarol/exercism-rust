use std::cmp;

pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Result<Triangle, bool> {
        if !sides.into_iter().fold(
            true,
            |acc, length| acc && *length != 0,
        )
        {
            return Result::Err(false);
        }

        let max_side = sides.into_iter().fold(
            0,
            |acc, length| cmp::max(acc, *length),
        );
        if max_side * 2 >= sides[0] + sides[1] + sides[2] {
            return Result::Err(false);
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
