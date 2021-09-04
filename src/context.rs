use crate::{
    audio,
    conf::Conf,
    filesystem::Filesystem,
    graphics,
    input::{input_handler::InputHandler, KeyboardContext, MouseContext},
    timer::TimeContext,
};

pub struct Context {
    pub filesystem: Filesystem,
    pub audio_context: audio::AudioContext,
    pub gfx_context: graphics::GraphicsContext,
    pub mouse_context: MouseContext,
    pub keyboard_context: KeyboardContext,
    pub timer_context: TimeContext,
    pub quad_ctx: miniquad::Context,
}

impl Context {
    pub(crate) fn new(mut quad_ctx: miniquad::Context, conf: Conf) -> Context {
        let input_handler = InputHandler::new();

        Context {
            filesystem: Filesystem::new(&conf),
            gfx_context: graphics::GraphicsContext::new(&mut quad_ctx),
            audio_context: audio::AudioContext::new(),
            mouse_context: MouseContext::new(input_handler),
            keyboard_context: KeyboardContext::new(),
            timer_context: TimeContext::new(),
            quad_ctx,
        }
    }

    pub(crate) fn framebuffer(&mut self) -> Option<miniquad::RenderPass> {
        self.gfx_context
            .canvas
            .as_ref()
            .map(|canvas| canvas.offscreen_pass.clone())
    }
}
