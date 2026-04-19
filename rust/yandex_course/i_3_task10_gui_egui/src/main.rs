use eframe::egui;

struct MyApp {
    counter: i32,
    text: String,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Инициализация состояния (загрузка настроек, шрифтов и т.д.)
        Self {
            counter: 0,
            text: "Привет, Linux!".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    // Основной цикл отрисовки и обработки событий
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Пример egui для Linux");
            ui.separator();

            ui.label("Введите текст:");
            ui.text_edit_singleline(&mut self.text);

            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("Счётчик: {}", self.counter));
                if ui.button("➕ +1").clicked() {
                    self.counter += 1;
                }
                if ui.button("🔄 Сброс").clicked() {
                    self.counter = 0;
                }
            });

            ui.separator();
            ui.small("Работает на X11 и Wayland из коробки");
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "Мое egui приложение",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}