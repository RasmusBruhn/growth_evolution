use crate::render;

/// A complete state for rendering
pub struct State {
    /// The color of the background
    background: wgpu::Color,
}

impl State {
    /// Constructs a new graphics state
    ///
    /// # Parameters
    ///
    /// background: The color of the background
    pub fn new(background: wgpu::Color) -> Self {
        return Self { background };
    }

    /// Renders the state onto the given view
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// view: The texture view to render to
    pub fn render(&self, render_state: &render::RenderState, view: &wgpu::TextureView) {
        // Create the encoder
        let mut encoder =
            render_state
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });

        // Initialize the render pass
        {
            let mut _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.background),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        // Submit
        render_state
            .get_queue()
            .submit(std::iter::once(encoder.finish()));
    }
}
