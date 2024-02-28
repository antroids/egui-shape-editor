use egui::{Color32, Margin, Pos2, Shape, Stroke};

#[derive(Clone)]
pub struct Light {
    pub path_point_stroke: Stroke,
    pub control_point_radius: f32,
    pub control_point_stroke: Stroke,
    pub preview_point_stroke: Stroke,

    pub canvas_bg_color: Color32,

    pub border_stroke: Stroke,

    pub selection_stroke: Stroke,
    pub selection_dash_length: f32,
    pub selection_gap_length: f32,

    pub rulers_width: f32,
    pub rulers_stroke: Stroke,
    pub rulers_half_stroke: Stroke,
    pub rulers_sub_stroke: Stroke,
    pub rulers_font: egui::FontId,
    pub rulers_font_color: Color32,
    pub rulers_div_height: f32,
    pub rulers_half_div_height: f32,
    pub rulers_sub_div_height: f32,
    pub rulers_text_position: Pos2,

    pub grid_line_zero_stroke: Stroke,
    pub grid_line_primary_stroke: Stroke,
    pub grid_line_secondary_stroke: Stroke,
    pub grid_line_secondary_gap: f32,
    pub grid_line_secondary_length: f32,

    pub snap_highlight_stroke: Stroke,
    pub snap_highlight_dash_length: f32,
    pub snap_highlight_gap_length: f32,
    pub snap_highlight_point_mark_size: f32,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            path_point_stroke: Stroke::new(2.0, Color32::RED),
            control_point_radius: 5.0,
            control_point_stroke: Stroke::new(2.0, Color32::GREEN),
            preview_point_stroke: Stroke::new(2.0, Color32::GRAY),

            canvas_bg_color: Color32::WHITE,
            border_stroke: Stroke::new(3.0, Color32::GRAY),

            selection_stroke: Stroke::new(1.0, Color32::GRAY),
            selection_dash_length: 2.0,
            selection_gap_length: 2.0,

            rulers_width: 16.0,
            rulers_stroke: Stroke::new(1.0, Color32::GRAY),
            rulers_half_stroke: Stroke::new(1.0, Color32::GRAY),
            rulers_sub_stroke: Stroke::new(1.0, Color32::GRAY),
            rulers_font: egui::FontId::monospace(8.0),
            rulers_font_color: Color32::GRAY,
            rulers_div_height: 3.0,
            rulers_half_div_height: 2.0,
            rulers_sub_div_height: 1.0,
            rulers_text_position: Pos2::new(0.0, 2.0),

            grid_line_zero_stroke: Stroke::new(2.0, Color32::LIGHT_GRAY),
            grid_line_primary_stroke: Stroke::new(0.5, Color32::LIGHT_GRAY),
            grid_line_secondary_stroke: Stroke::new(0.5, Color32::LIGHT_GRAY),
            grid_line_secondary_gap: 3.0,
            grid_line_secondary_length: 3.0,

            snap_highlight_stroke: Stroke::new(1.0, Color32::GRAY),
            snap_highlight_dash_length: 5.0,
            snap_highlight_gap_length: 5.0,
            snap_highlight_point_mark_size: 20.0,
        }
    }
}

