use std::cmp;
use super::{Graph,Element,ElementType,SimResult};
use super::linalg::{Matrix,Vector,gaussian_elimination};


pub fn solve(gr: &Graph) -> SimResult
{
    let (a,b) = build_eqns(gr);

    // Solve Ax = b
    let x = gaussian_elimination(&a, &b);

    // Save results
    let n = count_nets(gr);
    let (v,i) = x.data.split_at(n);

    SimResult {
        v: v.to_vec(),
        i: i.to_vec(),
        }
}

fn build_eqns(gr: &Graph) -> (Matrix,Vector)
{
    let m = gr.nodes.len();
    let n = count_nets(gr);

    let mut a = Matrix::new(m + n, n + m);
    let mut b = Vector::new(m + n);

    let mut prev: Option<&Element> = None;

    for (i,elem) in gr.nodes.iter().enumerate()
    {
        let na = elem.nets.0 as usize;
        let nb = elem.nets.1 as usize;

        // Current flow / adjacency
        a.data[m+na][n+i] += 1.0;
        a.data[m+nb][n+i] -= 1.0;

        // Voltage source
        if elem.kind == ElementType::ConstantVoltage ||
           elem.kind == ElementType::DependentVoltage
        {
            a.data[i][na] -= 1.0;
            a.data[i][nb] += 1.0;
        }

        // Current source
        if elem.kind == ElementType::ConstantCurrent ||
           elem.kind == ElementType::DependentCurrent
        {
            a.data[i][n+i] = 1.0;
        }

        // Constant source
        if elem.kind == ElementType::ConstantVoltage ||
           elem.kind == ElementType::ConstantCurrent
        {
            b.data[i] = elem.value;
        }

        // Dependent source
        if elem.kind == ElementType::DependentVoltage ||
           elem.kind == ElementType::DependentCurrent
        {
            if let Some(ref_elem) = prev
            {
                let ref_na = ref_elem.nets.0 as usize;
                let ref_nb = ref_elem.nets.1 as usize;

                // Reference should be dummy
                assert!(ref_elem.value == 0.0);

                match ref_elem.kind
                {
                    ElementType::ConstantCurrent =>
                    {
                        // Reference is open-circuit voltage
                        a.data[i][ref_na] += elem.value;
                        a.data[i][ref_nb] -= elem.value;
                    },

                    ElementType::ConstantVoltage =>
                    {
                        // Reference is short-circuit current
                        a.data[i][n+i-1] -= elem.value;
                    },

                    _ => panic!("Invalid dependent source reference")
                }
            }
            else
            {
                panic!("Missing dependent source reference");
            }
        }

        if elem.kind == ElementType::Resistor
        {
            a.data[i][na] -= 1.0;
            a.data[i][nb] += 1.0;

            a.data[i][n+i] = elem.value;
        }

        // Remember previous element
        prev = Some(&elem);
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
