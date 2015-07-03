use std::fmt;


pub type Float = f64;

pub struct Matrix
{
    pub data: Vec<Vec<Float>>,
}

pub struct Vector
{
    pub data: Vec<f64>,
}

impl fmt::Display for Matrix
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for row in self.data.iter()
        {
            for el in row
            {
                try!(write!(f, "{:9.4} ", el));
            }
            try!(write!(f, "\n"));
        }

        Ok(())
    }
}

impl fmt::Display for Vector
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for el in self.data.iter()
        {
            try!(write!(f, "{:9.4} ", el));
        }
        try!(write!(f, "\n"));

        Ok(())
    }
}

fn euclidean_dist(a: &[Float], b: &[Float]) -> Float
{
    a.iter()
     .zip(b)
     .map(|(xa,xb)| (xa-xb)*(xa-xb))
     .sum::<Float>()
     .sqrt()
}

fn inner_product(a: &[Float], b: &[Float]) -> Float
{
    a.iter()
     .zip(b)
     .map(|(xa,xb)| xa*xb)
     .sum()
}

// Gauss-Seidel solver for Ax=b
pub fn gauss_seidel(a: &Matrix, b: &Vector, eps: Float, max: usize) -> Vector
{
    let a = &a.data;
    let b = &b.data;

    let n = b.len();

    let mut x = vec![0.0; n];
    let mut x_prev = vec![0.0; n];

    for _ in 0..max
    {
        x_prev.clone_from(&x);

        for j in 0..n
        {
            x[j] += (b[j] - inner_product(&a[j], &x)) / a[j][j];
        }

        if euclidean_dist(&x_prev, &x) <= eps
        {
            break;
        }
    }

    Vector { data: x }
}

// Matrix-Vector multiply
fn mat_mul_vec(a: &Matrix, x: &Vector) -> Vector
{
    let a = &a.data;
    let x = &x.data;

    let n = a.len();
    let mut b = vec![0.0; n];

    for (el,row) in b.iter_mut().zip(a.iter())
    {
        *el = row.iter()
                 .zip(x.iter())
                 .map(|e| e.0 * e.1)
                 .sum();
    }

    Vector { data: b }
}

impl Matrix
{
    pub fn new(n: usize, m: usize) -> Matrix
    {
        Matrix { data: vec![vec![0.0; m]; n] }
    }
}

impl Vector
{
    pub fn new(n: usize) -> Vector
    {
        Vector { data: vec![0.0; n] }
    }
}
