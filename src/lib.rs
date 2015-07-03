#![feature(iter_arith)]
#![allow(dead_code)]

mod linalg;
mod fileformat;


type Net = u32;
type Nets = (Net,Net);

#[derive(Debug,Eq,PartialEq)]
pub enum ElementType
{
    Resistor,
    ConstantVoltage,
    ConstantCurrent,
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
