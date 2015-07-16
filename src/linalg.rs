use std::fmt;


pub type Float = f64;

pub struct Matrix
{
    pub data: Vec<Vec<Float>>,
}

pub struct Vector
{
    pub data: Vec<Float>,
}

impl fmt::Display for Matrix
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for row in &self.data
        {
            for el in row
            {
                try!(write!(f, "{:9.3} ", el));
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
        for el in &self.data
        {
            try!(write!(f, "{:9.3} ", el));
        }
        try!(write!(f, "\n"));

        Ok(())
    }
}

pub fn gaussian_elimination(a: &Matrix, b: &Vector) -> Vector
{
    type AugRow = (Vec<Float>, Float);
    type AugRowRef<'a> = &'a mut AugRow;

    fn leading_zeros(row: &AugRow) -> usize
    {
        row.0.iter()
             .cloned()
             .take_while(|x| *x == 0.0)
             .count()
    }

 // let n = a.data.len();
    let m = a.data[0].len();

    let mut x = vec![0.0; m];

    // Augmented matrix
    let mut aug: Vec<AugRow> =
        a.data.iter()
              .cloned()
              .zip(b.data.iter().cloned())
              .collect();

    // Row-echelon form
    for j in 0..m
    {
        let mut rows =
            aug.iter_mut()
               .filter(|x| leading_zeros(x) == j);

        if let Some(pivot) = rows.next()
        {
            for row in rows
            {
                let scale = pivot.0[j] / row.0[j];

                for (pp,rr) in pivot.0.iter()
                                      .cloned()
                                      .zip(&mut row.0)
                {
                    (*rr) *= scale;
                    (*rr) -= pp;
                }

                row.1 *= scale;
                row.1 -= pivot.1;
            }
        }
        else
        {
            panic!("Unconstrained variable");
        }
    }

    // Sort rows
    aug.sort_by(|lhs,rhs| {
        let l_zeros = leading_zeros(lhs);
        let r_zeros = leading_zeros(rhs);

        l_zeros.cmp(&r_zeros)
        });

    // Forward substitution
    for j in (0..m).rev()
    {
        let mut sum = aug[j].1;

        for k in (j+1)..m
        {
            sum -= aug[j].0[k] * x[k];
        }

        x[j] = sum / aug[j].0[j];
    }

    Vector { data: x }
}

fn euclidean_dist(a: &[Float], b: &[Float]) -> Float
{
    a.iter()
     .zip(b)
     .map(|(xa,xb)| (xa-xb)*(xa-xb))
     .fold(0.0, |s,x| s + x)
     .sqrt()
}

fn inner_product(a: &[Float], b: &[Float]) -> Float
{
    a.iter()
     .zip(b)
     .map(|(xa,xb)| xa*xb)
     .fold(0.0, |s,x| s + x)
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

    for (el,row) in b.iter_mut().zip(a)
    {
        *el = row.iter()
                 .zip(x)
                 .map(|e| e.0 * e.1)
                 .fold(0.0, |s,x| s + x)
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
