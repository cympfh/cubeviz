use std::collections::HashMap;
use svg::node::{
    self,
    element::{path::Data, Path, Text},
};
use svg::{node::element::Group, Document};

const CUBE_SIZE: i64 = 12;
const MARGIN: i64 = 2;
const SIDE_SIZE: i64 = 6;
const STROKE_WIDTH: i64 = 1;
const FONT_SIZE: i64 = 8;

pub enum CubeViz {
    Face(Face),
    Cube(Cube),
}

impl CubeViz {
    pub fn tosvg(&self) -> String {
        match self {
            CubeViz::Face(f) => f.tosvg(),
            CubeViz::Cube(c) => c.tosvg(),
        }
    }
}

#[derive(Clone)]
pub enum AttributeValue {
    Str(String),
}

pub struct Face {
    data: [[Color; 3]; 3],
    side: Option<[[Color; 3]; 4]>,
    attr: HashMap<String, AttributeValue>,
}

impl Face {
    pub fn new(
        data: [[Color; 3]; 3],
        side: Option<[[Color; 3]; 4]>,
        attr: HashMap<String, AttributeValue>,
    ) -> Self {
        Self { data, side, attr }
    }
    fn tosvg(&self) -> String {
        let mut g = Group::new();
        fn rect(x: i64, y: i64, width: i64, height: i64, color: Color) -> Path {
            let data = Data::new()
                .move_to((x, y))
                .line_by((width, 0))
                .line_by((0, height))
                .line_by((-width, 0))
                .close();
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", Color::Mask.rgb())
                .set("stroke-width", STROKE_WIDTH)
                .set("fill", color.rgb())
                .set("d", data);
            path
        }
        fn sq(x: i64, y: i64, color: Color) -> Path {
            rect(x, y, CUBE_SIZE, CUBE_SIZE, color)
        }
        fn side_hr(x: i64, y: i64, color: Color) -> Path {
            rect(x, y, CUBE_SIZE, SIDE_SIZE, color)
        }
        fn side_vt(x: i64, y: i64, color: Color) -> Path {
            rect(x, y, SIDE_SIZE, CUBE_SIZE, color)
        }
        let mut vleft = -MARGIN;
        let mut vtop = -MARGIN;
        let mut vwidth = MARGIN * 4 + CUBE_SIZE * 3;
        let mut vheight = MARGIN * 4 + CUBE_SIZE * 3;
        for i in 0..3_usize {
            for j in 0..3_usize {
                g = g.add(sq(
                    (MARGIN + CUBE_SIZE) * j as i64,
                    (MARGIN + CUBE_SIZE) * i as i64,
                    self.data[i][j],
                ));
            }
        }
        if let Some(side) = self.side {
            vleft -= MARGIN + SIDE_SIZE;
            vtop -= MARGIN + SIDE_SIZE;
            vwidth += (MARGIN + SIDE_SIZE) * 2;
            vheight += (MARGIN + SIDE_SIZE) * 2;
            for j in 0..3 {
                g = g.add(side_hr(
                    (MARGIN + CUBE_SIZE) * j as i64,
                    -SIDE_SIZE - MARGIN,
                    side[0][j],
                ));
            }
            for i in 0..3_usize {
                g = g.add(side_vt(
                    -SIDE_SIZE - MARGIN,
                    (MARGIN + CUBE_SIZE) * i as i64,
                    side[1][i],
                ));
            }
            for i in 0..3_usize {
                g = g.add(side_vt(
                    MARGIN * 3 + CUBE_SIZE * 3,
                    (MARGIN + CUBE_SIZE) * i as i64,
                    side[2][i],
                ));
            }
            for j in 0..3 {
                g = g.add(side_hr(
                    (MARGIN + CUBE_SIZE) * j as i64,
                    MARGIN * 3 + CUBE_SIZE * 3,
                    side[3][j],
                ));
            }
        }
        if let Some(value) = self.attr.get("label") {
            match value {
                AttributeValue::Str(label) => {
                    let label = Text::new(label)
                        .set("x", vleft + vwidth / 2)
                        .set("y", vtop + vheight + FONT_SIZE)
                        .set("font-size", FONT_SIZE)
                        .set("text-anchor", "middle")
                        .set("dominant-baseline", "central");
                    vheight += MARGIN + FONT_SIZE;
                    g = g.add(label);
                }
            }
        }
        let mut document = Document::new();
        document = document.add(g);
        document = document.set("viewBox", (vleft, vtop, vwidth, vheight));
        document.to_string()
    }
}

