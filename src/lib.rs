#![allow(dead_code)]

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
    value: f32,
    nets: Nets,
}

struct Graph
{
    nodes: Vec<Element>,
}
