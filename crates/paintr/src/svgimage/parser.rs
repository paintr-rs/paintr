use std::borrow::Borrow;
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Type;
use svg::node::Value;
use svg::parser::Event;

use druid::{Point, Rect, Size};

use std::collections::HashMap;
type SvgAttrs = HashMap<String, Value>;

// reexport
pub(crate) use svg::node::element::path;

#[derive(Debug)]
pub(crate) struct SvgNode {
    pub(crate) kind: SvgNodeKind,
    attrs: SvgAttrs,
    pub(crate) children: Vec<SvgNode>,
    pub(crate) commands: Vec<Command>,
}

#[derive(Debug)]
pub(crate) enum SvgNodeKind {
    Svg,
    Group,
    Path,
    Text,
    Unknown(String),
}

fn read_node<'a>(
    iter: &mut std::iter::Peekable<impl Iterator<Item = Event<'a>>>,
) -> Option<SvgNode> {
    match iter.next()? {
        Event::Tag(path, ty, attrs) => {
            let kind = match path {
                "svg" => SvgNodeKind::Svg,
                "g" => SvgNodeKind::Group,
                "path" => SvgNodeKind::Path,
                "text" => SvgNodeKind::Text,
                _ => SvgNodeKind::Unknown(path.to_string()),
            };

            let children = match ty {
                Type::Empty => Vec::new(),
                Type::End => unreachable!(),
                Type::Start => {
                    let mut children = Vec::new();

                    while let Some(next) = iter.peek() {
                        if let Event::Tag(_, ty, _) = next {
                            if ty == &Type::End {
                                // Consume the end tag
                                let _ = iter.next();
                                break;
                            }
                        }
                        if let Some(child) = read_node(iter) {
                            children.push(child);
                        }
                    }
                    children
                }
            };

            let commands = match attrs.get("d").map(|data| Data::parse(data).unwrap()) {
                Some(data) => data.iter().cloned().collect(),
                None => Vec::new(),
            };

            Some(SvgNode { kind, attrs, children, commands })
        }
        _ => None,
    }
}

impl SvgNode {
    pub fn parse(data: impl Borrow<str>) -> Result<SvgNode, Box<dyn std::error::Error>> {
        let svg_data = std::io::Cursor::new(data.borrow());

        let mut iter = svg::read(svg_data)?.into_iter().peekable();
        Ok(read_node(&mut iter).unwrap())
    }

    pub fn view_box(&self) -> Option<Rect> {
        let value = self.attrs.get("viewBox")?;
        let values: Result<Vec<_>, _> = value.split(" ").map(|s| s.parse::<f64>()).collect();
        let values = values.ok()?;

        Some(Rect::from_origin_size(
            Point::new(*values.get(0)?, *values.get(1)?),
            Size::new(*values.get(2)?, *values.get(3)?),
        ))
    }
}

fn display_node(f: &mut std::fmt::Formatter<'_>, node: &SvgNode, depth: usize) -> std::fmt::Result {
    let indent = "\t".repeat(depth);

    writeln!(f, "{}{:?} [len = {}]", indent, node.kind, node.children.len())?;
    writeln!(f, "{}\tattrs =>{:?}", indent, node.attrs)?;

    for n in &node.children {
        display_node(f, n, depth + 1)?;
    }

    for command in &node.commands {
        writeln!(f, "{}\t\t{:?}", indent, command)?;
    }

    Ok(())
}

impl std::fmt::Display for SvgNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_node(f, self, 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_node(node: &SvgNode) {
        if let SvgNodeKind::Unknown(s) = &node.kind {
            panic!("Unknown svg name {}", s);
        }
        for n in &node.children {
            check_node(n);
        }
    }

    #[test]
    fn read_svg() {
        let svg_view = SvgNode::parse(include_str!("../tests/assets/noun_move_tool.svg")).unwrap();
        check_node(&svg_view);
        println!("{}", svg_view);
    }
}
