use std::{
    f64::consts::PI,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// A 2D point
#[derive(Clone, Copy, Debug)]
pub struct Point {
    /// The x-coordinate
    pub x: f64,
    /// The y-coordinate
    pub y: f64,
}

impl Point {
    /// Creates a new point
    ///
    /// # Parameters
    ///
    /// x: The x-coordinate
    ///
    /// y: The y-coordinate
    pub fn new(x: f64, y: f64) -> Self {
        return Self { x, y };
    }

    /// Calculates the norm squared of the point
    pub fn norm_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y;
    }

    /// Calculates the norm of the point
    pub fn norm(&self) -> f64 {
        return self.norm_squared().sqrt();
    }

    /// Retrieves the data for the gpu
    pub fn get_data(&self) -> [f32; 2] {
        return [self.x as f32, self.y as f32];
    }

    /// Converts it to a size
    pub fn to_size(&self) -> Size {
        return Size::new(self.x, self.y);
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        return Self::Output::new(-self.x, -self.y);
    }
}

impl Neg for &Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        return Self::Output::new(-self.x, -self.y);
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Self::Output { x, y };
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Self::Output { x, y };
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Self::Output { x, y };
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Self::Output { x, y };
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        return Self::Output { x, y };
    }
}

impl Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        return Self::Output { x, y };
    }
}

impl Sub<Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        return Self::Output { x, y };
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        return Self::Output { x, y };
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;

        return Self::Output { x, y };
    }
}

impl Mul<&f64> for Point {
    type Output = Point;

    fn mul(self, rhs: &f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;

        return Self::Output { x, y };
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;

        return Self::Output { x, y };
    }
}

impl Mul<&f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: &f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;

        return Self::Output { x, y };
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;

        return Self::Output { x, y };
    }
}

impl Div<&f64> for Point {
    type Output = Point;

    fn div(self, rhs: &f64) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;

        return Self::Output { x, y };
    }
}

impl Div<f64> for &Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;

        return Self::Output { x, y };
    }
}

impl Div<&f64> for &Point {
    type Output = Point;

    fn div(self, rhs: &f64) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;

        return Self::Output { x, y };
    }
}

impl Mul<Point> for Point {
    type Output = f64;

    fn mul(self, rhs: Point) -> Self::Output {
        return self.x * rhs.x + self.y * rhs.y;
    }
}

impl Mul<&Point> for Point {
    type Output = f64;

    fn mul(self, rhs: &Point) -> Self::Output {
        return self.x * rhs.x + self.y * rhs.y;
    }
}

impl Mul<Point> for &Point {
    type Output = f64;

    fn mul(self, rhs: Point) -> Self::Output {
        return self.x * rhs.x + self.y * rhs.y;
    }
}

impl Mul<&Point> for &Point {
    type Output = f64;

    fn mul(self, rhs: &Point) -> Self::Output {
        return self.x * rhs.x + self.y * rhs.y;
    }
}

/// A 2D size of width and height which are both non-negative
#[derive(Clone, Copy, Debug)]
pub struct Size {
    /// The width
    pub w: f64,
    /// The height
    pub h: f64,
}

impl Size {
    /// Creates a new size, if any of width or height are negative their signs are flipped
    ///
    /// # Parameters
    ///
    /// w: The width
    ///
    /// h: The height
    pub fn new(w: f64, h: f64) -> Self {
        let use_w = if w < 0.0 { -w } else { w };
        let use_h = if h < 0.0 { -h } else { h };

        return Self { w: use_w, h: use_h };
    }
}

impl Mul<f64> for Size {
    type Output = Size;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs = if rhs < 0.0 { -rhs } else { rhs };
        let w = self.w * rhs;
        let h = self.h * rhs;
        return Self::Output { w, h };
    }
}

impl Mul<&f64> for Size {
    type Output = Size;

