/// Determines the range of values for rand
pub type RandomType = isize;

/// Determines the Point memory size
pub type PointType = isize;

/// This is an arbitrary max row value just to compare against
pub const MIN_ROW: PointType = 0;
pub const MAX_ROW: PointType = 50;
/// This is an arbitrary max col value just to compare against
pub const MIN_COL: PointType = 0;
pub const MAX_COL: PointType = 50;

/// How many random points to generate
pub const MAX_RANDS: usize = 1_000_000;

pub struct Point {
    pub row: PointType,
    pub col: PointType,
}

/// Generate a bunch of random Points
pub fn gen_random_points() -> Vec<Point> {
    (0..MAX_RANDS)
        .map(|_| {
            let row = rand::random::<RandomType>();
            let col = rand::random::<RandomType>();
            Point { row, col }
        })
        .collect()
}