pub struct Cube {
    up: Face,
    front: Face,
    right: Face,
    back: Face,
    left: Face,
    down: Face,
    attr: HashMap<String, AttributeValue>,
}

impl Cube {
    pub fn from(lines: Vec<Vec<Color>>, attr: HashMap<String, AttributeValue>) -> Self {
        fn to_face(lines: Vec<Vec<Color>>) -> Face {
            let mut data = [[Color::Mask; 3]; 3];
            data[0].copy_from_slice(&lines[0]);
            data[1].copy_from_slice(&lines[1]);
            data[2].copy_from_slice(&lines[2]);
            Face {
                data,
                side: None,
                attr: HashMap::new(),
            }
        }
        let up = to_face(lines[0..3].to_vec());
        let front = to_face(
            lines[3..6]
                .iter()
                .map(|line| line[..3].to_vec())
                .collect::<Vec<_>>(),
        );
        let right = to_face(
            lines[3..6]
                .iter()
                .map(|line| line[3..6].to_vec())
                .collect::<Vec<_>>(),
        );
        let back = to_face(
            lines[3..6]
                .iter()
                .map(|line| line[6..9].to_vec())
                .collect::<Vec<_>>(),
        );
        let left = to_face(
            lines[3..6]
                .iter()
                .map(|line| line[9..].to_vec())
                .collect::<Vec<_>>(),
        );
        let down = to_face(lines[6..9].to_vec());
        Self {
            up,
            front,
            right,
            back,
            left,
            down,
            attr,
        }
    }
    fn tosvg(&self) -> String {
        fn rect(x: i64, y: i64, width: i64, height: i64, color: Color) -> Path {
            let data = Data::new()
                .move_to((x, y))
                .line_by((width, 0))
                .line_by((0, height))
                .line_by((-width, 0))
                .close();
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", Color::Mask.rgb())
                .set("stroke-width", STROKE_WIDTH)
                .set("fill", color.rgb())
                .set("d", data);
            path
        }
        fn sq(x: i64, y: i64, color: Color) -> Path {
            rect(x, y, CUBE_SIZE, CUBE_SIZE, color)
        }
        fn face(f: &Face, offset_x: i64, offset_y: i64) -> Group {
            let mut g = Group::new();
            for i in 0..3_usize {
                for j in 0..3_usize {
                    g = g.add(sq(
                        offset_x + (MARGIN + CUBE_SIZE) * j as i64,
                        offset_y + (MARGIN + CUBE_SIZE) * i as i64,
                        f.data[i][j],
                    ));
                }
            }
            g
        }
        let cwidth = MARGIN * 4 + CUBE_SIZE * 3;
        let cheight = MARGIN * 4 + CUBE_SIZE * 3;
        let mut document = Document::new();
        document = document.add(face(&self.left, 0, cheight));
        document = document.add(face(&self.front, cwidth, cheight));
        document = document.add(face(&self.right, cwidth * 2, cheight));
        document = document.add(face(&self.back, cwidth * 3, cheight));
        document = document.add(face(&self.up, cwidth, 0));
        document = document.add(face(&self.down, cwidth, cheight * 2));
        let vleft = -MARGIN;
        let vtop = -MARGIN;
        let vwidth = cwidth * 4;
        let vheight = cheight * 3;
        document = document.set("viewBox", (vleft, vtop, vwidth, vheight));
        document.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
    Mask,
}

impl Color {
    fn rgb(&self) -> String {
        use Color::*;
        let r = match self {
            White => "#f9f9f9",
            Yellow => "#ee4",
            Red => "#f77",
            Orange => "#fa4",
            Blue => "#77f",
            Green => "#7f7",
            Mask => "#c5c5c5",
        };
        String::from(r)
    }
}
