use sdl2;
use sdl2::Sdl;
use sdl2::render::Renderer;

/// Simple SDL2 abstraction 
pub struct System<'a> {
    pub sdl_context : Sdl,
    pub renderer : Renderer<'a>
}

impl<'a> System<'a> {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let ctx = sdl2::init().unwrap();
        let video_ctx = ctx.video().unwrap();

        let window = video_ctx.window(title, width, height)
                            .position_centered()
                            .opengl()
                            .build()
                            .unwrap();

        let renderer = window.renderer()
                             .accelerated()
                             .build()
                             .unwrap();
        System {
            sdl_context : ctx,
            renderer : renderer
        }
    }
}