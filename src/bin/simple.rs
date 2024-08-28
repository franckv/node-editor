use tracing::Level;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter, FmtSubscriber};

use node_editor::{
    compiler::{ShaderCompiler, ShaderSection},
    graph::SimpleNode,
    ui::NodeUI,
};

#[derive(Default)]
struct App {
    node_ui: NodeUI<SimpleNode, ShaderCompiler, ShaderSection>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.node_ui.draw_ui(ctx);
    }
}

pub fn init_logger() {
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Logger initialized");
}

fn main() {
    init_logger();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024., 768.]),
        ..Default::default()
    };
    eframe::run_native("NodeUI", options, Box::new(|_| Ok(Box::<App>::default()))).unwrap();
}
