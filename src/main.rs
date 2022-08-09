use ggez::{event::EventHandler, *};

fn main() {
    let state = GameState {
        delta_time: std::time::Duration::new(0, 0),
        path: Path::new(),
    };

    // Pass this to ContextBuilder.default_conf() when we have more configuration
    // let config = conf::Conf::new();

    let (ctx, event_loop) = ContextBuilder::new("Tower Defense", "Henrique Gasques")
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}

struct GameState {
    delta_time: std::time::Duration,
    path: Path,
}

struct Path {
    coordinates: Vec<(f32, f32)>,
    bg_color: graphics::Color,
}

impl Path {
    fn new() -> Path {
        todo!()
    } 
}

impl EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.delta_time = ctx.time.delta();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let bg_color = graphics::Color::GREEN;
        let mut canvas = graphics::Canvas::from_frame(ctx, bg_color);

        // let circle = graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     mint::Point2{x: 200.0, y: 300.0},
        //     100.0, // Radius
        //     0.1, // Tolerance
        //     graphics::Color::WHITE,
        //     )?;
        //
        // canvas.draw(&circle, graphics::DrawParam::default());

        draw_path(ctx, &mut canvas)?;
        canvas.finish(ctx)?;

        Ok(())
    }
}

fn draw_path(ctx: &mut Context, canvas: &mut graphics::Canvas) -> Result<(), GameError> {
    let path_coordinates = vec![(250.0, 250.0), (250.0, 300.0), (250.0, 350.0), (300.0, 350.0) ];

    for (x, y) in path_coordinates {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(x, y, 50.0, 50.0),
            graphics::Color::WHITE,
            )?;

        canvas.draw(&rect, graphics::DrawParam::default());
    }

    Ok(())
}
