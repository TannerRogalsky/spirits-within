pub use solstice_2d::*;

pub struct Renderer {
    pub d2: solstice_2d::Graphics,
}

impl Renderer {
    pub fn draw(&mut self, ctx: &mut solstice::Context, t: f32) {
        ctx.enable(solstice_2d::solstice::Feature::CullFace(
            solstice_2d::solstice::CullFace::Back,
            solstice_2d::solstice::VertexWinding::CounterClockWise,
        ));
        let mut g = self.d2.lock(ctx);

        g.set_camera(Transform3D::translation(0., 0., -1.5));

        let tx = Transform3D::rotation(Rad(0.), Rad(t * std::f32::consts::TAU), Rad(0.));

        let geometry = Polyhedron::dodecahedron(1., 0);

        let mut points = Vec::new();

        const CONNECTORS: [[f32; 4]; 3] =
            [[1., 0., 0., 0.25], [0., 1., 0., 0.25], [0., 0., 1., 0.25]];

        for (index, indices) in geometry.indices.chunks(3).enumerate() {
            let p1 = geometry.vertices[indices[0] as usize].normalize();
            let p2 = geometry.vertices[indices[1] as usize].normalize();
            let p3 = geometry.vertices[indices[2] as usize].normalize();

            fn lv(p: Point3D, color: [f32; 4]) -> LineVertex {
                LineVertex {
                    position: [p.x, p.y, p.z],
                    width: 2.0,
                    color,
                }
            }

            // const ALPHA: [f32; 4] = [0., 0., 0., 0.25];
            let alpha = CONNECTORS[index % 3];
            const COLOR: [f32; 4] = [0.1, 0.1, 0.1, 0.5];
            points.extend_from_slice(&[
                lv(p1, alpha),
                lv(p1, COLOR),
                lv(p2, COLOR),
                lv(p3, COLOR),
                lv(p3, alpha),
            ]);
        }

        g.set_transform(tx);
        g.line_3d(points);
        g.set_transform(Transform3D::default());

        g.draw_with_color_and_transform(geometry, [1., 0., 0., 0.1], tx);
        // g.stroke_with_color_and_transform(geometry, [0., 0., 0., 1.], tx);
        drop(g);
        ctx.disable(solstice_2d::solstice::Feature::CullFace(
            solstice_2d::solstice::CullFace::Back,
            solstice_2d::solstice::VertexWinding::CounterClockWise,
        ));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
