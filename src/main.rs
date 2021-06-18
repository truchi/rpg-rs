mod consts;
mod editor;
mod graphics;
mod imports;
mod input;

use imports::*;

pub use consts::*;
pub use editor::*;
pub use graphics::*;
pub use input::*;

pub type Point<T = f32> = Point2<T>;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("RPG", "Romain TRUCHI")
        .window_setup(WindowSetup {
            title: String::from("RPG"),
            ..<_>::default()
        })
        .window_mode(WindowMode {
            width: 1366.0,
            height: 768.0,
            maximized: true,
            fullscreen_type: FullscreenType::True,
            ..<_>::default()
        })
        .build()
        .expect("Could not create ggez context!");

    let editor = Editor::new(&mut ctx);
    event::run(ctx, event_loop, editor);
}
