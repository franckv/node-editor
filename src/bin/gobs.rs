use tracing::Level;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter, FmtSubscriber};

use gobs::{
    game::{
        app::{Application, Run},
        input::Input,
    },
    gfx::Device,
    render::{
        context::Context,
        graph::{FrameGraph, RenderError},
        pass::PassType,
        renderable::Renderable,
    },
    ui::UIRenderer,
};

use node_editor::{node::math::MathNode, ui::NodeUI};

struct App {
    graph: FrameGraph,
    ui: UIRenderer,
    node_ui: NodeUI<MathNode>,
}

impl Run for App {
    async fn create(ctx: &Context) -> Self {
        let graph = FrameGraph::ui(ctx);
        let ui = UIRenderer::new(ctx, graph.pass_by_type(PassType::Ui).unwrap());

        App {
            graph,
            ui,
            node_ui: NodeUI::default(),
        }
    }

    fn update(&mut self, ctx: &Context, delta: f32) {
        self.graph.update(ctx, delta);

        self.ui.update(
            ctx,
            self.graph.pass_by_type(PassType::Ui).unwrap(),
            delta,
            |ectx| {
                self.node_ui.draw_ui(ectx);
            },
        );
    }

    fn render(&mut self, ctx: &mut Context) -> Result<(), RenderError> {
        tracing::trace!("Render frame {}", ctx.frame_number);

        self.graph.begin(ctx)?;

        self.graph.render(ctx, &mut |pass, batch| match pass.ty() {
            PassType::Ui => {
                self.ui.draw(ctx, pass, batch);
            }
            _ => (),
        })?;

        self.graph.end(ctx)?;

        tracing::trace!("End render");

        Ok(())
    }

    fn input(&mut self, _ctx: &Context, input: Input) {
        self.ui.input(input);
    }

    fn resize(&mut self, ctx: &mut Context, width: u32, height: u32) {
        self.graph.resize(ctx);
        self.ui.resize(width, height);
    }

    async fn start(&mut self, _ctx: &Context) {}

    fn close(&mut self, ctx: &Context) {
        tracing::info!("Closing");

        ctx.device.wait();

        tracing::info!("Closed");
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

    tracing::info!("Engine start");

    Application::<App>::new("Egui", 1024, 768).run();
}
