#![feature(iter_arith)]
#![allow(dead_code)]

mod linalg;
mod fileformat;


type Net = u32;
type Nets = (Net,Net);

#[derive(Debug)]
enum ElementType
{
    Resistor,
    ConstantVoltage,
    ConstantCurrent,
}

#[derive(Debug)]
struct Element
{
    kind: ElementType,
    value: linalg::Float,
    nets: Nets,
}

struct Graph
{
    nodes: Vec<Element>,
}
