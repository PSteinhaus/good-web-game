use stdweb::{
    traits::*,
    web::{CanvasRenderingContext2d, FillRule, TextAlign, TextBaseline},
};

use crate::goodies::matrix_transform_2d::*;
use crate::graphics::types::Rect;

use cgmath::{EuclideanSpace, InnerSpace, Matrix3, Point2, Vector2};

pub struct CanvasContext {
    pub canvas: CanvasRenderingContext2d,
}

impl CanvasContext {
    pub fn new(canvas: CanvasRenderingContext2d) -> Self {
        CanvasContext { canvas }
    }
}

impl CanvasContext {
    pub(crate) fn set_transform_with_matrix(&self, matrix: &Matrix3<f32>) {
        self.canvas.set_transform(
            matrix.x.x as f64,
            matrix.x.y as f64,
            matrix.y.x as f64,
            matrix.y.y as f64,
            matrix.z.x as f64,
            matrix.z.y as f64,
        );
    }

    pub(crate) fn transform_with_matrix(&self, matrix: &Matrix3<f32>) {
        self.canvas.transform(
            matrix.x.x as f64,
            matrix.x.y as f64,
            matrix.y.x as f64,
            matrix.y.y as f64,
            matrix.z.x as f64,
            matrix.z.y as f64,
        );
    }

    pub fn set_transform(&self, transform: &Matrix3<f32>) {
        self.set_transform_with_matrix(transform);
    }

    pub fn push_transform(&self, transform: &Matrix3<f32>) {
        self.canvas.save();
        self.transform_with_matrix(transform);
    }

    pub fn pop_transform(&self) {
        self.canvas.restore();
    }

    pub fn size(&self) -> (f64, f64) {
        let width = self.canvas.get_canvas().offset_width();
        let height = self.canvas.get_canvas().offset_height();

        (width as f64, height as f64)
    }

    pub fn set_screen_coordinates(&mut self, rect: Rect) {
        let (width, height) = self.size();
        let translate = Matrix3::from_translation(Vector2::new(-rect.x, -rect.y));
        let scale = Matrix3::from_nonuniform_scale(width as f32 / rect.w, height as f32 / rect.h);
        let transform = scale * translate;

        self.set_transform(&transform);
    }

    pub fn clear(&self) {
        let size = self.size();

        self.canvas.save();
        self.set_transform(&cgmath::One::one());
        self.canvas
            .clear_rect(0.0f64, 0.0f64, size.0 as f64, size.1 as f64);
        self.canvas.restore();
    }

    pub fn draw_arc(&self, position: Point2<f32>, radius: f32, attrs: &[ArcAttr]) {
        self.canvas.save();

        let mut angle = std::f32::consts::PI * 2.0;
        let mut start_angle = 0.;
        let mut sector = false;
        let mut forward = false;
        for attr in attrs.iter() {
            match attr {
                ArcAttr::Stroke(color) => {
                    self.canvas.set_stroke_style_color(color);
                }
                ArcAttr::Fill(color) => {
                    self.canvas.set_fill_style_color(color);
                }
                ArcAttr::Dash(dash) => self
                    .canvas
                    .set_line_dash(dash.iter().map(|d| *d as f64).collect()),
                ArcAttr::Angle(a) => angle = *a,
                ArcAttr::Forward(dir) => {
                    start_angle = Vector2::new(dir.x, -dir.y).angle(Vector2::new(-1., 0.)).0;
                    forward = true;
                }
                ArcAttr::Sector => sector = true,
            }
        }
        if forward == false {
            start_angle = angle / 2.;
        }

        self.canvas.begin_path();
        if sector {
            self.canvas.move_to(position.x as f64, position.y as f64);
        }
        self.canvas.arc(
            position.x as f64,
            position.y as f64,
            radius as f64,
            (start_angle - angle / 2.) as f64,
            (start_angle + angle / 2.) as f64,
            false,
        );
        if sector {
            self.canvas.move_to(position.x as f64, position.y as f64);
        }
        for attr in attrs.iter() {
            match attr {
                ArcAttr::Stroke(_) => self.canvas.stroke(),
                ArcAttr::Fill(_) => {
                    self.canvas.fill(FillRule::NonZero);
                }
                _ => {}
            }
        }
        self.canvas.restore();
    }

