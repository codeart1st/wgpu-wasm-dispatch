use std::sync::Arc;

use log::info;

const PREFERRED_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
const PREFERRED_ALPHA_MODE: wgpu::CompositeAlphaMode = wgpu::CompositeAlphaMode::PreMultiplied;

pub struct Renderer {
  /// wgpu device queue pair
  pub device_queue: (Arc<wgpu::Device>, Arc<wgpu::Queue>),

  /// preferred texutre format of surface
  pub texture_format: wgpu::TextureFormat,
}

pub trait ToSurface {
  /// Creates a surface from a raw window handle.
  ///
  /// If the specified display and window handle are not supported by any of the backends, then the surface
  /// will not be supported by any adapters.
  ///
  /// # Safety
  ///
  /// - Raw Window Handle must be a valid object to create a surface upon and
  ///   must remain valid for the lifetime of the returned surface.
  /// - If not called on the main thread, metal backend will panic.
  unsafe fn create_surface(
    &self,
    instance: &wgpu::Instance,
  ) -> Result<wgpu::Surface, wgpu::CreateSurfaceError>;
}

impl Renderer {
  pub async fn new<W: ToSurface>(window: &W, (width, height): (u32, u32)) -> Self {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::all()),
      dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default(),
    });

    let swapchain;
    unsafe {
      swapchain = match window.create_surface(&instance) {
        Ok(surface) => surface,
        Err(err) => {
          panic!("{}", err.to_string())
        }
      }
    };

    info!("surface: {:?}", &swapchain);

    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::util::power_preference_from_env()
          .unwrap_or(wgpu::PowerPreference::HighPerformance),
        force_fallback_adapter: false,
        compatible_surface: Some(&swapchain),
      })
      .await
      .expect("Adapter not created.");

    info!("adapter: {:?}", &adapter);

    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          label: None,
          features: wgpu::Features::default(),
          limits: wgpu::Limits::default(),
        },
        None,
      )
      .await
      .expect("Device can't be created.");

    info!("device: {:?}", device);

    let device = Arc::new(device);
    let queue = Arc::new(queue);

    let swapchain_capabilities = swapchain.get_capabilities(&adapter);

    info!(
      "supported surface formats: {:?}",
      swapchain_capabilities.formats
    );

    let texture_format = if swapchain_capabilities
      .formats
      .contains(&PREFERRED_TEXTURE_FORMAT)
    {
      PREFERRED_TEXTURE_FORMAT
    } else {
      swapchain_capabilities
        .formats
        .first()
        .expect("Can't get texture format for surface.")
        .to_owned()
    };

    info!(
      "supported alpha modes: {:?}",
      swapchain_capabilities.alpha_modes
    );

    let _alpha_mode = if swapchain_capabilities
      .alpha_modes
      .contains(&PREFERRED_ALPHA_MODE)
    {
      PREFERRED_ALPHA_MODE
    } else {
      swapchain_capabilities
        .alpha_modes
        .first()
        .expect("Can't get present mode for surface.")
        .to_owned()
    };

    let surface_config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: texture_format,
      width,
      height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
      view_formats: vec![],
    };

    swapchain.configure(&device, &surface_config);

    Self {
      device_queue: (device, queue),
      texture_format,
    }
  }
}
