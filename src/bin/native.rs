use glutin::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, ModifiersState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use iced_glutin::{glutin, program, Clipboard, Debug, Size};
use iced_solstice::{Backend, Renderer, Settings, Viewport};
use iced_winit::conversion;

pub fn main() {
    // env_logger::init();

    // Initialize winit
    let event_loop = EventLoop::new();
    let wb = iced_glutin::glutin::window::WindowBuilder::new()
        .with_title("Editor")
        // .with_fullscreen(Some(glutin::window::Fullscreen::Borderless(None)))
        .with_inner_size(PhysicalSize::new(1280, 720))
        .with_resizable(true);
    let window_ctx = iced_glutin::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(8)
        .build_windowed(wb, &event_loop)
        .unwrap();
    let window_ctx = unsafe { window_ctx.make_current() }.unwrap();
    let gl = unsafe {
        use glow::HasContext;
        use solstice::glow;
        let gl = glow::Context::from_loader_function(|addr| window_ctx.get_proc_address(addr));
        // gl.enable(glow::FRAMEBUFFER_SRGB);
        // gl.enable(glow::BLEND);
        // gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

        // Disable multisampling by default
        // gl.disable(glow::MULTISAMPLE);
        gl
    };
    let mut gl = solstice::Context::new(gl);

    let physical_size = window_ctx.window().inner_size();
    let mut viewport = Viewport::with_physical_size(
        Size::new(physical_size.width, physical_size.height),
        window_ctx.window().scale_factor(),
    );
    gl.set_viewport(
        0,
        0,
        viewport.physical_size().width as _,
        viewport.physical_size().height as _,
    );
    let mut gfx = renderer::Renderer {
        d2: renderer::Graphics::new(
            &mut gl,
            viewport.physical_size().width as _,
            viewport.physical_size().height as _,
        )
        .unwrap(),
    };

    let mut cursor_position = PhysicalPosition::new(-1.0, -1.0);
    let mut modifiers = ModifiersState::default();

    let mut resized = false;

    // Initialize iced
    let mut debug = Debug::new();
    let mut renderer = Renderer::new(Backend::new(&mut gl, Settings::default()));
    let mut clipboard = Clipboard::connect(window_ctx.window());

    let seed = rand::RngCore::next_u64(&mut rand::thread_rng());
    let controls = spirits_within_app::Application::new(seed);

    let mut state = program::State::new(
        controls,
        viewport.logical_size(),
        conversion::cursor_position(cursor_position, viewport.scale_factor()),
        &mut renderer,
        &mut debug,
    );

    let epoch = std::time::Instant::now();

    // Run event loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        cursor_position = position;
                    }
                    //     WindowEvent::MouseWheel { delta, .. } => match delta {
                    //         MouseScrollDelta::LineDelta(_x, y) => {
                    //             state.queue_message(Message::TargetFrameDelta(y.round() as i32));
                    //         }
                    //         MouseScrollDelta::PixelDelta(delta) => {
                    //             state.queue_message(Message::TargetFrameDelta(delta.y.round() as i32));
                    //         }
                    //     },
                    WindowEvent::ModifiersChanged(new_modifiers) => {
                        modifiers = new_modifiers;
                    }
                    WindowEvent::Resized(new_size) => {
                        viewport = Viewport::with_physical_size(
                            Size::new(new_size.width, new_size.height),
                            window_ctx.window().scale_factor(),
                        );

                        resized = true;
                    }
                    //     WindowEvent::KeyboardInput {
                    //         input:
                    //         KeyboardInput {
                    //             state: key_state,
                    //             virtual_keycode: Some(keycode),
                    //             ..
                    //         },
                    //         ..
                    //     } => {}
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }

                // Map window event to iced event
                let scale_factor = window_ctx.window().scale_factor();
                let event = conversion::window_event(&event, scale_factor, modifiers);
                if let Some(event) = event {
                    state.queue_event(event);
                }
            }
            Event::MainEventsCleared => {
                // If there are events pending
                if !state.is_queue_empty() {
                    // We update iced
                    let _ = state.update(
                        viewport.logical_size(),
                        conversion::cursor_position(cursor_position, viewport.scale_factor()),
                        &mut renderer,
                        &mut clipboard,
                        &mut debug,
                    );
                }
                // and request a redraw
                window_ctx.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                if resized {
                    let size = window_ctx.window().inner_size();
                    gl.set_viewport(0, 0, size.width as i32, size.height as i32);
                    gfx.d2.set_width_height(size.width as _, size.height as _);

                    resized = false;
                }

                solstice::Renderer::clear(
                    &mut gl,
                    solstice::ClearSettings {
                        color: Some(
                            solstice::Color {
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

                {
                    let now = std::time::Instant::now();
                    const DURATION: f32 = 10.;
                    let t = now.duration_since(epoch).as_secs_f32() % DURATION / DURATION;
                    gfx.draw(&mut gl, t);
                }
                // game_renderer.render(
                //     &mut gl,
                //     state.program().context.state(),
                //     state.program().context.level(),
                // );

                // And then iced on top
                let _mouse_interaction = renderer.backend_mut().draw(
                    &mut gl,
                    &viewport,
                    state.primitive(),
                    &debug.overlay(),
                );

                // Update the mouse cursor
                // window_ctx
                //     .window()
                //     .set_cursor_icon(conversion::mouse_interaction(mouse_interaction));

                window_ctx
                    .swap_buffers()
                    .expect("terrible, terrible damage");
            }
            _ => {}
        }
    })
}
