use eframe::egui::{self, Area, CentralPanel, Color32, Frame, Pos2, Sense, Stroke, Vec2};

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 1200.0])
            .with_resizable(true)
            .with_transparent(true),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Selectable Areas with Gradient Borders",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
}

#[derive(Default)]
struct MyApp {
    selected_area: Option<usize>, // 選択されたエリアを追跡
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_size = ctx.screen_rect().size();
        let half_width = screen_size.x / 2.0; // 領域1が画面の半分の幅を占める
        let half_height = screen_size.y / 2.0;
        let margin = 10.0; // 余白を設定

        CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
                let draw_gradient_border =
                    |ui: &mut egui::Ui,
                     rect: egui::Rect,
                     selected: Option<usize>,
                     area_index: usize| {
                        let painter = ui.painter();
                        let border_thickness = 3.0;
                        let num_steps = 20; // グラデーションの段階数
                        let selected_top_color = Color32::from_rgb(0, 255, 255); // シアン
                        let selected_bottom_color = Color32::from_rgb(255, 0, 255); // マゼンタ

                        let unselected_top_color = Color32::from_rgb(0, 80, 80); // 黒っぽい薄いシアン
                        let unselected_bottom_color = Color32::from_rgb(80, 0, 80); // 黒っぽい薄いマゼンタ

                        let (top_color, bottom_color) = if selected == Some(area_index) {
                            (selected_top_color, selected_bottom_color)
                        } else {
                            (unselected_top_color, unselected_bottom_color)
                        };

                        for i in 0..num_steps {
                            let t = i as f32 / (num_steps as f32 - 1.0);
                            let r = (1.0 - t) * top_color.r() as f32 + t * bottom_color.r() as f32;
                            let g = (1.0 - t) * top_color.g() as f32 + t * bottom_color.g() as f32;
                            let b = (1.0 - t) * top_color.b() as f32 + t * bottom_color.b() as f32;
                            let current_color = Color32::from_rgb(r as u8, g as u8, b as u8);

                            let y1 = rect.top() + (i as f32 / num_steps as f32) * rect.height();
                            let y2 =
                                rect.top() + ((i + 1) as f32 / num_steps as f32) * rect.height();

                            painter.line_segment(
                                [Pos2::new(rect.left(), y1), Pos2::new(rect.left(), y2)],
                                Stroke::new(border_thickness, current_color),
                            );

                            painter.line_segment(
                                [Pos2::new(rect.right(), y1), Pos2::new(rect.right(), y2)],
                                Stroke::new(border_thickness, current_color),
                            );
                        }

                        painter.line_segment(
                            [rect.left_top(), rect.right_top()],
                            Stroke::new(border_thickness, top_color),
                        );

                        painter.line_segment(
                            [rect.left_bottom(), rect.right_bottom()],
                            Stroke::new(border_thickness, bottom_color),
                        );
                    };

                let selected_area = self.selected_area;
                // 左側の縦長の領域（1番）を画面の半分まで広げる
                Area::new("Area 1".into())
                    .fixed_pos(Pos2::new(margin, margin))
                    .show(ctx, |ui| {
                        Frame::default()
                            .fill(Color32::from_rgba_premultiplied(0, 0, 0, 96))
                            .show(ui, |ui| {
                                ui.set_min_size(Vec2::new(
                                    half_width - margin * 1.5,
                                    screen_size.y - margin * 2.0,
                                ));
                                let response = ui.interact(
                                    ui.max_rect(),
                                    ui.id().with("area_1"),
                                    Sense::click(),
                                );
                                if response.clicked() {
                                    self.selected_area = Some(1);
                                }
                                draw_gradient_border(ui, ui.max_rect(), selected_area, 1);
                                ui.label("1");
                            });
                    });

                // 右上の領域（2番）を残りの半分に調整
                Area::new("Area 2".into())
                    .fixed_pos(Pos2::new(half_width + margin / 2.0, margin))
                    .show(ctx, |ui| {
                        Frame::default()
                            .fill(Color32::from_rgba_premultiplied(0, 0, 0, 96))
                            .show(ui, |ui| {
                                ui.set_min_size(Vec2::new(
                                    half_width - margin * 1.5,
                                    half_height - margin * 1.5,
                                ));
                                let response = ui.interact(
                                    ui.max_rect(),
                                    ui.id().with("area_2"),
                                    Sense::click(),
                                );
                                if response.clicked() {
                                    self.selected_area = Some(2);
                                }
                                draw_gradient_border(ui, ui.max_rect(), selected_area, 2);
                                ui.label("2");
                            });
                    });

                // 右下の領域（3番）も残りの半分に調整
                Area::new("Area 3".into())
                    .fixed_pos(Pos2::new(
                        half_width + margin / 2.0,
                        half_height + margin / 2.0,
                    ))
                    .show(ctx, |ui| {
                        Frame::default()
                            .fill(Color32::from_rgba_premultiplied(0, 0, 0, 96))
                            .show(ui, |ui| {
                                ui.set_min_size(Vec2::new(
                                    half_width - margin * 1.5,
                                    half_height - margin * 1.5,
                                ));
                                let response = ui.interact(
                                    ui.max_rect(),
                                    ui.id().with("area_3"),
                                    Sense::click(),
                                );
                                if response.clicked() {
                                    self.selected_area = Some(3);
                                }
                                draw_gradient_border(ui, ui.max_rect(), selected_area, 3);
                                ui.label("3");
                            });
                    });

                // 領域4は消去
            });
    }
}
