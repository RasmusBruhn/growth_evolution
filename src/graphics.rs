use crate::{constants::INV_SQRT_3, render, types};
use wgpu::util::DeviceExt;

/// All settings for rendering
#[derive(Clone, Copy, Debug)]
pub struct Settings {
    /// The color of the background
    pub color_background: wgpu::Color,
    /// The color of the edges
    pub color_edge: wgpu::Color,
}

/// A complete state for rendering
pub struct State {
    /// All of the settings for rendering
    settings: Settings,
    /// All pipelines used for rendering
    pipelines: Pipelines,
    /// All uniform buffers used fgiven settings to the shader
    uniforms: Uniforms,
    /// The buffers for drawing hexagons
    buffers_hex: BuffersHex,
}

impl State {
    /// Constructs a new graphics state
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// settings: The settings for this state
    pub fn new(render_state: &render::RenderState, settings: Settings) -> Self {
        // Create pipelines
        let pipelines = Pipelines::new(render_state);

        // Create the uniforms
        let uniforms = Uniforms::new(render_state);
        uniforms.write_edge_color(render_state, &settings.color_edge);

        // Create the hex buffers
        let buffers_hex = BuffersHex::new(render_state);

        return Self {
            settings,
            pipelines,
            uniforms,
            buffers_hex,
        };
    }

    /// Sets the color of the background
    ///
    /// # Parameters
    ///
    /// color: The color to set for the background
    pub fn set_color_background(&mut self, color: wgpu::Color) {
        self.settings.color_background = color;
    }

    /// Sets the color of the edges
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// color: The new color for the edges
    pub fn set_color_edge(&mut self, render_state: &render::RenderState, color: wgpu::Color) {
        self.settings.color_edge = color;

        // Update the gpu data
        self.uniforms
            .write_edge_color(render_state, &self.settings.color_edge);
    }

    /// Renders the state onto the given view
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// view: The texture view to render to
    ///
    /// transform: The transform to go from world to screen coordinates
    pub fn render(
        &self,
        render_state: &render::RenderState,
        view: &wgpu::TextureView,
        transform: &types::Transform2D,
    ) {
        self.render_single(render_state, view, transform, DrawMode::Fill);
        self.render_single(render_state, view, transform, DrawMode::Edge);
    }

    /// Renders the state onto the given view
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// view: The texture view to render to
    ///
    /// transform: The transform to go from world to screen coordinates
    ///
    /// draw_mode: Describes wether to draw with fill or outline mode
    fn render_single(
        &self,
        render_state: &render::RenderState,
        view: &wgpu::TextureView,
        transform: &types::Transform2D,
        draw_mode: DrawMode,
    ) {
        // Set the draw mode and transform
        self.uniforms.write_draw_mode(render_state, draw_mode);
        self.uniforms.write_transform(render_state, transform);

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
                        load: match draw_mode {
                            DrawMode::Fill => wgpu::LoadOp::Clear(self.settings.color_background),
                            DrawMode::Edge => wgpu::LoadOp::Load,
                        },
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Set the pipeline for fill
            self.pipelines.set(&mut render_pass, draw_mode);

            // Set the main uniforms
            self.uniforms.set(&mut render_pass);

            // Set vertices for a single hexagon
            let index_count = self.buffers_hex.set(&mut render_pass, draw_mode);

            // Draw
            render_pass.draw_indexed(0..index_count, 0, 0..1);
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
    /// The render pipeline for the outline
    outline: wgpu::RenderPipeline,
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
                    bind_group_layouts: &[&Uniforms::bind_group_layout(render_state)],
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

        // Create the outline pipeline
        let outline =
            render_state
                .get_device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline Outline"),
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
                        topology: wgpu::PrimitiveTopology::LineStrip,
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

        Self { fill, outline }
    }

    /// Sets the correct pipeline for the render pass
    ///
    /// # Parameters
    ///
    /// render_pass: The render pass to draw to
    ///
    /// draw_mode: Describes wether to draw with fill or outline mode
    fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, draw_mode: DrawMode) {
        match draw_mode {
            DrawMode::Fill => render_pass.set_pipeline(&self.fill),
            DrawMode::Edge => render_pass.set_pipeline(&self.outline),
        };
    }
}

/// Holds all of the global uniforms for the shader and the bind group for them
struct Uniforms {
    transform: wgpu::Buffer,
    /// The draw mode buffer
    draw_mode: wgpu::Buffer,
    /// The edge color buffer
    edge_color: wgpu::Buffer,
    /// The bind group for all uniforms
    bind_group: wgpu::BindGroup,
}

