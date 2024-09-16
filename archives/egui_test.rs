use eframe::egui::{self, Area, Color32, Context, Frame, Pos2, Rect, Sense, Stroke, Vec2};
use eframe::{App, NativeOptions};

fn main() {
    let options = NativeOptions {
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Dynamic Area Split with Gradient Border and Remove Feature",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
}

#[derive(Default)]
struct MyApp {
    areas: Vec<AreaData>,            // エリアのデータ
    selected_area: Option<usize>,    // 選択されたエリアを追跡
    area_pairs: Vec<(usize, usize)>, // エリアのペア (分割時の親子関係を記録)
}

struct AreaData {
    id: usize,
    relative_size: Vec2, // ウィンドウに対する割合のサイズ
    relative_pos: Vec2,  // ウィンドウに対する割合のポジション
}

impl MyApp {
    // グラデーションの縁を描画する関数
    fn draw_gradient_border(
        ui: &mut egui::Ui,
        rect: Rect,
        selected: bool, // 選択されているかどうかを確認
    ) {
        let painter = ui.painter();
        let border_thickness = 3.0;
        let num_steps = 20; // グラデーションの段階数

        // 選択された場合の明るいグラデーション
        let selected_top_color = Color32::from_rgb(255, 0, 255); // マゼンタ
        let selected_bottom_color = Color32::from_rgb(0, 255, 255); // シアン

        // 選択されていない場合の暗めのグラデーション
        let unselected_top_color = Color32::from_rgb(100, 0, 100); // 暗めのマゼンタ
        let unselected_bottom_color = Color32::from_rgb(0, 100, 100); // 暗めのシアン

        let (top_color, bottom_color) = if selected {
            (selected_top_color, selected_bottom_color)
        } else {
            (unselected_top_color, unselected_bottom_color)
        };

        // 左右のグラデーション描画
        for i in 0..num_steps {
            let t = i as f32 / (num_steps as f32 - 1.0);
            let r = (1.0 - t) * top_color.r() as f32 + t * bottom_color.r() as f32;
            let g = (1.0 - t) * top_color.g() as f32 + t * bottom_color.g() as f32;
            let b = (1.0 - t) * top_color.b() as f32 + t * bottom_color.b() as f32;
            let current_color = Color32::from_rgb(r as u8, g as u8, b as u8);

            let y1 = rect.top() + (i as f32 / num_steps as f32) * rect.height();
            let y2 = rect.top() + ((i + 1) as f32 / num_steps as f32) * rect.height();

            painter.line_segment(
                [Pos2::new(rect.left(), y1), Pos2::new(rect.left(), y2)],
                Stroke::new(border_thickness, current_color),
            );

            painter.line_segment(
                [Pos2::new(rect.right(), y1), Pos2::new(rect.right(), y2)],
                Stroke::new(border_thickness, current_color),
            );
        }

        // 上部の直線
        painter.line_segment(
            [rect.left_top(), rect.right_top()],
            Stroke::new(border_thickness, top_color),
        );

        // 下部の直線
        painter.line_segment(
            [rect.left_bottom(), rect.right_bottom()],
            Stroke::new(border_thickness, bottom_color),
        );
    }

    // フレーム色とクリックの処理: 動的に処理される部分のみ
    fn handle_dynamic_ui(&mut self, ctx: &Context) {
        let screen_size = ctx.screen_rect().size();
        let margin = 20.0; // 各エリア間のマージン

        for area in &self.areas {
            let frame = Frame::default()
                .fill(Color32::from_rgba_premultiplied(0, 0, 0, 96)) // 半透明の黒背景を適用
                .stroke(Stroke::new(2.0, Color32::TRANSPARENT));

            // 相対サイズと相対位置をウィンドウサイズに基づいて計算
            let size = Vec2::new(
                screen_size.x * area.relative_size.x - margin,
                screen_size.y * area.relative_size.y - margin,
            );
            let pos = Pos2::new(
                screen_size.x * area.relative_pos.x + margin / 2.0,
                screen_size.y * area.relative_pos.y + margin / 2.0,
            );

            // フレームの描画とクリック判定
            Area::new(format!("Area {}", area.id).into())
                .fixed_pos(pos)
                .show(ctx, |ui| {
                    frame.show(ui, |ui| {
                        ui.set_min_size(size);
                        let rect = ui.min_rect(); // エリアの矩形

                        // グラデーションのアウトライン描画
                        let selected = self.selected_area == Some(area.id);
                        MyApp::draw_gradient_border(ui, rect, selected);

                        let response = ui.interact(
                            rect,
                            ui.id().with(format!("area_{}", area.id)),
                            Sense::click(),
                        );

                        if response.clicked() {
                            self.selected_area = Some(area.id);
                            ctx.request_repaint(); // クリックされた場合のみ再描画を要求
                        }

                        ui.label(format!("Area {}", area.id));
                    });
                });
        }
    }

    // エリアを縦横比に基づいて分割
    fn split_selected_area(&mut self, ctx: &egui::Context) {
        if let Some(selected_area_id) = self.selected_area {
            if let Some(index) = self
                .areas
                .iter()
                .position(|area| area.id == selected_area_id)
            {
                let screen_size = ctx.screen_rect().size();

                // 現在のウィンドウサイズを基に、各エリアの実際のサイズを計算
                let area_size = Vec2::new(
                    screen_size.x * self.areas[index].relative_size.x,
                    screen_size.y * self.areas[index].relative_size.y,
                );

                if area_size.x > area_size.y {
                    // 横に分割（左右に新しいエリアを配置）
                    let new_width = self.areas[index].relative_size.x / 2.0;
                    let new_pos_x = self.areas[index].relative_pos.x + new_width;

                    let new_area_id = self.areas.len() + 1;
                    self.area_pairs.push((selected_area_id, new_area_id)); // ペアを記録
                    self.areas[index].relative_size.x = new_width; // 既存エリアの幅を半分に
                    self.areas.push(AreaData {
                        id: new_area_id,
                        relative_size: Vec2::new(new_width, self.areas[index].relative_size.y),
                        relative_pos: Vec2::new(new_pos_x, self.areas[index].relative_pos.y),
                    });
                } else {
                    // 縦に分割（上下に新しいエリアを配置）
                    let new_height = self.areas[index].relative_size.y / 2.0;
                    let new_pos_y = self.areas[index].relative_pos.y + new_height;

                    let new_area_id = self.areas.len() + 1;
                    self.area_pairs.push((selected_area_id, new_area_id)); // ペアを記録
                    self.areas[index].relative_size.y = new_height; // 既存エリアの高さを半分に
                    self.areas.push(AreaData {
                        id: new_area_id,
                        relative_size: Vec2::new(self.areas[index].relative_size.x, new_height),
                        relative_pos: Vec2::new(self.areas[index].relative_pos.x, new_pos_y),
                    });
                }
            }
        }
    }

    // エリア削除とサイズ復元
    fn delete_selected_area(&mut self, ctx: &egui::Context) {
        if let Some(selected_area_id) = self.selected_area {
            // エリアがペアであるかどうか確認
            if let Some((parent_id, child_id)) = self
                .area_pairs
                .iter()
                .find(|(p, c)| *p == selected_area_id || *c == selected_area_id)
            {
                let (remaining_id, _) = if *parent_id == selected_area_id {
                    (*child_id, *parent_id)
                } else {
                    (*parent_id, *child_id)
                };

                // 残ったエリアを元の大きさに戻す
                if let Some(index) = self.areas.iter().position(|area| area.id == remaining_id) {
                    let remaining_area = &mut self.areas[index];
                    let original_size = Vec2::new(
                        remaining_area.relative_size.x * 2.0,
                        remaining_area.relative_size.y * 2.0,
                    );
                    remaining_area.relative_size = original_size;
                }

                // ペアから削除し、選択したエリアを削除
                self.area_pairs
                    .retain(|(p, c)| *p != selected_area_id && *c != selected_area_id);
                self.areas.retain(|area| area.id != selected_area_id);

                self.selected_area = None;
                ctx.request_repaint();
            }
        }
    }

    // キー入力処理
    fn check_key_input(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::Q)) {
            if self.selected_area.is_some() {
                self.split_selected_area(ctx);
                ctx.request_repaint(); // エリアが分割されたときに再描画を要求
            }
        }

        if ctx.input(|i| i.key_pressed(egui::Key::C)) {
            if self.selected_area.is_some() && self.areas.len() > 1 {
                self.delete_selected_area(ctx); // 選択されたエリアを削除
            }
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 起動時に1つのエリアを作成
        if self.areas.is_empty() {
            self.areas.push(AreaData {
                id: 1,
                relative_size: Vec2::new(1.0, 1.0), // ウィンドウ全体をカバーする
                relative_pos: Vec2::new(0.0, 0.0),
            });
        }

        // キー入力をチェックしてエリアを分割・削除
        self.check_key_input(ctx);

        // 動的なフレームの色やクリック判定部分の処理
        self.handle_dynamic_ui(ctx);
    }
}
