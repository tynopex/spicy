#![feature(iter_arith)]
#![allow(dead_code)]

pub mod linalg;
pub mod fileformat;
pub mod solve;


type Net = u32;
type Nets = (Net,Net);

#[derive(Debug,Eq,PartialEq)]
pub enum ElementType
{
    Resistor,
    ConstantVoltage,
    ConstantCurrent,
    DependentVoltage,
    DependentCurrent,
}

#[derive(Debug)]
pub struct Element
{
    kind: ElementType,
    value: linalg::Float,
    nets: Nets,
}

pub struct Graph
{
    nodes: Vec<Element>,
}

pub struct SimResult
{
    pub v: Vec<linalg::Float>,
    pub i: Vec<linalg::Float>,
}


#[test]
fn test_solve()
{
    let gr = fileformat::load_spicy("simple.spicy");
    let sr = solve::solve(gr.as_ref().unwrap());

    assert_eq!(sr.v, vec![0.0, 5.0, 10.0]);
    assert_eq!(sr.i, vec![0.1, 0.1, -0.1]);
}
