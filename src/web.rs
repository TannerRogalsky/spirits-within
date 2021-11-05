use iced_solstice::{Backend, Renderer, Settings, Viewport};
use iced_winit::{conversion, program, winit::dpi::PhysicalPosition, Clipboard, Debug, Size};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
}

#[allow(unused)]
struct Closures {
    mouse_down: Closure<dyn FnMut(web_sys::MouseEvent)>,
    mouse_move: Closure<dyn FnMut(web_sys::MouseEvent)>,
    mouse_up: Closure<dyn FnMut(web_sys::MouseEvent)>,
}

#[wasm_bindgen]
pub struct Application {
    #[allow(unused)]
    canvas: web_sys::HtmlCanvasElement,
    ctx: solstice::Context,

    events: crossbeam_channel::Receiver<iced_winit::event::Event>,
    #[allow(unused)]
    closures: Closures,

    state: program::State<crate::Application>,
    viewport: Viewport,
    debug: Debug,
    renderer: Renderer,
    clipboard: Clipboard,
    cursor_position: PhysicalPosition<f64>,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<Application, JsValue> {
        use wasm_bindgen::JsCast;
        let ctx = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()?;
        let ctx = solstice::glow::Context::from_webgl1_context(ctx);
        let mut ctx = solstice::Context::new(ctx);

        let physical_size =
            iced_winit::winit::dpi::PhysicalSize::new(canvas.width(), canvas.height());
        let viewport =
            Viewport::with_physical_size(Size::new(physical_size.width, physical_size.height), 1.);

        let mut debug = Debug::new();
        let mut renderer = Renderer::new(Backend::new(&mut ctx, Settings::default()));
        let clipboard = Clipboard::new();

        let cursor_position = PhysicalPosition::new(-1.0, -1.0);
        // let modifiers = ModifiersState::default();

        let controls = crate::Application::new();

        let state = program::State::new(
            controls,
            viewport.logical_size(),
            conversion::cursor_position(cursor_position, viewport.scale_factor()),
            &mut renderer,
            &mut debug,
        );

        let (events_sx, events) = crossbeam_channel::unbounded();
        let closures = {
            use iced_winit::{event::Event, mouse};
            fn button(code: i16) -> mouse::Button {
                match code {
                    0 => mouse::Button::Left,
                    1 => mouse::Button::Middle,
                    2 => mouse::Button::Right,
                    _ => mouse::Button::Other(code as _),
                }
            }

            let mouse_down = {
                let events_sx = events_sx.clone();
                let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                    log::trace!("mouse down");
                    events_sx
                        .send(Event::Mouse(mouse::Event::ButtonPressed(button(
                            event.button(),
                        ))))
                        .unwrap();
                }) as Box<dyn FnMut(_)>);
                canvas.add_event_listener_with_callback(
                    "mousedown",
                    closure.as_ref().unchecked_ref(),
                )?;
                closure
            };
            let mouse_move = {
                let events_sx = events_sx.clone();
                let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                    log::trace!("mouse move");
                    events_sx
                        .send(Event::Mouse(mouse::Event::CursorMoved {
                            position: iced_winit::Point::new(
                                event.offset_x() as _,
                                event.offset_y() as _,
                            ),
                        }))
                        .unwrap();
                }) as Box<dyn FnMut(_)>);
                canvas.add_event_listener_with_callback(
                    "mousemove",
                    closure.as_ref().unchecked_ref(),
                )?;
                closure
            };
            let mouse_up = {
                let events_sx = events_sx.clone();
                let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                    log::trace!("mouse up");
                    events_sx
                        .send(Event::Mouse(mouse::Event::ButtonReleased(button(
                            event.button(),
                        ))))
                        .unwrap();
                }) as Box<dyn FnMut(_)>);
                canvas.add_event_listener_with_callback(
                    "mouseup",
                    closure.as_ref().unchecked_ref(),
                )?;
                closure
            };

            Closures {
                mouse_down,
                mouse_move,
                mouse_up,
            }
        };

        Ok(Self {
            canvas,
            ctx,
            events,
            closures,
            state,
            viewport,
            debug,
            renderer,
            clipboard,
            cursor_position,
        })
    }

    pub fn update(&mut self) {
        for event in self.events.try_iter() {
            match &event {
                iced_winit::Event::Mouse(iced_winit::mouse::Event::CursorMoved { position }) => {
                    self.cursor_position = iced_winit::winit::dpi::PhysicalPosition::new(
                        position.x.into(),
                        position.y.into(),
                    );
                }
                _ => {}
            }
            self.state.queue_event(event);
        }

        if !self.state.is_queue_empty() {
            // We update iced
            let _ = self.state.update(
                self.viewport.logical_size(),
                conversion::cursor_position(self.cursor_position, self.viewport.scale_factor()),
                &mut self.renderer,
                &mut self.clipboard,
                &mut self.debug,
            );
        }

        solstice::Renderer::clear(
            &mut self.ctx,
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

        let _mouse_interaction = self.renderer.backend_mut().draw(
            &mut self.ctx,
            &self.viewport,
            self.state.primitive(),
            &self.debug.overlay(),
        );
    }
}
