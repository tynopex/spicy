use std::cmp;
use super::{Graph,ElementType};
use super::linalg::{Matrix,Vector};


fn build_eqns(gr: &Graph) -> (Matrix,Vector)
{
    let m = gr.nodes.len();
    let n = count_nets(gr);

    let mut a = Matrix::new(m + n, n + m);
    let mut b = Vector::new(m + n);

    for (i,elem) in gr.nodes.iter().enumerate()
    {
        let na = elem.nets.0 as usize;
        let nb = elem.nets.1 as usize;

        a.data[m+na][n+i] += 1.0;
        a.data[m+nb][n+i] -= 1.0;

        if elem.kind == ElementType::ConstantVoltage
        {
            a.data[i][na] -= 1.0;
            a.data[i][nb] += 1.0;

            b.data[i] = elem.value;
        }

        if elem.kind == ElementType::ConstantCurrent
        {
            a.data[i][n+i] = 1.0;

            b.data[i] = elem.value;
        }

        if elem.kind == ElementType::Resistor
        {
            a.data[i][na] -= 1.0;
            a.data[i][nb] += 1.0;

            a.data[i][n+i] = -elem.value;
        }
    }

    // Replace node 0 current equation with V_0 = 0 reference
    a.data[m] = vec![0.0; n + m];
    a.data[m][0] = 1.0;

    (a,b)
}

fn count_nets(gr: &Graph) -> usize
{
    // Find highest numbered node
    let max = gr.nodes.iter()
                      .map(|el| cmp::max(el.nets.0, el.nets.1))
                      .max()
                      .unwrap_or(0) as usize;

    max + 1
}
