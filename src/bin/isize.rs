use std::time::Instant;

use range_test::*;

/// Generate a bunch of random Points
fn gen_random_points() -> Vec<Point> {
    (0..MAX_RANDS)
        .map(|_| {
            let row = rand::random::<RandomType>();
            let col = rand::random::<RandomType>();
            Point { row, col }
        })
        .collect()
}

/// Determine if a point is within a boundary by using
/// Range semantics.
fn cmp_by_range(point: &Point) -> bool {
    (MIN_ROW..MAX_ROW).contains(&point.row) && (MIN_ROW..MAX_COL).contains(&point.col)
}

/// Determine if a point is withing a boundary by using
/// classic if statements.  Since we are using
fn cmp_by_ifs(point: &Point) -> bool {
    point.row >= MIN_ROW && point.row < MAX_ROW && point.col >= MIN_COL && point.col < MAX_COL
}

/// Loop through all points using `cmp_by_ifs`
fn time_ifs(points: &[Point]) {
    let now = Instant::now();
    let mut fails = 0;
    for point in points {
        if !cmp_by_ifs(point) {
            fails += 1;
        }
    }
    println!("By If statements: {} - {:.2?}", fails, now.elapsed());
}

/// Loop through all points using `cmp_by_range`
fn time_range(points: &[Point]) {
    let now = Instant::now();
    let mut fails = 0;
    for point in points {
        if !cmp_by_range(point) {
            fails += 1;
        }
    }
    println!("By ranges: {} - {:.2?}", fails, now.elapsed());
}

fn main() {
    let points = gen_random_points();
    time_ifs(&points);
    time_range(&points);
}
