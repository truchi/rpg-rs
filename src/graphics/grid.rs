use super::*;

pub struct Grid;

impl Grid {
    pub fn draw(ctx: &mut Context, viewport: Viewport) {
        let mut grid = MeshBuilder::new();
        let mut base = MeshBuilder::new();

        Self::grid(&mut grid, viewport);
        Self::base(&mut base, viewport);

        let dest = |origin, tile| {
            if origin > 0. {
                (origin % tile) - tile
            } else if origin < 0. {
                origin % tile
            } else {
                0.
            }
        };

        let grid = grid.build(ctx).unwrap();
        grid.draw(
            ctx,
            DrawParam::new().dest([
                dest(viewport.origin().x, viewport.tile().x),
                dest(viewport.origin().y, viewport.tile().y),
            ]),
        )
        .unwrap();

        let base = base.build(ctx).unwrap();
        base.draw(ctx, DrawParam::new().dest(viewport.origin()))
            .unwrap();
    }

    fn grid(mesh: &mut MeshBuilder, viewport: Viewport) {
        let green = Color::new(0., 1., 0., 0.5);
        let red = Color::new(
            0.,
            0.5,
            0.,
            if viewport.scale() >= SCALE_LIMIT / 2. {
                0.25
            } else {
                0.
            },
        );

        let w = viewport.tile().x;
        let h = viewport.tile().y;
        let width = viewport.w() + w;
        let height = viewport.h() + h;

        // Vertical lines
        let mut x = 1.;
        while x < width {
            Self::vertical(mesh, x, height, green);

            for i in 1..16 {
                let i = i as f32 * w / 16.;
                Self::vertical(mesh, x + i, height, red);
            }

            x += w;
        }

        // Horizontal lines
        let mut y = 1.;
        while y < height {
            Self::horizontal(mesh, y, width, green);

            for i in 1..16 {
                let i = i as f32 * h / 16.;
                Self::horizontal(mesh, y + i, width, red);
            }

            y += h;
        }
    }

    fn base(mesh: &mut MeshBuilder, viewport: Viewport) {
        let blue = Color::new(0., 0., 1., 1.);
        let w = viewport.tile().x;
        let h = viewport.tile().y;

        // Origin
        Self::circle(mesh, (w + h) / 8., blue);

        // Base
        Self::horizontal(mesh, 1., w, blue);
        Self::vertical(mesh, 1., h, blue);
    }

    fn vertical(mesh: &mut MeshBuilder, x: f32, h: f32, color: Color) {
        mesh.line(&[[x, 0.], [x, h]], 1., color).unwrap();
    }

    fn horizontal(mesh: &mut MeshBuilder, y: f32, w: f32, color: Color) {
        mesh.line(&[[0., y], [w, y]], 1., color).unwrap();
    }

    fn circle(mesh: &mut MeshBuilder, r: f32, color: Color) {
        mesh.circle(DrawMode::stroke(1.), [0., 0.], r, 1., color)
            .unwrap();
    }
}
