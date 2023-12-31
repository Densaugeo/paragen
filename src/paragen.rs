use std::sync::Mutex;
use std::sync::atomic::{Ordering, AtomicU32};

pub mod prelude {
  pub use paragen_macros::paragen;
  pub use crate::GLTF;
  pub use crate::Scene;
  pub use crate::Node;
  pub use crate::ErrorCode;
}

pub static MUTEX_TEST: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static POINTER: AtomicU32 = AtomicU32::new(0);
static SIZE: AtomicU32 = AtomicU32::new(0);

// WebAssembly is rumored to always be 32 bit, so assume that's the pointer size
#[no_mangle]
pub extern "C" fn pointer() -> i32 {
  POINTER.load(Ordering::Relaxed) as i32
}

// WebAssembly is rumored to always be 32 bit, so assume that's the pointer size
#[no_mangle]
pub extern "C" fn size() -> i32 {
  SIZE.load(Ordering::Relaxed) as i32
}

// These error codes are return from WebAssembly functions, so must use a
// WebAssembly variable type
#[repr(i32)]
pub enum ErrorCode {
    None = 0,
    Mutex = 1,
    Generation = 2,
}

struct DryRunWriter {
  bytes_written: usize,
}

impl DryRunWriter {
  fn new() -> Self {
    Self { bytes_written: 0 }
  }
}

