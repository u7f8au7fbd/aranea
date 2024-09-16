use eframe::egui::{self, Area, CentralPanel, Color32, Frame, Pos2, Sense, Stroke, Vec2};
use eframe::NativeOptions;

fn main() {
    let options = NativeOptions {
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Selectable Areas Example",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
}

#[derive(Default)]
struct MyApp {
    selected_area: Option<usize>, // 選択されたエリアを追跡
    click_registered: bool,       // クリックが登録されたか
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_size = ctx.screen_rect().size();
        let half_width = screen_size.x / 2.0;
        let margin = 10.0; // 余白を設定

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Selectable Areas with High Responsiveness");

            // フレーム定義を共通化して冗長性を排除
            let mut draw_area = |ui: &mut egui::Ui, area_num: usize, label: &str, pos: Pos2| {
                let is_selected = self.selected_area == Some(area_num);
                let frame_color = if is_selected { Color32::GREEN } else { Color32::BLACK };
                let frame = Frame::canvas(ui.style()).stroke(Stroke::new(2.0, frame_color));

                Area::new(format!("Area {}", area_num).into())
                    .fixed_pos(pos)
                    .show(ctx, |ui| {
                        frame.show(ui, |ui| {
                            ui.set_min_size(Vec2::new(
                                half_width - margin * 1.5,
                                screen_size.y - margin * 1.5,
                            ));
                            let response = ui.interact(
                                ui.min_rect(),
                                ui.id().with(format!("area_{}", area_num)),
                                Sense::click(),
                            );

                            // クリックイベントを即時反映させる
                            if response.clicked() {
                                self.selected_area = Some(area_num);
                                self.click_registered = true;
                                ctx.request_repaint(); // 即時再描画を要求
                            }

                            ui.label(label);
                        });
                    });
            };

            // 左上の領域
            draw_area(ui, 1, "Left", Pos2::new(margin, margin));

            // 右上の領域
            draw_area(ui, 2, "Right", Pos2::new(half_width + margin / 2.0, margin));
        });

        // クリックが登録された場合に即座に再描画
        if self.click_registered {
            self.click_registered = false;
            ctx.request_repaint(); // もう一度再描画を要求してクリックを素早く反映
        }
    }
}
