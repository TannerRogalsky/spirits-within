use glutin::{dpi::PhysicalSize, event_loop::EventLoop, window, ContextBuilder};

fn main() {
    let event_loop = EventLoop::new();
    let wb = window::WindowBuilder::new()
        .with_title("Main")
        // .with_fullscreen(Some(glutin::window::Fullscreen::Borderless(None)))
        .with_inner_size(PhysicalSize::new(1280, 720))
        .with_resizable(true);
    let window_ctx = ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(8)
        .build_windowed(wb, &event_loop)
        .unwrap();
    let window_ctx = unsafe { window_ctx.make_current() }.unwrap();
    let ctx = unsafe {
        // use solstice_2d::solstice::glow::{self, HasContext};
        let gl = solstice_2d::solstice::glow::Context::from_loader_function(|addr| {
            window_ctx.get_proc_address(addr)
        });
        // gl.enable(glow::FRAMEBUFFER_SRGB);
        // gl.enable(glow::BLEND);
        // gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        gl
    };
    let mut ctx = solstice_2d::solstice::Context::new(ctx);
    let physical_size = window_ctx.window().inner_size();
    ctx.set_viewport(0, 0, physical_size.width as _, physical_size.height as _);
    let mut renderer = renderer::Renderer {
        d2: renderer::Graphics::new(
            &mut ctx,
            physical_size.width as _,
            physical_size.height as _,
        )
        .unwrap(),
    };

    let epoch = std::time::Instant::now();

    event_loop.run(move |event, _target, cf| {
        use glutin::event::*;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *cf = glutin::event_loop::ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    *cf = glutin::event_loop::ControlFlow::Exit;
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                window_ctx.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                solstice_2d::solstice::Renderer::clear(
                    &mut ctx,
                    solstice_2d::solstice::ClearSettings {
                        color: Some(
                            solstice_2d::solstice::Color {
                                red: 1f32,
                                blue: 1.,
                                green: 1.,
                                alpha: 1.,
                            }
                            .into(),
                        ),
                        ..Default::default()
                    },
                );

                const DURATION: f32 = 10.;
                let t = epoch.elapsed().as_secs_f32() % DURATION / DURATION;
                renderer.draw(&mut ctx, t);
                window_ctx
                    .swap_buffers()
                    .expect("terrible, terrible damage");
            }
            _ => {}
        }
    });
}