impl std::io::Write for DryRunWriter {
  fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
    self.bytes_written += buf.len();
    Ok(buf.len())
  }
  
  fn flush(&mut self) -> Result<(), std::io::Error> {
    Ok(())
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Asset {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub copyright: String,
  
  #[serde(skip_serializing_if = "String::is_empty")]
  pub generator: String,
  
  // Don't skip if empty...this field is mandatory per GLTF spec!
  pub version: String,
  
  #[serde(skip_serializing_if = "String::is_empty")]
  #[serde(rename = "minVersion")]
  pub min_version: String,
  
  // pub extensions: ??,
  
  // In the .gltf spec, but will have to wait for later
  //pub extra: ??,
}

impl Asset {
  pub fn new() -> Self {
    Self {
      copyright: String::from(""),
      generator: String::from("Paragen v0.1.0"),
      version: String::from("2.0"),
      min_version: String::from("2.0"),
    }
  }
}

#[derive(Clone, serde::Serialize)]
pub struct GLTF {
  // Don't skip if empty...this field is mandatory per GLTF spec!
  pub asset: Asset,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub scene: Option<u32>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub scenes: Vec<Scene>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub nodes: Vec<Node>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub materials: Vec<Material>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub meshes: Vec<Mesh>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub accessors: Vec<Accessor>,
  
  #[serde(rename = "bufferViews")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub buffer_views: Vec<BufferView>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub buffers: Vec<Buffer>,
  
  // In the .gltf spec, but will have to wait for later
  /*pub animations: ??
  pub asset: ??
  pub extensionsUsed: ??
  pub extensionsRequired: ??
  pub cameras: ??
  pub images: ??
  pub samplers: ??
  pub skins: ??
  pub textures: ??
  pub extensions: ??
  pub extras: ??*/
}

impl GLTF {
  pub fn new() -> Self {
    Self {
      asset: Asset::new(),
      nodes: Vec::new(),
      materials: Vec::new(),
      scene: None,
      scenes: Vec::new(),
      meshes: Vec::new(),
      accessors: Vec::new(),
      buffer_views: Vec::new(),
      buffers: Vec::new(),
    }
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Scene {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub nodes: Vec<u32>,
  
  //pub extensions: Vec<??>,
  
  // In the .gltf spec but not currently used:
  //pub extras: Vec<A JSON-serializable struct>,
}

impl Scene {
  pub fn new() -> Self {
    Self { name: String::from(""), nodes: Vec::new() }
  }
}

#[derive(Clone, PartialEq)]
#[derive(serde_tuple::Serialize_tuple)]
pub struct Translation {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Translation {
  pub fn new() -> Self { Self { x: 0.0, y: 0.0, z: 0.0 } }
  pub fn is_default(&self) -> bool { *self == Self::new() }
}

#[derive(Clone, PartialEq)]
#[derive(serde_tuple::Serialize_tuple)]
pub struct Rotation {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
}

impl Rotation {
  pub fn new() -> Self { Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 } }
  pub fn is_default(&self) -> bool { *self == Self::new() }
}

#[derive(Clone, PartialEq)]
#[derive(serde_tuple::Serialize_tuple)]
pub struct Scale {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Scale {
  pub fn new() -> Self { Self { x: 1.0, y: 1.0, z: 1.0 } }
  pub fn is_default(&self) -> bool { *self == Self::new() }
}

#[derive(Clone, serde::Serialize)]
pub struct Node {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mesh: Option<u32>,
  
  #[serde(rename = "translation")]
  #[serde(skip_serializing_if = "Translation::is_default")]
  pub t: Translation,
  
  #[serde(rename = "rotation")]
  #[serde(skip_serializing_if = "Rotation::is_default")]
  pub r: Rotation,
  
  #[serde(rename = "scale")]
  #[serde(skip_serializing_if = "Scale::is_default")]
  pub s: Scale,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub children: Vec<u32>,
  
  //pub mesh: ??,
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub camera: ??,
  pub skin: ??,
  pub matrix: ??,
  pub weights: ??,
  pub extras: ??,*/
}

impl Node {
  pub fn new() -> Self {
    Self {
      name: String::from(""),
      mesh: None,
      t: Translation::new(),
      r: Rotation::new(),
      s: Scale::new(),
      children: Vec::new(),
    }
  }
}

#[derive(Clone, PartialEq, serde::Serialize)]
pub enum AlphaMode {
  OPAQUE,
  MASK,
  BLEND,
}

#[derive(Clone, PartialEq)]
#[derive(serde_tuple::Serialize_tuple)]
pub struct Color4 {
  pub r: f64,
  pub g: f64,
  pub b: f64,
  pub a: f64,
}

impl Color4 {
  pub fn new() -> Self { Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 } }
  pub fn is_default(&self) -> bool { *self == Self::new() }
}

#[derive(Clone, serde::Serialize)]
pub struct PBRMetallicRoughness {
  #[serde(rename = "baseColorFactor")]
  #[serde(skip_serializing_if = "Color4::is_default")]
  pub base_color_factor: Color4,
  
  #[serde(rename = "metallicFactor")]
  #[serde(skip_serializing_if = "is_default_metallic_factor")]
  pub metallic_factor: f64,
  
  #[serde(rename = "roughnessFactor")]
  #[serde(skip_serializing_if = "is_default_roughness_factor")]
  pub roughness_factor: f64,
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub extras: ??,
  pub metallicRoughnessTexture: ??,
  pub baseColorTexture: ??,
  */
}

impl PBRMetallicRoughness {
  pub fn new() -> Self {
    Self {
      base_color_factor: Color4::new(),
      metallic_factor: 1.0,
      roughness_factor: 1.0,
    }
  }
}

fn is_default_metallic_factor(value: &f64) -> bool {
  *value == 1.0
}

fn is_default_roughness_factor(value: &f64) -> bool {
  *value == 1.0
}

fn is_default_emissive_factor(value: &[f64; 3]) -> bool {
  *value == [0.0, 0.0, 0.0]
}

fn is_default_alpha_mode(value: &AlphaMode) -> bool {
  *value == AlphaMode::OPAQUE
}

fn is_default_alpha_cutoff(value: &f64) -> bool {
  *value == 0.5
}

fn is_default_double_sided(value: &bool) -> bool {
  *value == false
}

#[derive(Clone, serde::Serialize)]
pub struct Material {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  #[serde(rename = "emissiveFactor")]
  #[serde(skip_serializing_if = "is_default_emissive_factor")]
  pub emissive_factor: [f64; 3],
  
  #[serde(rename = "alphaMode")]
  #[serde(skip_serializing_if = "is_default_alpha_mode")]
  pub alpha_mode: AlphaMode,
  
  #[serde(rename = "alphaCutoff")]
  #[serde(skip_serializing_if = "is_default_alpha_cutoff")]
  pub alpha_cutoff: f64,
  
  #[serde(rename = "doubleSided")]
  #[serde(skip_serializing_if = "is_default_double_sided")]
  pub double_sided: bool,
  
  #[serde(rename = "pbrMetallicRoughness")]
  // Not sure how to skip serializing when unused for this one
  pub pbr_metallic_roughness: PBRMetallicRoughness,
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub extras: ??,
  pub normalTexture: ??,
  pub occlusionTexture: ??,
  pub emissiveTexture: ??,*/
}

impl Material {
  pub fn new() -> Self {
    Self {
      name: String::from(""),
      emissive_factor: [0.0, 0.0, 0.0],
      alpha_mode: AlphaMode::OPAQUE,
      alpha_cutoff: 0.5,
      double_sided: false,
      pbr_metallic_roughness: PBRMetallicRoughness::new(),
    }
  }
}

// The fields here are in the spec in section 3.7 - Concepts / Geometry,
// which took me a while to find
#[derive(Clone, serde::Serialize)]
pub struct Attributes {
  #[serde(rename = "COLOR_0")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub color_0: Option<u32>,
  
  #[serde(rename = "JOINTS_0")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub joints_0: Option<u32>,
  
  #[serde(rename = "NORMAL")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub normal: Option<u32>,
  
  #[serde(rename = "POSITION")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub position: Option<u32>,
  
  #[serde(rename = "TANGENT")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tangent: Option<u32>,
  
  #[serde(rename = "TEXCOORD_0")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub texcoord_0: Option<u32>,
  
  #[serde(rename = "TEXCOORD_1")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub texcoord_1: Option<u32>,
  
  #[serde(rename = "TEXCOORD_2")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub texcoord_2: Option<u32>,
  
  #[serde(rename = "TEXCOORD_3")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub texcoord_3: Option<u32>,
  
  #[serde(rename = "WEIGHTS_0")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub weights_0: Option<u32>,
}

impl Attributes {
  pub fn new() -> Self {
    Self {
      color_0: None,
      joints_0: None,
      normal: None,
      position: None,
      tangent: None,
      texcoord_0: None,
      texcoord_1: None,
      texcoord_2: None,
      texcoord_3: None,
      weights_0: None,
    }
  }
}

#[derive(Clone, PartialEq, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum Mode {
  Points = 0,
  Lines = 1,
  LineLoop = 2,
  LineStrip = 3,
  Triangles = 4,
  TriangleStrip = 5,
  TriangleFan = 6,
}

fn is_default_mode(value: &Mode) -> bool {
  *value == Mode::Triangles
}

#[derive(Clone, serde::Serialize)]
pub struct MeshPrimitive {
  pub attributes: Attributes,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub indices: Option<u32>,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub material: Option<u32>,
  
  #[serde(skip_serializing_if = "is_default_mode")]
  pub mode: Mode, // Default is triangles
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub extras: ??,
  pub targets: ??,*/
}

impl MeshPrimitive {
  pub fn new() -> Self {
    Self {
      attributes: Attributes::new(),
      indices: None,
      material: None,
      mode: Mode::Triangles,
    }
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Mesh {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  // No serialization filter, this is required per spec
  pub primitives: Vec<MeshPrimitive>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub weights: Vec<f64>,
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub extras: ??,*/
}

impl Mesh {
  pub fn new() -> Self {
    Self {
      primitives: Vec::new(),
      weights: Vec::new(),
      name: String::from(""),
    }
  }
}

#[derive(Clone, PartialEq, serde_repr::Serialize_repr)]
#[repr(u16)]
pub enum ComponentType {
  Byte = 5120,
  UnsignedByte = 5121,
  Short = 5122,
  UnsignedShort = 5123,
  UnsignedInt = 5125,
  Float = 5126,
}

#[derive(Clone, serde::Serialize)]
pub enum Type {
  SCALAR,
  VEC2,
  VEC3,
  VEC4,
  MAT2,
  MAT3,
  MAT4,
}

#[derive(Clone, serde::Serialize)]
pub struct Accessor {
  // Next time I modify this, I want to try out:
  // #[serde(rename_all = "camelCase")]
  
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  #[serde(rename = "bufferView")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub buffer_view: Option<u32>,
  
  #[serde(rename = "byteOffset")]
  #[serde(skip_serializing_if = "is_default_byte_offset")]
  pub byte_offset: u32,
  
  #[serde(rename = "componentType")]
  pub component_type: ComponentType,
  
  #[serde(skip_serializing_if = "is_default_normalized")]
  pub normalized: bool,
  
  pub count: u32,
  
  #[serde(rename = "type")]
  pub type_: Type,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub max: Vec<f64>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub min: Vec<f64>,
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /* pub max: ??,
  pub min: ??,
  pub sparse: ??,
  pub extras: ??,*/
}

impl Accessor {
  pub fn new() -> Self {
    Self {
      name: String::from(""),
      buffer_view: None,
      byte_offset: 0,
      component_type: ComponentType::Byte,
      normalized: false,
      count: 0,
      type_: Type::SCALAR,
      min: Vec::new(),
      max: Vec::new(),
    }
  }
}

fn is_default_byte_offset(value: &u32) -> bool {
  *value == 0
}

fn is_default_normalized(value: &bool) -> bool {
  *value == false
}

#[derive(Clone, PartialEq, serde_repr::Serialize_repr)]
#[repr(u16)]
pub enum Target {
  ArrayBuffer = 34962,
  ElementArrayBuffer = 34963,
}

#[derive(Clone, serde::Serialize)]
pub struct BufferView {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  pub buffer: u32,
  
  #[serde(rename = "byteLength")]
  pub byte_length: u32,
  
  #[serde(rename = "byteOffset")]
  pub byte_offset: u32,
  
  #[serde(rename = "byteStride")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub byte_stride: Option<u32>,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub target: Option<Target>,
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub extras: ??,*/
}

impl BufferView {
  pub fn new() -> Self {
    Self {
      name: String::from(""),
      buffer: 0,
      byte_length: 0,
      byte_offset: 0,
      byte_stride: None,
      target: None,
    }
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Buffer {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  #[serde(rename = "byteLength")]
  pub byte_length: u32,
  
  #[serde(skip_serializing_if = "String::is_empty")]
  pub uri: String,
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub extras: ??,*/
}

impl Buffer {
  pub fn new() -> Self {
    Self {
      name: String::from(""),
      byte_length: 0,
      uri: String::from(""),
    }
  }
}

pub fn write_gltf(buffer: &mut Vec<u8>, gltf: GLTF) {
  let mut dry_run_writer = DryRunWriter::new();
  serde_json::ser::to_writer_pretty(&mut dry_run_writer, &gltf).unwrap();
  let space_required = dry_run_writer.bytes_written;
  
  buffer.reserve_exact(space_required);
  serde_json::ser::to_writer_pretty(&mut (*buffer), &gltf).unwrap();
  buffer.shrink_to_fit();
  
  POINTER.store(buffer.as_ptr() as u32, Ordering::Relaxed);
  SIZE.store(buffer.len() as u32, Ordering::Relaxed);
}