    fn mul(self, rhs: &f64) -> Self::Output {
        let rhs = if *rhs < 0.0 { -*rhs } else { *rhs };
        let w = self.w * rhs;
        let h = self.h * rhs;
        return Self::Output { w, h };
    }
}

impl Mul<f64> for &Size {
    type Output = Size;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs = if rhs < 0.0 { -rhs } else { rhs };
        let w = self.w * rhs;
        let h = self.h * rhs;
        return Self::Output { w, h };
    }
}

impl Mul<&f64> for &Size {
    type Output = Size;

    fn mul(self, rhs: &f64) -> Self::Output {
        let rhs = if *rhs < 0.0 { -*rhs } else { *rhs };
        let w = self.w * rhs;
        let h = self.h * rhs;
        return Self::Output { w, h };
    }
}

impl Add<Size> for Size {
    type Output = Size;

    fn add(self, rhs: Size) -> Self::Output {
        let w = self.w + rhs.w;
        let h = self.h + rhs.h;

        return Self::Output { w, h };
    }
}

impl Add<&Size> for Size {
    type Output = Size;

    fn add(self, rhs: &Size) -> Self::Output {
        let w = self.w + rhs.w;
        let h = self.h + rhs.h;

        return Self::Output { w, h };
    }
}

impl Add<Size> for &Size {
    type Output = Size;

    fn add(self, rhs: Size) -> Self::Output {
        let w = self.w + rhs.w;
        let h = self.h + rhs.h;

        return Self::Output { w, h };
    }
}

impl Add<&Size> for &Size {
    type Output = Size;

    fn add(self, rhs: &Size) -> Self::Output {
        let w = self.w + rhs.w;
        let h = self.h + rhs.h;

        return Self::Output { w, h };
    }
}

/// A 2D index
#[derive(Clone, Copy, Debug)]
pub struct Index {
    /// The x-index
    pub x: i64,
    /// The y-index
    pub y: i64,
}

impl Index {
    /// Creates a new index
    ///
    /// # Parameters
    ///
    /// x: The x-index
    ///
    /// y: The y-index
    pub fn new(x: i64, y: i64) -> Self {
        return Self { x, y };
    }
}

impl Add<Index> for Index {
    type Output = Index;

    fn add(self, rhs: Index) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Self::Output { x, y };
    }
}

impl Add<&Index> for Index {
    type Output = Index;

    fn add(self, rhs: &Index) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Self::Output { x, y };
    }
}

impl Add<Index> for &Index {
    type Output = Index;

    fn add(self, rhs: Index) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Self::Output { x, y };
    }
}

impl Add<&Index> for &Index {
    type Output = Index;

    fn add(self, rhs: &Index) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Self::Output { x, y };
    }
}

/// Defines a view of the map
#[derive(Clone, Copy, Debug)]
pub struct View {
    /// The center of the rectangle
    center: Point,
    /// The size of the rectangle
    size: Size,
}

impl View {
    /// Creates a new view
    ///
    /// # Parameters
    ///
    /// center: The center of the rectangle
    ///
    /// size: The size of the rectangle
    pub fn new(center: &Point, size: &Size) -> Self {
        return Self {
            center: *center,
            size: *size,
        };
    }

    /// Retrieves the center
    pub fn get_center(&self) -> &Point {
        return &self.center;
    }

    /// Retrieves the size
    pub fn get_size(&self) -> &Size {
        return &self.size;
    }

    pub fn contains(&self, other: &View) -> bool {
        return self.center.x - self.size.w * 0.5 <= other.center.x - other.size.w * 0.5
            && self.center.y - self.size.h * 0.5 <= other.center.y - other.size.h * 0.5
            && self.center.x + self.size.w * 0.5 >= other.center.x + other.size.w * 0.5
            && self.center.y + self.size.h * 0.5 >= other.center.y + other.size.h * 0.5;
    }
}