impl Uniforms {
    /// Creates a new set of uniforms for the gpu
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn new(render_state: &render::RenderState) -> Self {
        // Create transform buffer
        let transform = render_state
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Transform Buffer"),
                size: (std::mem::size_of::<f32>() * 4) as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        // Create draw mode buffer
        let draw_mode = render_state
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Draw Mode Buffer"),
                size: std::mem::size_of::<u32>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        // Create edge color buffer
        let edge_color = render_state
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Draw Mode Buffer"),
                size: (std::mem::size_of::<f32>() * 4) as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        // Create bind group for the uniforms
        let bind_group = render_state
            .get_device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Bind Group Uniforms"),
                layout: &Self::bind_group_layout(render_state),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: transform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: draw_mode.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: edge_color.as_entire_binding(),
                    },
                ],
            });

        Self {
            transform,
            draw_mode,
            edge_color,
            bind_group,
        }
    }

    /// Update the transform, this must be run once before the first rendering as it is not initialized
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// transform: The transform to apply to all vertices going from world coordinates to screen coordinates
    fn write_transform(&self, render_state: &render::RenderState, transform: &types::Transform2D) {
        render_state.get_queue().write_buffer(
            &self.transform,
            0,
            bytemuck::cast_slice(&[transform.get_data_center_transform()]),
        );
    }

    /// Update the draw mode, this must be run once before the first rendering as it is not initialized
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// draw_mode: The draw mode to tell the shader if it is drawing edges or filling
    fn write_draw_mode(&self, render_state: &render::RenderState, draw_mode: DrawMode) {
        render_state.get_queue().write_buffer(
            &self.draw_mode,
            0,
            bytemuck::cast_slice(&[draw_mode.get_data()]),
        );
    }

    /// Update the edge color, this must be run once before the first rendering as it is not initialized
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// draw_mode: The draw mode to tell the shader if it is drawing edges or filling
    fn write_edge_color(&self, render_state: &render::RenderState, edge_color: &wgpu::Color) {
        render_state.get_queue().write_buffer(
            &self.edge_color,
            0,
            bytemuck::cast_slice(&[get_color_data(edge_color)]),
        );
    }

    /// Binds the uniforms to the given render pass
    ///
    /// # Parameters
    ///
    /// render_pass: The render pass to draw to
    fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
    }

    /// Creates the bind group layout for a set of uniforms
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn bind_group_layout(render_state: &render::RenderState) -> wgpu::BindGroupLayout {
        render_state
            .get_device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Uniform Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            })
    }
}

/// Holds GPU buffers for the vertex data to draw a single hexagon
struct BuffersHex {
    /// The buffer holding all six vertices of the hex
    vertices: wgpu::Buffer,
    /// The 12 indices describing all 4 triangles of the hex
    indices_bulk: wgpu::Buffer,
    /// The 7 indices describing all 6 edge pieces of the hex
    indices_edge: wgpu::Buffer,
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

        // Create the indices for the bulk
        let indices_edge =
            render_state
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Hex Edge Index Buffer"),
                    contents: bytemuck::cast_slice(&Vertex::indices_edge_hex()),
                    usage: wgpu::BufferUsages::INDEX,
                });

        Self {
            vertices,
            indices_bulk,
            indices_edge,
        }
    }

    /// Sets the hexagon vertex information for the given render pass
    ///
    /// # Parameters
    ///
    /// render_pass: The render pass to set the vertex info for
    ///
    /// draw_mode: The mode describing whether to draw in fill or edge mode
    fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, draw_mode: DrawMode) -> u32 {
        // Set the vertex buffer
        render_pass.set_vertex_buffer(0, self.vertices.slice(..));

        // Set the index buffer and return the number of indices
        return match draw_mode {
            DrawMode::Fill => {
                render_pass
                    .set_index_buffer(self.indices_bulk.slice(..), wgpu::IndexFormat::Uint16);
                Vertex::COUNT_INDEX_BULK_HEX as u32
            }
            DrawMode::Edge => {
                render_pass
                    .set_index_buffer(self.indices_edge.slice(..), wgpu::IndexFormat::Uint16);
                Vertex::COUNT_INDEX_EDGE_HEX as u32
            }
        };
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
    const COUNT_VERTEX_HEX: usize = 6;
    const COUNT_INDEX_BULK_HEX: usize = 12;
    const COUNT_INDEX_EDGE_HEX: usize = 7;

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
    fn vertices_hex() -> [Self; Self::COUNT_VERTEX_HEX] {
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

    /// Generates indices for the vertices for the edge of a hexagon
    const fn indices_edge_hex() -> [u16; Self::COUNT_INDEX_EDGE_HEX] {
        [0, 1, 2, 3, 4, 5, 0]
    }
}

/// Describes if rendering should be done on the filling or outline of hexagons
#[derive(Copy, Clone, Debug)]
enum DrawMode {
    Fill,
    Edge,
}

impl DrawMode {
    /// Retrieves the code for the gpu for this mode
    fn get_data(&self) -> u32 {
        match *self {
            Self::Fill => 0,
            Self::Edge => 1,
        }
    }
}

fn get_color_data(color: &wgpu::Color) -> [f32; 4] {
    return [
        color.r as f32,
        color.g as f32,
        color.b as f32,
        color.a as f32,
    ];
}
