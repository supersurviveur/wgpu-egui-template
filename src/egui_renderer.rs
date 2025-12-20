use egui::{Context, PaintCallbackInfo};
use egui_wgpu::{CallbackResources, CallbackTrait, Renderer, RendererOptions, ScreenDescriptor};
use egui_winit::State;
use wgpu::{CommandEncoder, Device, Queue, RenderPass, TextureFormat, TextureView};
use winit::{event::WindowEvent, window::Window};

pub struct EguiState {
    pub state: State,
    pub renderer: Renderer,
    pub frame_started: bool,
    pub color_format: TextureFormat,
}

impl EguiState {
    pub fn context(&self) -> &Context {
        self.state.egui_ctx()
    }

    pub fn new(device: &Device, output_color_format: TextureFormat, window: &Window) -> EguiState {
        let egui_context = Context::default();

        let egui_state = egui_winit::State::new(
            egui_context,
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );

        let options = RendererOptions {
            ..Default::default()
        };
        let egui_renderer = Renderer::new(device, output_color_format, options);

        EguiState {
            state: egui_state,
            renderer: egui_renderer,
            frame_started: false,
            color_format: output_color_format,
        }
    }

    pub fn resize(&mut self, _device: &Device, _width: u32, _height: u32) {}

    pub fn handle_event(&mut self, window: &Window, event: &WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    pub fn begin_frame(&mut self, window: &Window) {
        let input = self.state.take_egui_input(window);
        self.state.egui_ctx().begin_pass(input);
        self.frame_started = true;
    }

    pub fn end_frame_and_draw(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        window: &Window,
        window_surface_view: &TextureView,
        screen_descriptor: ScreenDescriptor,
    ) {
        if !self.frame_started {
            panic!("begin_frame must be called before end_frame_and_draw can be called!");
        }

        let full_output = self.state.egui_ctx().end_pass();

        self.state
            .handle_platform_output(window, full_output.platform_output);

        let tris = self
            .state
            .egui_ctx()
            .tessellate(full_output.shapes, self.state.egui_ctx().pixels_per_point());
        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }
        self.renderer
            .update_buffers(device, queue, encoder, &tris, &screen_descriptor);
        let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: window_surface_view,
                resolve_target: None,
                ops: egui_wgpu::wgpu::Operations {
                    load: egui_wgpu::wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            label: Some("egui main render pass"),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.renderer
            .render(&mut rpass.forget_lifetime(), &tris, &screen_descriptor);
        for x in &full_output.textures_delta.free {
            self.renderer.free_texture(x)
        }

        self.frame_started = false;
    }
}

pub struct CallbackFn<P>
where
    P: Fn(PaintCallbackInfo, &mut RenderPass<'static>, &CallbackResources),
{
    pub paint_fn: P,
}

impl<P> CallbackTrait for CallbackFn<P>
where
    P: Fn(PaintCallbackInfo, &mut RenderPass<'static>, &CallbackResources)
        + std::marker::Sync
        + std::marker::Send,
{
    fn paint(
        &self,
        info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &egui_wgpu::CallbackResources,
    ) {
        (self.paint_fn)(info, render_pass, callback_resources)
    }
}
