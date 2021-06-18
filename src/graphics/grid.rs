use super::*;

pub struct Grid;

impl Grid {
    pub fn draw(
        ctx: &mut Context,
        origin: Point,
        (screen_width, screen_height): (f32, f32),
        zoom: u8,
    ) {
        let lerp = |start: f32, end: f32| -> f32 {
            (end - start) / (MAX_ZOOM as f32) * zoom as f32 + start
        };

        let scale = zoom_to_scale(zoom);
        let tile_width = scale * TILE_WIDTH;
        let tile_height = scale * TILE_HEIGHT;

        let delta = Point {
            x: (origin.x) % (tile_width),
            y: (origin.y) % (tile_height),
        };

        let green = Color::new(0., 1., 0., lerp(0.25, 0.5));
        let red = Color::new(1., 0., 0., lerp(0., 0.25));
        let blue = Color::new(0., 0., 1., 1.);

        let mut mesh = MeshBuilder::new();

        // Vertical lines
        let mut x = delta.x + 1.;
        while x < screen_width {
            mesh.line(&[[x, 0.], [x, screen_height]], 1., green)
                .unwrap();

            for i in 1..16 {
                let i = i as f32 * tile_width / 16.;
                mesh.line(&[[x + i, 0.], [x + i, screen_height]], 1., red)
                    .unwrap();
            }

            x += tile_width;
        }

        // Horizontal lines
        let mut y = delta.y + 1.;
        while y < screen_height {
            mesh.line(&[[0., y], [screen_width, y]], 1., green).unwrap();

            for i in 1..16 {
                let i = i as f32 * tile_height / 16.;
                mesh.line(&[[0., y + i], [screen_width, y + i]], 1., red)
                    .unwrap();
            }

            y += tile_height;
        }

        // Origin
        mesh.circle(
            DrawMode::stroke(1.),
            [origin.x, origin.y],
            (tile_width + tile_height) / 8.,
            1.,
            blue,
        )
        .unwrap();

        // Base
        mesh.line(
            &[[1. + origin.x, 1. + origin.y], [
                1. + origin.x + tile_width,
                1. + origin.y,
            ]],
            1.,
            blue,
        )
        .unwrap();
        mesh.line(
            &[[1. + origin.x, 1. + origin.y], [
                1. + origin.x,
                1. + origin.y + tile_height,
            ]],
            1.,
            blue,
        )
        .unwrap();

        let mesh = mesh.build(ctx).unwrap();
        mesh.draw(ctx, Default::default()).unwrap();
    }
}
