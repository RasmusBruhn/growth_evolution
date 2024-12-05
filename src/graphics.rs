use crate::render;
use wgpu::util::DeviceExt;

const INV_SQRT_3: f64 = 0.5773502691896257645091487805019574556476017512701268760186023264;

/// All settings for rendering
#[derive(Clone, Copy, Debug)]
pub struct Settings {
    /// The color of the background
    pub background: wgpu::Color,
}

/// A complete state for rendering
pub struct State {
    /// All of the settings for rendering
    settings: Settings,
    /// All pipelines used for rendering
    pipelines: Pipelines,
    /// The buffers for drawing hexagons
    buffers_hex: BuffersHex,
}

impl State {
    /// Constructs a new graphics state
    ///
    /// # Parameters
    ///
    /// background: The color of the background
    pub fn new(render_state: &render::RenderState, settings: Settings) -> Self {
        // Create pipelines
        let pipelines = Pipelines::new(render_state);

        // Create the hex buffers
        let buffers_hex = BuffersHex::new(render_state);

        return Self {
            settings,
            pipelines,
            buffers_hex,
        };
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
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.settings.background),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Set the pipeline for fill
            self.pipelines.set(&mut render_pass);

            // Set vertices for a single hexagon
            self.buffers_hex.set(&mut render_pass);

            // Draw
            render_pass.draw_indexed(0..Vertex::COUNT_INDEX_BULK_HEX as u32, 0, 0..1);
        }

        // Submit
        render_state
            .get_queue()
            .submit(std::iter::once(encoder.finish()));
    }
}

/// Holds all render pipelines
struct Pipelines {
    /// The render pipeline for filling
    fill: wgpu::RenderPipeline,
}

impl Pipelines {
    /// Constructs a new set of render pipelines
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn new(render_state: &render::RenderState) -> Self {
        // Create the shader
        let shader = wgpu::include_wgsl!("shader.wgsl");
        let shader = render_state.get_device().create_shader_module(shader);

        // Create render pipeline
        let layout =
            render_state
                .get_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Pipeline Layout Descriptor"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        // Create the fill pipeline
        let fill =
            render_state
                .get_device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline Fill"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        buffers: &[Vertex::desc_hex()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: render_state.get_config().format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                    cache: None,
                });

        Self { fill }
    }

    /// Sets the correct pipeline for the render pass
    ///
    /// # Parameters
    ///
    /// render_pass: The render pass to draw to
    fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.fill);
    }
}

/// Holds GPU buffers for the vertex data to draw a single hexagon
struct BuffersHex {
    /// The buffer holding all six vertices of the hex
    vertices: wgpu::Buffer,
    /// The 12 indices describing all 4 triangles of the hex
    indices_bulk: wgpu::Buffer,
}

impl BuffersHex {
    /// Creates a new set of hexagon buffers
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn new(render_state: &render::RenderState) -> Self {
        // Create the vertices
        let vertices =
            render_state
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Hex Vertex Buffer"),
                    contents: bytemuck::cast_slice(&Vertex::vertices_hex()),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        // Create the indices for the bulk
        let indices_bulk =
            render_state
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Hex Bulk Index Buffer"),
                    contents: bytemuck::cast_slice(&Vertex::indices_bulk_hex()),
                    usage: wgpu::BufferUsages::INDEX,
                });

        Self {
            vertices,
            indices_bulk,
        }
    }

    /// Sets the hexagon vertex information for the given render pass
    ///
    /// # Parameters
    ///
    /// render_pass: The render pass to set the vertex info for
    fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        // Set the vertex buffer
        render_pass.set_vertex_buffer(0, self.vertices.slice(..));

        // Set the index buffer and return the number of indices
        render_pass.set_index_buffer(self.indices_bulk.slice(..), wgpu::IndexFormat::Uint16);
    }
}

/// Describes a single vertex in the gpu
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    /// The position in the plane
    position: [f32; 2],
}

impl Vertex {
    const COUNT_INDEX_BULK_HEX: usize = 12;
    const COUNT_VERTEX_BULK_HEX: usize = 6;

    /// Gets the memory description of a hex vertex
    fn desc_hex() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }
    }

    /// Generates vertices for one hexagon
    fn vertices_hex() -> [Self; Self::COUNT_VERTEX_BULK_HEX] {
        [
            Self {
                position: [INV_SQRT_3 as f32, 0.0],
            },
            Self {
                position: [0.5 * INV_SQRT_3 as f32, 0.5],
            },
            Self {
                position: [-0.5 * INV_SQRT_3 as f32, 0.5],
            },
            Self {
                position: [-INV_SQRT_3 as f32, 0.0],
            },
            Self {
                position: [-0.5 * INV_SQRT_3 as f32, -0.5],
            },
            Self {
                position: [0.5 * INV_SQRT_3 as f32, -0.5],
            },
        ]
    }

    /// Generates indices for the vertices for the bulk of a hexagon
    const fn indices_bulk_hex() -> [u16; Self::COUNT_INDEX_BULK_HEX] {
        [2, 3, 4, 2, 4, 5, 1, 2, 5, 0, 1, 5]
    }
}
