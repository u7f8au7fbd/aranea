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
    area_frames: Vec<Frame>,      // 各エリアのフレームを静的に保持
}

impl MyApp {
    // エリアの初期化を行う
    fn init_area_frames(&mut self, ctx: &egui::Context) {
        if self.area_frames.is_empty() {
            // 黒色の枠線 (初期状態)
            let default_frame =
                Frame::canvas(&ctx.style()).stroke(Stroke::new(1.0, Color32::BLACK));
            self.area_frames.push(default_frame); // area_1用
            self.area_frames.push(default_frame); // area_2用
        }
    }

    // 選択状態に応じてフレームを更新する
    fn update_frame_for_selected_area(&mut self) {
        for (i, frame) in self.area_frames.iter_mut().enumerate() {
            if Some(i + 1) == self.selected_area {
                *frame =
                    Frame::canvas(&egui::Style::default()).stroke(Stroke::new(1.0, Color32::GREEN));
            } else {
                *frame =
                    Frame::canvas(&egui::Style::default()).stroke(Stroke::new(1.0, Color32::BLACK));
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 初期化処理：一度だけ実行される
        self.init_area_frames(ctx);

        // 選択状態に応じてフレームを更新
        self.update_frame_for_selected_area();

        let screen_size = ctx.screen_rect().size();
        let half_width = screen_size.x / 2.0;
        let margin = 10.0; // 余白を設定

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Selectable Areas with Green Border");

            // 左上の領域
            Area::new("Area 1".into())
                .fixed_pos(Pos2::new(margin, margin))
                .show(ctx, |ui| {
                    self.area_frames[0].show(ui, |ui| {
                        ui.set_min_size(Vec2::new(
                            half_width - margin * 1.5,
                            screen_size.y - margin * 1.5,
                        ));
                        let response =
                            ui.interact(ui.max_rect(), ui.id().with("area_1"), Sense::click());
                        if response.clicked() {
                            self.selected_area = Some(1); // エリア1がクリックされたら選択状態に
                        }
                        ui.label("Left");
                    });
                });

            // 右上の領域
            Area::new("Area 2".into())
                .fixed_pos(Pos2::new(half_width + margin / 2.0, margin))
                .show(ctx, |ui| {
                    self.area_frames[1].show(ui, |ui| {
                        ui.set_min_size(Vec2::new(
                            half_width - margin * 1.5,
                            screen_size.y - margin * 1.5,
                        ));
                        let response =
                            ui.interact(ui.max_rect(), ui.id().with("area_2"), Sense::click());
                        if response.clicked() {
                            self.selected_area = Some(2); // エリア2がクリックされたら選択状態に
                        }
                        ui.label("Right");
                    });
                });
        });
    }
}