/// Defines a 2x2 matrix
#[derive(Clone, Copy, Debug)]
pub struct Matrix {
    /// The values of the matrix
    pub values: [[f64; 2]; 2],
}

impl Matrix {
    /// Creates a new matrix
    ///
    /// # Parameters
    ///
    /// values: The values of the matrix, first index is row, second index is column
    pub fn new(values: &[[f64; 2]; 2]) -> Self {
        return Self { values: *values };
    }

    /// Transposes the matrix
    pub fn transpose(&self) -> Self {
        return Self::new(&[
            [self.values[0][0], self.values[1][0]],
            [self.values[0][1], self.values[1][1]],
        ]);
    }

    /// Inverts the matrix
    ///
    /// # Panics
    ///
    /// In debug mode it panics if the determinant is 0 (it is not invertible)
    pub fn inv(&self) -> Self {
        // Calculate determinant
        let d = self.values[0][0] * self.values[1][1] - self.values[0][1] * self.values[1][0];

        // Make sure it is not invalid
        if cfg!(debug_assertions) && d == 0.0 {
            panic!("The matrix is not invertible: {:?}", self);
        }

        // Calculate inverse
        return Self::new(&[
            [self.values[1][1] / d, -self.values[0][1] / d],
            [-self.values[1][0] / d, self.values[0][0] / d],
        ]);
    }

    /// Calculates the determinant
    pub fn det(&self) -> f64 {
        return self.values[0][0] * self.values[1][1] - self.values[0][1] * self.values[1][0];
    }

    /// Calculates the two eigenvalues sorting them from largest to smallest
    pub fn eigenvalues(&self) -> [f64; 2] {
        let d = (self.values[0][0] + self.values[1][1]) * (self.values[0][0] + self.values[1][1])
            - 4.0 * self.det();

        // Make sure it is not invalid
        if cfg!(debug_assertions) && d < 0.0 {
            panic!("The matrix is singular: {:?}", self);
        }

        let sqrt_d = d.sqrt();

        return [
            0.5 * ((self.values[0][0] + self.values[1][1]) + sqrt_d),
            0.5 * ((self.values[0][0] + self.values[1][1]) - sqrt_d),
        ];
    }

    /// Retrieves the data for the gpu
    pub fn get_data(&self) -> [f32; 4] {
        return [
            self.values[0][0] as f32,
            self.values[1][0] as f32,
            self.values[0][1] as f32,
            self.values[1][1] as f32,
        ];
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        return Self::Output::new(&[
            [
                self.values[0][0] * rhs.values[0][0] + self.values[0][1] * rhs.values[1][0],
                self.values[0][0] * rhs.values[0][1] + self.values[0][1] * rhs.values[1][1],
            ],
            [
                self.values[1][0] * rhs.values[0][0] + self.values[1][1] * rhs.values[1][0],
                self.values[1][0] * rhs.values[0][1] + self.values[1][1] * rhs.values[1][1],
            ],
        ]);
    }
}

impl Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        return Self::Output::new(&[
            [-self.values[0][0], -self.values[0][1]],
            [-self.values[1][0], -self.values[1][1]],
        ]);
    }
}

impl Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        return Self::Output::new(&[
            [
                self.values[0][0] + rhs.values[0][0],
                self.values[0][1] + rhs.values[0][1],
            ],
            [
                self.values[1][0] + rhs.values[1][0],
                self.values[1][1] + rhs.values[1][1],
            ],
        ]);
    }
}

impl Sub<Matrix> for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Matrix) -> Self::Output {
        return Self::Output::new(&[
            [
                self.values[0][0] - rhs.values[0][0],
                self.values[0][1] - rhs.values[0][1],
            ],
            [
                self.values[1][0] - rhs.values[1][0],
                self.values[1][1] - rhs.values[1][1],
            ],
        ]);
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        return Self::Output::new(
            self.values[0][0] * rhs.x + self.values[0][1] * rhs.y,
            self.values[1][0] * rhs.x + self.values[1][1] * rhs.y,
        );
    }
}

impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        return Self::Output::new(&[
            [rhs * self.values[0][0], rhs * self.values[0][1]],
            [rhs * self.values[1][0], rhs * self.values[1][1]],
        ]);
    }
}

/// A 2D transform which acts on Point types, including rotation, scaling and translation.
///
/// The operation is y = r * (x - c) where
///
/// y: The output point
///
/// x: The input point
///
/// c: The center point
///
/// r: The 2x2 center_transform matrix
#[derive(Copy, Clone, Debug)]
pub struct Transform2D {
    /// The transform to apply relative to the center
    pub center_transform: Matrix,
    /// The center of the coordinate system
    pub center: Point,
}

impl Transform2D {
    /// Creates the identity operation
    pub fn identity() -> Self {
        let center_transform = Matrix::new(&[[1.0, 0.0], [0.0, 1.0]]);
        let center = Point::new(0.0, 0.0);

        return Self {
            center_transform,
            center,
        };
    }

    /// Rotate around origo
    ///
    /// # Parameters
    ///
    /// angle: The angle to rotate
    pub fn rotation(angle: f64) -> Self {
        let center_transform =
            Matrix::new(&[[angle.cos(), -angle.sin()], [angle.sin(), angle.cos()]]);
        let center = Point::new(0.0, 0.0);

        return Self {
            center_transform,
            center,
        };
    }

    /// Rotate around center
    ///
    /// # Parameters
    ///
    /// angle: The angle to rotate
    ///
    /// rotation_center: The center of the rotation
    pub fn rotation_at(angle: f64, rotation_center: &Point) -> Self {
        let center_transform =
            Matrix::new(&[[angle.cos(), -angle.sin()], [angle.sin(), angle.cos()]]);
        let center = *rotation_center - center_transform.inv() * *rotation_center;

        return Self {
            center_transform,
            center,
        };
    }

    /// Scale at origo
    ///
    /// # Parameters
    ///
    /// scale: The ratio to scale x and y with
    pub fn scale(scale: &Point) -> Self {
        let center_transform = Matrix::new(&[[scale.x, 0.0], [0.0, scale.y]]);
        let center = Point::new(0.0, 0.0);

        return Self {
            center_transform,
            center,
        };
    }

    /// Scale at center
    ///
    /// # Parameters
    ///
    /// scale: The ratio to scale x and y with
    ///
    /// center: The center of the scaling
    pub fn scale_at(scale: &Point, scale_center: &Point) -> Self {
        let center_transform = Matrix::new(&[[scale.x, 0.0], [0.0, scale.y]]);
        let center = *scale_center - center_transform.inv() * *scale_center;

        return Self {
            center_transform,
            center,
        };
    }

    /// Translates a point
    ///
    /// # Parameters
    ///
    /// offset: The amount to translate
    pub fn translate(offset: &Point) -> Self {
        let center_transform = Matrix::new(&[[1.0, 0.0], [0.0, 1.0]]);
        let center = *offset;

        return Self {
            center_transform,
            center,
        };
    }

    /// Retrieves the inverse transform
    pub fn inv(&self) -> Self {
        let center_transform = self.center_transform.inv();
        let center = -self.center_transform * self.center;

        return Self {
            center_transform,
            center,
        };
    }

    /// Retrieves the offset
    pub fn get_center(&self) -> &Point {
        return &self.center;
    }

    /// Retrieves the center transform
    pub fn get_center_transform(&self) -> &Matrix {
        return &self.center_transform;
    }

    /// Retrieves the data for the offset
    pub fn get_data_offset(&self) -> [f32; 2] {
        return self.center.get_data();
    }

    /// Retrieves the data for the center transform
    pub fn get_data_center_transform(&self) -> [f32; 4] {
        return self.center_transform.get_data();
    }
}

