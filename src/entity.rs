use svg::node::element::{path::Data, Path};
use svg::Document;

pub enum CubeViz {
    Face(Face),
}

impl CubeViz {
    pub fn tosvg(self) -> String {
        match self {
            CubeViz::Face(f) => f.tosvg(),
        }
    }
}

pub struct Face {
    data: [[Color; 3]; 3],
    side: Option<[[Color; 3]; 4]>,
}

impl Face {
    pub fn new(data: [[Color; 3]; 3], side: Option<[[Color; 3]; 4]>) -> Self {
        Self { data, side }
    }
    pub fn tosvg(self) -> String {
        const CUBE_SIZE: i64 = 12;
        const MARGIN: i64 = 2;
        const SIDE_SIZE: i64 = 6;
        const STROKE_WIDTH: i64 = 1;
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
        let mut document = Document::new();
        for i in 0..3_usize {
            for j in 0..3_usize {
                document = document.add(sq(
                    (MARGIN + CUBE_SIZE) * j as i64,
                    (MARGIN + CUBE_SIZE) * i as i64,
                    self.data[i][j],
                ));
            }
        }
        if let Some(side) = self.side {
            for j in 0..3 {
                document = document.add(side_hr(
                    (MARGIN + CUBE_SIZE) * j as i64,
                    -SIDE_SIZE - MARGIN,
                    side[0][j],
                ));
            }
            for i in 0..3_usize {
                document = document.add(side_vt(
                    -SIDE_SIZE - MARGIN,
                    (MARGIN + CUBE_SIZE) * i as i64,
                    side[1][i],
                ));
            }
            for i in 0..3_usize {
                document = document.add(side_vt(
                    MARGIN * 3 + CUBE_SIZE * 3,
                    (MARGIN + CUBE_SIZE) * i as i64,
                    side[2][i],
                ));
            }
            for j in 0..3 {
                document = document.add(side_hr(
                    (MARGIN + CUBE_SIZE) * j as i64,
                    MARGIN * 3 + CUBE_SIZE * 3,
                    side[3][j],
                ));
            }
        }

        document = document.set(
            "viewBox",
            if self.side.is_none() {
                (
                    -MARGIN,
                    -MARGIN,
                    CUBE_SIZE * 3 + MARGIN * 4,
                    CUBE_SIZE * 3 + MARGIN * 4,
                )
            } else {
                (
                    -MARGIN * 2 - SIDE_SIZE,
                    -MARGIN * 2 - SIDE_SIZE,
                    CUBE_SIZE * 3 + MARGIN * 6 + SIDE_SIZE * 2,
                    CUBE_SIZE * 3 + MARGIN * 6 + SIDE_SIZE * 2,
                )
            },
        );
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
            Orange => "#fb7",
            Blue => "#7f7",
            Green => "#77f",
            Mask => "#c5c5c5",
        };
        String::from(r)
    }
}