impl Style for Light {
    fn selection_shape(&self, min: Pos2, max: Pos2) -> Shape {
        let mut vec = Shape::dashed_line(
            &[min, Pos2::new(max.x, min.y)],
            self.selection_stroke,
            self.selection_dash_length,
            self.selection_gap_length,
        );
        vec.extend(Shape::dashed_line(
            &[min, Pos2::new(min.x, max.y)],
            self.selection_stroke,
            self.selection_dash_length,
            self.selection_gap_length,
        ));
        vec.extend(Shape::dashed_line(
            &[Pos2::new(min.x, max.y), max],
            self.selection_stroke,
            self.selection_dash_length,
            self.selection_gap_length,
        ));
        vec.extend(Shape::dashed_line(
            &[Pos2::new(max.x, min.y), max],
            self.selection_stroke,
            self.selection_dash_length,
            self.selection_gap_length,
        ));
        Shape::Vec(vec)
    }
    fn rulers_margins(&self) -> Margin {
        Margin {
            left: self.rulers_width,
            right: 0.0,
            top: self.rulers_width,
            bottom: 0.0,
        }
    }
    fn path_point_stroke(&self) -> Stroke {
        self.path_point_stroke
    }
    fn control_point_radius(&self) -> f32 {
        self.control_point_radius
    }
    fn control_point_stroke(&self) -> Stroke {
        self.control_point_stroke
    }
    fn preview_point_stroke(&self) -> Stroke {
        self.preview_point_stroke
    }
    fn canvas_bg_color(&self) -> Color32 {
        self.canvas_bg_color
    }
    fn border_stroke(&self) -> Stroke {
        self.border_stroke
    }
    fn selection_stroke(&self) -> Stroke {
        self.selection_stroke
    }
    fn selection_dash_length(&self) -> f32 {
        self.selection_dash_length
    }
    fn selection_gap_length(&self) -> f32 {
        self.selection_gap_length
    }
    fn rulers_width(&self) -> f32 {
        self.rulers_width
    }
    fn rulers_stroke(&self) -> Stroke {
        self.rulers_stroke
    }
    fn rulers_half_stroke(&self) -> Stroke {
        self.rulers_half_stroke
    }
    fn rulers_sub_stroke(&self) -> Stroke {
        self.rulers_sub_stroke
    }
    fn rulers_font(&self) -> &egui::FontId {
        &self.rulers_font
    }
    fn rulers_font_color(&self) -> Color32 {
        self.rulers_font_color
    }
    fn rulers_div_height(&self) -> f32 {
        self.rulers_div_height
    }
    fn rulers_half_div_height(&self) -> f32 {
        self.rulers_half_div_height
    }
    fn rulers_sub_div_height(&self) -> f32 {
        self.rulers_sub_div_height
    }
    fn rulers_text_position(&self) -> Pos2 {
        self.rulers_text_position
    }
    fn grid_line_zero_stroke(&self) -> Stroke {
        self.grid_line_zero_stroke
    }
    fn grid_line_primary_stroke(&self) -> Stroke {
        self.grid_line_primary_stroke
    }
    fn grid_line_secondary_stroke(&self) -> Stroke {
        self.grid_line_secondary_stroke
    }
    fn grid_line_secondary_gap(&self) -> f32 {
        self.grid_line_secondary_gap
    }
    fn grid_line_secondary_length(&self) -> f32 {
        self.grid_line_secondary_length
    }
    fn snap_highlight_stroke(&self) -> Stroke {
        self.snap_highlight_stroke
    }
    fn snap_highlight_dash_length(&self) -> f32 {
        self.snap_highlight_dash_length
    }
    fn snap_highlight_gap_length(&self) -> f32 {
        self.snap_highlight_gap_length
    }
    fn snap_highlight_point_mark_size(&self) -> f32 {
        self.snap_highlight_point_mark_size
    }
}

pub trait Style {
    fn selection_shape(&self, min: Pos2, max: Pos2) -> Shape;
    fn rulers_margins(&self) -> Margin;
    fn path_point_stroke(&self) -> Stroke;
    fn control_point_radius(&self) -> f32;
    fn control_point_stroke(&self) -> Stroke;
    fn preview_point_stroke(&self) -> Stroke;
    fn canvas_bg_color(&self) -> Color32;
    fn border_stroke(&self) -> Stroke;
    fn selection_stroke(&self) -> Stroke;
    fn selection_dash_length(&self) -> f32;
    fn selection_gap_length(&self) -> f32;
    fn rulers_width(&self) -> f32;
    fn rulers_stroke(&self) -> Stroke;
    fn rulers_half_stroke(&self) -> Stroke;
    fn rulers_sub_stroke(&self) -> Stroke;
    fn rulers_font(&self) -> &egui::FontId;
    fn rulers_font_color(&self) -> Color32;
    fn rulers_div_height(&self) -> f32;
    fn rulers_half_div_height(&self) -> f32;
    fn rulers_sub_div_height(&self) -> f32;
    fn rulers_text_position(&self) -> Pos2;
    fn grid_line_zero_stroke(&self) -> Stroke;
    fn grid_line_primary_stroke(&self) -> Stroke;
    fn grid_line_secondary_stroke(&self) -> Stroke;
    fn grid_line_secondary_gap(&self) -> f32;
    fn grid_line_secondary_length(&self) -> f32;
    fn snap_highlight_stroke(&self) -> Stroke;
    fn snap_highlight_dash_length(&self) -> f32;
    fn snap_highlight_gap_length(&self) -> f32;
    fn snap_highlight_point_mark_size(&self) -> f32;
}