impl Mul<Transform2D> for Transform2D {
    type Output = Transform2D;

    /// t2 * t1 * x = r2 * (r1 * (x - c1) - c2) = r2 * r1 * (x - c1 - r1^-1 * c2)
    fn mul(self, rhs: Transform2D) -> Self::Output {
        let center_transform = self.center_transform * rhs.center_transform;
        let center = rhs.center + rhs.center_transform.inv() * self.center;

        return Self::Output {
            center_transform,
            center,
        };
    }
}

impl Mul<&Transform2D> for Transform2D {
    type Output = Transform2D;

    /// t2 * t1 * x = r2 * (r1 * (x - c1) - c2) = r2 * r1 * (x - c1 - r1^-1 * c2)
    fn mul(self, rhs: &Transform2D) -> Self::Output {
        let center_transform = self.center_transform * rhs.center_transform;
        let center = rhs.center + rhs.center_transform.inv() * self.center;

        return Self::Output {
            center_transform,
            center,
        };
    }
}

impl Mul<Transform2D> for &Transform2D {
    type Output = Transform2D;

    /// t2 * t1 * x = r2 * (r1 * (x - c1) - c2) = r2 * r1 * (x - c1 - r1^-1 * c2)
    fn mul(self, rhs: Transform2D) -> Self::Output {
        let center_transform = self.center_transform * rhs.center_transform;
        let center = rhs.center + rhs.center_transform.inv() * self.center;

        return Self::Output {
            center_transform,
            center,
        };
    }
}

impl Mul<&Transform2D> for &Transform2D {
    type Output = Transform2D;

    /// t2 * t1 * x = r2 * (r1 * (x - c1) - c2) = r2 * r1 * (x - c1 - r1^-1 * c2)
    fn mul(self, rhs: &Transform2D) -> Self::Output {
        let center_transform = self.center_transform * rhs.center_transform;
        let center = rhs.center + rhs.center_transform.inv() * self.center;

        return Self::Output {
            center_transform,
            center,
        };
    }
}

impl Mul<Point> for Transform2D {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        return self.center_transform * (rhs - self.center);
    }
}

impl Mul<&Point> for Transform2D {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        return self.center_transform * (rhs - self.center);
    }
}

impl Mul<Point> for &Transform2D {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        return self.center_transform * (rhs - self.center);
    }
}

impl Mul<&Point> for &Transform2D {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        return self.center_transform * (rhs - self.center);
    }
}

/// Describes a single 2D Gaussian
#[derive(Clone, Copy, Debug)]
pub struct Gaussian {
    /// The norm of the Gaussian
    pub norm: f64,
    /// The mean of the Gaussian
    pub mean: Point,
    /// The inverse covariance matrix divided by 2
    pub matrix: Matrix,
}

impl Gaussian {
    /// Constructs a new Gaussian
    ///
    /// # Parameters
    ///
    /// norm: The normalization of the Gaussian
    ///
    /// mean: The mean value of the Gaussian
    ///
    /// cov: The covariance matrix of the Gaussian
    pub fn new(norm: f64, mean: Point, cov: Matrix) -> Self {
        return Self {
            norm,
            mean,
            matrix: cov.inv() * 0.5,
        };
    }

    /// Evaluates the Gaussian in a number of locations
    ///
    /// # Parameters
    ///
    /// points: The points to evaluate at
    pub fn evaluate(&self, offset: &Point, points: &[Point]) -> Vec<f64> {
        let coeff = self.norm * self.matrix.det().sqrt() / PI;

        return points
            .iter()
            .map(|point| {
                let rel_point = (point + offset) - self.mean;
                let exponent = -rel_point * (self.matrix * rel_point);
                return coeff * exponent.exp();
            })
            .collect();
    }

    /// Retrieves the covariance matrix
    pub fn get_covariance(&self) -> Matrix {
        return (self.matrix * 2.0).inv();
    }
}