    pub fn draw_line(&self, from: Point2<f32>, to: Point2<f32>, color: &str) {
        self.canvas.save();
        self.canvas.set_stroke_style_color(color);
        self.canvas.begin_path();
        self.canvas.move_to(from.x as f64, from.y as f64);
        self.canvas.line_to(to.x as f64, to.y as f64);
        self.canvas.stroke();
        self.canvas.restore();
    }

    pub fn draw_rect(&self, rect: Rect, attrs: &[RectAttr]) {
        self.canvas.save();
        let (mut x, mut y) = (rect.x, rect.y);

        for attr in attrs.iter() {
            match attr {
                RectAttr::Stroke(color) => {
                    self.canvas.set_stroke_style_color(color);
                }
                RectAttr::Fill(color) => {
                    self.canvas.set_fill_style_color(color);
                }
                RectAttr::Rotate(angle) => {
                    self.canvas.translate(
                        rect.x as f64 + rect.w as f64 / 2.,
                        rect.y as f64 + rect.h as f64 / 2.,
                    );
                    self.canvas.rotate(*angle as f64);
                    x = -rect.w / 2.;
                    y = -rect.h / 2.;
                }
            }
        }
        self.canvas.begin_path();
        self.canvas
            .rect(x as f64, y as f64, rect.w as f64, rect.h as f64);
        for attr in attrs.iter() {
            match attr {
                RectAttr::Stroke(_) => self.canvas.stroke(),
                RectAttr::Fill(_) => {
                    self.canvas.fill(FillRule::NonZero);
                }
                RectAttr::Rotate(_) => {}
            }
        }

        self.canvas.restore();
    }

    pub fn draw_label(
        &self,
        label: &str,
        pos: Point2<f32>,
        scale: Option<Vector2<f32>>,
        font: Option<&str>,
        color: Option<&str>,
    ) {
        self.canvas.save();
        if let Some(scale) = scale {
            let scale = Matrix3::from_nonuniform_scale(scale.x as f32, scale.y as f32);
            let pos = Matrix3::from_translation(pos.cast::<f32>().unwrap().to_vec());
            let transform = pos * scale;

            self.push_transform(&transform);
        }
        if let Some(color) = color {
            self.canvas.set_fill_style_color(color);
        }
        if font.is_some() {
            self.canvas.set_font(font.unwrap());
        }

        self.canvas.set_text_align(TextAlign::Left);
        self.canvas.set_text_baseline(TextBaseline::Hanging);

        let (x, y) = if scale.is_none() {
            (pos.x as f64, pos.y as f64)
        } else {
            (0., 0.)
        };
        for (n, line) in label.split('\n').enumerate() {
            self.canvas.fill_text(line, x, y + n as f64 * 10., None);
        }

        if scale.is_some() {
            self.pop_transform();
        }
        self.canvas.restore();
    }

    pub fn measure_label(&self, label: &str, font: Option<&str>) -> Vector2<f32> {
        self.canvas.save();
        if font.is_some() {
            self.canvas.set_font(font.unwrap());
        }
        self.canvas.set_text_align(TextAlign::Left);
        self.canvas.set_text_baseline(TextBaseline::Hanging);
        let mut max_width = 0.;
        let mut height = 0.;
        for (n, line) in label.split('\n').enumerate() {
            let measures = self.canvas.measure_text(line).unwrap();
            let width = measures.get_width();
            if max_width < width {
                max_width = width;
            }
            height = (n + 1) as f64 * 10.;
        }
        self.canvas.restore();

        Vector2::new(max_width as f32, height as f32)
    }
}

pub enum RectAttr<'a> {
    Stroke(&'a str),
    Fill(&'a str),
    Rotate(f32),
}

pub enum ArcAttr {
    Stroke(&'static str),
    Fill(&'static str),
    Dash(Vec<f32>),
    Angle(f32),
    Forward(Vector2<f32>),
    Sector,
}
