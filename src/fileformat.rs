/*
    # Sample.spicy
    CV[3.3] 0 1
    R[10k] 0 1
    R[1k] 1 2
    R[4700] 0 2
    CC[10m] 0 2
*/

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use super::{Graph,ElementType,Element,Net};
use super::linalg::Float;


pub fn load_spicy(fname: &str) -> Option<Graph>
{
    let f = File::open(fname).unwrap();

    let mut gr = Graph { nodes: vec![] };

    for line in BufReader::new(f).lines()
    {
        let line = line.unwrap();
        let line = line.trim();

        if line.starts_with("#") || line == ""
        {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() == 4);

        let kind = match parts[0] {
            "R" => ElementType::Resistor,
            "CV" => ElementType::ConstantVoltage,
            "CC" => ElementType::ConstantCurrent,
            _ => panic!("Unknown type"),
            };

        let val = parse_value(parts[1]);

        let net_a: Net = parts[2].parse().unwrap();
        let net_b: Net = parts[3].parse().unwrap();

        let el = Element { kind: kind, value: val, nets: (net_a,net_b) };

        gr.nodes.push(el);
    }
    
    Some(gr)
}

fn parse_value(s: &str) -> Float
{
    let idx = s.rfind(char::is_alphabetic);

    let mut val: Float = match idx {
        Some(i) => &s[..i],
        None => s,
        }
        .parse()
        .unwrap();

    if let Some(i) = idx
    {
        let scale: Float = match &s[i..] {
            "p" => 1e-12,
            "n" => 1e-9,
            "u" => 1e-6,
            "m" => 1e-3,
            "k" => 1e3,
            "M" => 1e6,
            _ => panic!("Unknown size prefix"),
            };

        val *= scale;
    }

    val
}

#[test]
fn test_load()
{
    load_spicy("sample.spicy");
}
