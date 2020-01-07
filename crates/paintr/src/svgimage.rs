mod parser;

use std::borrow::Borrow;
use std::sync::Arc;

use crate::Paintable;
use druid::kurbo::BezPath;
use druid::{Color, Point, Rect, RenderContext, Size, Vec2};

use parser::{
    path::{Command, Position},
    SvgNode, SvgNodeKind,
};

struct DefaultBrush {
    fill: Option<Color>,
    stroke: Option<(Color, f64)>,
}

pub struct SvgImage {
    root: Arc<SvgNode>,
    view_box: Option<Rect>,
    default_brush: DefaultBrush,
}

fn position(last: Point, mode: &Position, params: &[f32]) -> Point {
    match mode {
        Position::Absolute => Point::new(params[0] as f64, params[1] as f64),
        Position::Relative => (last + Vec2::new(params[0] as f64, params[1] as f64)),
    }
}

fn paint_node(node: &SvgNode, ctx: &mut impl RenderContext, brush: &DefaultBrush) {
    let mut last: Point = (0.0, 0.0).into();

    if let SvgNodeKind::Path = &node.kind {
        let mut path = BezPath::new();
        let mut last_ctrl = None;
        let mut start = None;

        for cmd in &node.commands {
            let mut curr_ctrl = None;

            match cmd {
                Command::Move(pos, params) => {
                    path.move_to(position(last, pos, &params));
                    last = position(last, pos, &params);
                    start = Some(last);
                }
                Command::Line(pos, params) => {
                    path.line_to(position(last, pos, &params));
                    last = position(last, pos, &params);
                }
                Command::CubicCurve(pos, params) => {
                    path.curve_to(
                        position(last, pos, &params[0..]),
                        position(last, pos, &params[2..]),
                        position(last, pos, &params[4..]),
                    );

                    let ctrl = position(last, pos, &params[2..]);
                    last = position(last, pos, &params[4..]);
                    curr_ctrl = Some(last + (last.to_vec2() - ctrl.to_vec2()));
                }
                Command::SmoothCubicCurve(pos, params) => {
                    path.curve_to(
                        last_ctrl.unwrap_or(last),
                        position(last, pos, &params[0..]),
                        position(last, pos, &params[2..]),
                    );

                    let ctrl = position(last, pos, &params[0..]);
                    last = position(last, pos, &params[2..]);
                    curr_ctrl = Some(last + (last.to_vec2() - ctrl.to_vec2()));
                }
                Command::VerticalLine(pos, params) => {
                    let x = match pos {
                        Position::Absolute => last.x as f32,
                        Position::Relative => 0.0,
                    };
                    path.line_to(position(last, pos, &[x, params[0]]));
                    last = position(last, pos, &[x, params[0]]);
                }
                Command::HorizontalLine(pos, params) => {
                    let y = match pos {
                        Position::Absolute => last.y as f32,
                        Position::Relative => 0.0,
                    };
                    path.line_to(position(last, pos, &[params[0], y]));
                    last = position(last, pos, &[params[0], y]);
                }
                Command::Close => {
                    path.close_path();
                    if let Some(start) = start {
                        last = start;
                    }
                    start = Some(last);
                }
                _ => eprintln!("SVG Command [{:?}] is not implemented", cmd),
            }

            if start.is_none() {
                start = Some(last);
            }

            last_ctrl = curr_ctrl;
        }

        // FIXME: Handle custom fill and stroke
        if let Some(fill) = &brush.fill {
            ctx.fill(path.clone(), fill);
        }

        if let Some((brush, width)) = &brush.stroke {
            ctx.stroke(path, brush, *width);
        }
    }

    for child in &node.children {
        paint_node(child, ctx, brush);
    }
}

impl Paintable for SvgImage {
    fn paint(&self, render_ctx: &mut impl RenderContext) {
        paint_node(&self.root, render_ctx, &self.default_brush);
    }
    fn paint_size(&self) -> Option<Size> {
        self.view_box.map(|rt| rt.size())
    }
}

impl SvgImage {
    pub fn new(data: impl Borrow<str>) -> Result<SvgImage, Box<dyn std::error::Error>> {
        let root = SvgNode::parse(data)?;
        let view_box = root.view_box();

        let default_brush = DefaultBrush { stroke: None, fill: Some(Color::BLACK) };

        Ok(SvgImage { root: Arc::new(root), view_box, default_brush })
    }

    pub fn set_default_fill(&mut self, color: Color) {
        self.default_brush.fill = Some(color);
    }
}
