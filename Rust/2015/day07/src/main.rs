use std::fs;

pub type WireValue = u16;

fn main() {
    let input = fs::read_to_string("../inputs/07-input").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wire(u16);

impl Wire {
    const CHARACTER_RANGE_LENGTH: u16 = 'z' as u16 - 'a' as u16 + 1;

    const MAX_INDEX: usize = (Self::CHARACTER_RANGE_LENGTH * Self::CHARACTER_RANGE_LENGTH
        + Self::CHARACTER_RANGE_LENGTH) as usize;

    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

impl std::fmt::Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.0 / Self::CHARACTER_RANGE_LENGTH;
        if value != 0 {
            let c = char::from_u32(value as u32 + 'a' as u32 - 1).unwrap();
            write!(f, "{}", c)?;
        }
        let c = char::from_u32((self.0 % Self::CHARACTER_RANGE_LENGTH) as u32 + 'a' as u32 - 1)
            .unwrap();
        write!(f, "{}", c)?;

        Ok(())
    }
}

impl<S: AsRef<str>> From<S> for Wire {
    fn from(s: S) -> Self {
        let s = s.as_ref();

        let mut chars = s.chars();
        let value = match chars.next() {
            Some(c) if c.is_ascii_lowercase() => c as u16 - 'a' as u16 + 1,
            _ => panic!("invalid input: invalid wire name: '{}'", s),
        };
        let value = match chars.next() {
            Some(c) if c.is_ascii_lowercase() => {
                (value * Self::CHARACTER_RANGE_LENGTH) + (c as u16 - 'a' as u16 + 1)
            }
            Some(_) => panic!("invalid input: invalid wire name: '{}'", s),
            None => value,
        };
        if chars.next().is_some() {
            panic!("invalid input: invalid wire name: '{}'", s);
        }

        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Literal(WireValue),
    Wire(Wire),
    Not(Wire),
    And(String, String),
    Or(Wire, Wire),
    LeftShift(Wire, u8),
    RightShift(Wire, u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Connection {
    expression: Expression,
    out_wire: Wire,
}

fn parse_input<S: AsRef<str>>(input: S) -> Vec<Connection> {
    let input = input.as_ref();

    let mut connections = Vec::new();
    for line in input.lines() {
        let (expression, out_wire) = line.split_once(" -> ").expect("invalid input");
        let out_wire = Wire::from(out_wire);
        let expression = if expression.starts_with("NOT") {
            let lhs = expression["NOT ".len()..].to_owned();
            Expression::Not(Wire::from(lhs))
        } else if let Some((a, b)) = expression.split_once(" AND ") {
            Expression::And(a.to_owned(), b.to_owned())
        } else if let Some((a, b)) = expression.split_once(" OR ") {
            Expression::Or(Wire::from(a), Wire::from(b))
        } else if let Some((a, b)) = expression.split_once(" LSHIFT ") {
            Expression::LeftShift(
                Wire::from(a),
                b.parse::<u8>()
                    .expect(format!("invalid input: invalid lshift offset '{}'", &b).as_str()),
            )
        } else if let Some((a, b)) = expression.split_once(" RSHIFT ") {
            Expression::RightShift(
                Wire::from(a),
                b.parse::<u8>()
                    .expect(format!("invalid input: invalid rshift offset '{}'", &b).as_str()),
            )
        } else {
            match expression.parse::<WireValue>() {
                Ok(x) => Expression::Literal(x),
                Err(_) => Expression::Wire(Wire::from(expression)),
            }
        };
        let connection = Connection {
            expression,
            out_wire,
        };
        connections.push(connection);
    }

    return connections;
}

fn solve<S: AsRef<str>>(input: S) -> (WireValue, WireValue) {
    let connections = parse_input(&input);
    let connections2 = connections.clone();
    let mut wires: Vec<Option<WireValue>> = vec![None; Wire::MAX_INDEX];
    simulate(&mut wires, connections);

    let part1 =
        wires[Wire::from("a").index()].expect("invalid input: wire 'a' has no value (part 1)");

    let mut wires2: Vec<Option<WireValue>> = vec![None; Wire::MAX_INDEX];
    wires2[Wire::from("b").index()] = Some(part1);
    simulate(&mut wires2, connections2);
    let part2 =
        wires2[Wire::from("a").index()].expect("invalid input: wire 'a' has no value (part 2)");

    (part1, part2)
}

fn simulate(wires: &mut Vec<Option<WireValue>>, mut connections: Vec<Connection>) {
    while !connections.is_empty() {
        for i in (0..connections.len()).rev() {
            let connection = &connections[i];
            let wire_idx = connection.out_wire.index();

            if wires[wire_idx].is_some() {
                if connection.out_wire == Wire::from("b") {
                    connections.remove(i);
                    continue;
                } else {
                    panic!(
                        "invalid input: multiple inputs to wire {}",
                        &connection.out_wire
                    );
                }
            }

            let result = match &connection.expression {
                Expression::Literal(value) => Some(*value),
                Expression::Wire(wire) => wires[wire.index()],
                Expression::Or(a, b) => match (wires[a.index()], wires[b.index()]) {
                    (Some(value_a), Some(value_b)) => Some(value_a | value_b),
                    _ => None,
                },
                Expression::And(a, b) => {
                    if let Some(value_b) = wires[Wire::from(b).index()] {
                        let value_a = if a == "1" {
                            Some(1)
                        } else if let Some(value_a) = wires[Wire::from(a).index()] {
                            Some(value_a)
                        } else {
                            None
                        };
                        value_a.map(|value_a| value_a & value_b)
                    } else {
                        None
                    }
                }
                Expression::Not(wire) => wires[wire.index()].map(|value| !value),
                Expression::LeftShift(wire, x) => wires[wire.index()].map(|value| value << x),
                Expression::RightShift(wire, x) => wires[wire.index()].map(|value| value >> x),
            };
            if let Some(value) = result {
                wires[wire_idx] = Some(value);
                connections.remove(i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_index_logic() {
        let mut all = vec![false; 1000];
        for c in 'a'..'z' {
            let mut s = String::from(c);
            let wire = Wire::from(s.as_str());
            assert!(!all[wire.index()]);
            all[wire.index()] = true;
        }
        for c1 in 'a'..'z' {
            for c2 in 'a'..'z' {
                let mut s = String::new();
                s.push(c1);
                s.push(c2);
                let wire = Wire::from(s.as_str());
                assert!(!all[wire.index()]);
                all[wire.index()] = true;
            }
        }
    }

    #[test]
    fn wire_to_string() {
        for c in 'a'..'z' {
            let s = c.to_string();
            assert_eq!(Wire::from(&s).to_string(), s);
        }
        for c1 in 'a'..'z' {
            for c2 in 'a'..'z' {
                let mut s = String::new();
                s.push(c1);
                s.push(c2);
                assert_eq!(Wire::from(&s).to_string(), s);
            }
        }
    }

    #[test]
    fn example() {
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"
            .to_string();

        let expected_str = "d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456"
            .to_string();
        let mut expected = vec![None; 'z' as usize + 1];

        for line in expected_str.lines() {
            let (wire, value) = line.split_once(": ").unwrap();
            let wire = Wire::from(wire);
            let value = value.parse::<WireValue>().unwrap();
            expected[wire.index()] = Some(value);
        }

        let mut wires: Vec<Option<WireValue>> = vec![None; Wire::MAX_INDEX];
        simulate(&mut wires, parse_input(input));

        assert!(wires.starts_with(&expected));
    }
}
