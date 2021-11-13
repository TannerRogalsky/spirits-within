use iced_solstice::{Backend, Renderer, Settings, Viewport};
use iced_winit::{conversion, program, winit::dpi::PhysicalPosition, Clipboard, Debug, Size};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
}

const MOUSE_DOWN_EVENT: &'static str = "mousedown";
const MOUSE_MOVE_EVENT: &'static str = "mousemove";
const MOUSE_UP_EVENT: &'static str = "mouseup";
const MOUSE_WHEEL_EVENT: &'static str = "wheel";

#[allow(unused)]
struct Closures {
    canvas: web_sys::HtmlCanvasElement,
    mouse_down: Closure<dyn FnMut(web_sys::MouseEvent)>,
    mouse_move: Closure<dyn FnMut(web_sys::MouseEvent)>,
    mouse_up: Closure<dyn FnMut(web_sys::MouseEvent)>,
    mouse_wheel: Closure<dyn FnMut(web_sys::WheelEvent)>,
}

impl Drop for Closures {
    fn drop(&mut self) {
        use wasm_bindgen::JsCast;
        self.canvas
            .remove_event_listener_with_callback(
                MOUSE_DOWN_EVENT,
                self.mouse_down.as_ref().unchecked_ref(),
            )
            .unwrap();
        self.canvas
            .remove_event_listener_with_callback(
                MOUSE_MOVE_EVENT,
                self.mouse_move.as_ref().unchecked_ref(),
            )
            .unwrap();
        self.canvas
            .remove_event_listener_with_callback(
                MOUSE_UP_EVENT,
                self.mouse_up.as_ref().unchecked_ref(),
            )
            .unwrap();
    }
}

#[wasm_bindgen]
pub struct Application {
    #[allow(unused)]
    closures: Closures,
    ctx: solstice::Context,
    d2: renderer::Renderer,
    events: crossbeam_channel::Receiver<iced_winit::event::Event>,

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
        let d2 = renderer::Renderer {
            d2: renderer::Graphics::new(
                &mut ctx,
                physical_size.width as _,
                physical_size.height as _,
            )
            .unwrap(),
        };

        let mut debug = Debug::new();
        let mut renderer = Renderer::new(Backend::new(&mut ctx, Settings::default()));
        let clipboard = Clipboard::new();

        let cursor_position = PhysicalPosition::new(-1.0, -1.0);
        // let modifiers = ModifiersState::default();

        let controls = crate::Application::new({
            let rand: web_sys::Crypto = web_sys::window().unwrap().crypto()?;
            let mut seed = [0; 8];
            rand.get_random_values_with_u8_array(&mut seed)?;
            u64::from_ne_bytes(seed)
        });

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
                    MOUSE_DOWN_EVENT,
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
                    MOUSE_MOVE_EVENT,
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
                    MOUSE_UP_EVENT,
                    closure.as_ref().unchecked_ref(),
                )?;
                closure
            };
            let mouse_wheel = {
                let events_sx = events_sx.clone();
                let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
                    log::trace!("mouse wheel");
                    let x = event.delta_x() as _;
                    let y = -event.delta_y() as _;
                    let delta = match event.delta_mode() {
                        web_sys::WheelEvent::DOM_DELTA_PIXEL => mouse::ScrollDelta::Pixels { x, y },
                        web_sys::WheelEvent::DOM_DELTA_LINE => mouse::ScrollDelta::Lines { x, y },
                        web_sys::WheelEvent::DOM_DELTA_PAGE => return,
                        _ => return,
                    };
                    events_sx
                        .send(Event::Mouse(mouse::Event::WheelScrolled { delta }))
                        .unwrap();
                }) as Box<dyn FnMut(_)>);
                canvas.add_event_listener_with_callback(
                    MOUSE_WHEEL_EVENT,
                    closure.as_ref().unchecked_ref(),
                )?;
                closure
            };

            Closures {
                canvas,
                mouse_down,
                mouse_move,
                mouse_up,
                mouse_wheel,
            }
        };

        Ok(Self {
            ctx,
            d2,
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

    pub fn update(&mut self, t: f32) {
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

        const DURATION: f32 = 10.;
        let t = (t / 1000.) % DURATION / DURATION;
        self.d2.draw(&mut self.ctx, t);

        let _mouse_interaction = self.renderer.backend_mut().draw(
            &mut self.ctx,
            &self.viewport,
            self.state.primitive(),
            &self.debug.overlay(),
        );
    }
}
