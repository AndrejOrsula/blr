//! Small, public, std-only authoring scaffold for the browser scene demo.
//!
//! This is a deliberately limited command builder, not the native `blr` crate. The host
//! validates the same limits again before passing these recipes to Blender.

use std::fmt::{self, Write as _};

const MAX_TEXT_BYTES: usize = 256 * 1024;
const MAX_OBJECTS: usize = 64;
const MAX_VERTICES: usize = 100_000;
const MAX_TRIANGLES: usize = 200_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Cube,
    Plane,
    UvSphere,
    Cylinder,
    Cone,
    Torus,
}

impl Kind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Cube => "cube",
            Self::Plane => "plane",
            Self::UvSphere => "uv_sphere",
            Self::Cylinder => "cylinder",
            Self::Cone => "cone",
            Self::Torus => "torus",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Object {
    kind: Kind,
    name: String,
    location: [f64; 3],
    rotation: [f64; 3],
    scale: [f64; 3],
    color: [f64; 4],
    params: Vec<(&'static str, f64)>,
    counts: Vec<(&'static str, u32)>,
}

impl Object {
    fn new(kind: Kind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
            location: [0.0; 3],
            rotation: [0.0; 3],
            scale: [1.0; 3],
            color: [1.0, 1.0, 1.0, 1.0],
            params: Vec::new(),
            counts: Vec::new(),
        }
    }

    pub fn cube(name: impl Into<String>, size: f64) -> Self {
        Self::new(Kind::Cube, name).param("size", size)
    }

    pub fn plane(name: impl Into<String>, size: f64) -> Self {
        Self::new(Kind::Plane, name).param("size", size)
    }

    pub fn sphere(name: impl Into<String>, radius: f64) -> Self {
        Self::new(Kind::UvSphere, name).param("radius", radius)
    }

    pub fn cylinder(name: impl Into<String>, radius: f64, depth: f64) -> Self {
        Self::new(Kind::Cylinder, name)
            .param("radius", radius)
            .param("depth", depth)
    }

    pub fn cone(name: impl Into<String>, radius: f64, depth: f64) -> Self {
        Self::new(Kind::Cone, name)
            .param("radius1", radius)
            .param("depth", depth)
    }

    pub fn torus(name: impl Into<String>, major_radius: f64, minor_radius: f64) -> Self {
        Self::new(Kind::Torus, name)
            .param("major_radius", major_radius)
            .param("minor_radius", minor_radius)
    }

    pub fn at(mut self, location: [f64; 3]) -> Self {
        self.location = location;
        self
    }

    /// Euler rotation in radians, in Blender's XYZ order.
    pub fn rotated(mut self, rotation: [f64; 3]) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn scaled(mut self, scale: [f64; 3]) -> Self {
        self.scale = scale;
        self
    }

    /// Linear RGBA color. Each channel must be in `0.0..=1.0`.
    pub fn colored(mut self, color: [f64; 4]) -> Self {
        self.color = color;
        self
    }

    pub fn segments(mut self, segments: u32) -> Self {
        self.set_count("segments", segments);
        self
    }

    pub fn rings(mut self, rings: u32) -> Self {
        self.set_count("ring_count", rings);
        self
    }

    pub fn major_segments(mut self, segments: u32) -> Self {
        self.set_count("major_segments", segments);
        self
    }

    pub fn minor_segments(mut self, segments: u32) -> Self {
        self.set_count("minor_segments", segments);
        self
    }

    pub fn bottom_radius(mut self, radius: f64) -> Self {
        self.set_param("radius2", radius);
        self
    }

    fn param(mut self, key: &'static str, value: f64) -> Self {
        self.set_param(key, value);
        self
    }

    fn set_param(&mut self, key: &'static str, value: f64) {
        if let Some((_, current)) = self.params.iter_mut().find(|(name, _)| *name == key) {
            *current = value;
        } else {
            self.params.push((key, value));
        }
    }

    fn set_count(&mut self, key: &'static str, value: u32) {
        if let Some((_, current)) = self.counts.iter_mut().find(|(name, _)| *name == key) {
            *current = value;
        } else {
            self.counts.push((key, value));
        }
    }

    fn validate(&self) -> Result<(usize, usize), Error> {
        if self.name.is_empty() || self.name.len() > 128 || self.name.contains('\0') {
            return Err(Error::Invalid(
                "name must contain 1..=128 UTF-8 bytes and no NUL",
            ));
        }
        for values in [self.location, self.rotation, self.scale] {
            if values.iter().any(|v| !v.is_finite() || v.abs() > 1000.0) {
                return Err(Error::Invalid(
                    "transforms must be finite and within +/-1000",
                ));
            }
        }
        if self.scale.iter().any(|v| *v == 0.0) {
            return Err(Error::Invalid("scale components must be nonzero"));
        }
        if self
            .color
            .iter()
            .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
        {
            return Err(Error::Invalid(
                "linear RGBA channels must be finite and within 0..=1",
            ));
        }
        for (key, _) in &self.params {
            let allowed = match self.kind {
                Kind::Cube | Kind::Plane => *key == "size",
                Kind::UvSphere => *key == "radius",
                Kind::Cylinder => matches!(*key, "radius" | "depth"),
                Kind::Cone => matches!(*key, "radius1" | "radius2" | "depth"),
                Kind::Torus => matches!(*key, "major_radius" | "minor_radius"),
            };
            if !allowed {
                return Err(Error::Invalid(
                    "primitive parameter does not apply to this shape",
                ));
            }
        }
        for (key, _) in &self.counts {
            let allowed = match self.kind {
                Kind::Cube | Kind::Plane => false,
                Kind::UvSphere => matches!(*key, "segments" | "ring_count"),
                Kind::Cylinder | Kind::Cone => *key == "segments",
                Kind::Torus => matches!(*key, "major_segments" | "minor_segments"),
            };
            if !allowed {
                return Err(Error::Invalid(
                    "segment parameter does not apply to this shape",
                ));
            }
        }
        for (key, value) in &self.params {
            let valid = match *key {
                "radius2" => value.is_finite() && (0.0..=1000.0).contains(value),
                _ => value.is_finite() && *value > 0.0 && *value <= 1000.0,
            };
            if !valid {
                return Err(Error::Invalid("primitive dimensions must be finite and within their supported positive bounds"));
            }
        }
        let count = |key: &str, default: u32, min: u32, max: u32| -> Result<usize, Error> {
            let value = self
                .counts
                .iter()
                .find(|(name, _)| *name == key)
                .map_or(default, |(_, value)| *value);
            if (min..=max).contains(&value) {
                Ok(value as usize)
            } else {
                Err(Error::Invalid(
                    "primitive segment count is outside bridge bounds",
                ))
            }
        };
        let param = |key: &str, default: f64| {
            self.params
                .iter()
                .find(|(name, _)| *name == key)
                .map_or(default, |(_, value)| *value)
        };
        let minimum_feature = match self.kind {
            Kind::Cube => param("size", 1.0),
            Kind::Plane => param("size", 2.0),
            Kind::UvSphere => param("radius", 1.0),
            Kind::Cylinder => param("radius", 1.0).min(param("depth", 2.0)),
            Kind::Cone => param("radius1", 1.0).min(param("depth", 2.0)),
            Kind::Torus => param("minor_radius", 0.25),
        };
        let min_scale = self
            .scale
            .iter()
            .map(|value| value.abs())
            .fold(f64::INFINITY, f64::min);
        if minimum_feature * min_scale < 1.0e-4 {
            return Err(Error::Invalid(
                "effective minimum primitive feature must be at least 0.0001 world units",
            ));
        }
        let primitive_radius = match self.kind {
            Kind::Cube => param("size", 1.0) * 3.0_f64.sqrt() / 2.0,
            Kind::Plane => param("size", 2.0) / 2.0_f64.sqrt(),
            Kind::UvSphere => param("radius", 1.0),
            Kind::Cylinder => param("radius", 1.0).hypot(param("depth", 2.0) / 2.0),
            Kind::Cone => param("radius1", 1.0)
                .max(param("radius2", 0.0))
                .hypot(param("depth", 2.0) / 2.0),
            Kind::Torus => param("major_radius", 1.0) + param("minor_radius", 0.25),
        };
        let max_scale = self
            .scale
            .iter()
            .map(|value| value.abs())
            .fold(0.0, f64::max);
        let max_location = self
            .location
            .iter()
            .map(|value| value.abs())
            .fold(0.0, f64::max);
        // A bounding sphere contains the rotated, component-scaled primitive; this protocol has
        // no shear or parenting transforms, so the same conservative preflight fits the host.
        if max_location + primitive_radius * max_scale > 1_000_000.0 {
            return Err(Error::Invalid(
                "transformed primitive bound exceeds +/-1000000 world units",
            ));
        }
        let geometry = match self.kind {
            Kind::Cube => {
                let _ = param("size", 1.0);
                (8, 12)
            }
            Kind::Plane => {
                let _ = param("size", 2.0);
                (4, 2)
            }
            Kind::UvSphere => {
                let s = count("segments", 16, 3, 64)?;
                let r = count("ring_count", 8, 3, 64)?;
                let _ = param("radius", 1.0);
                // Match the bridge's conservative bound across Blender's ring conventions.
                (s * r + 2, 2 * s * r)
            }
            Kind::Cylinder => {
                let s = count("segments", 32, 3, 128)?;
                let _ = (param("radius", 1.0), param("depth", 2.0));
                (2 * s, 4 * s - 4)
            }
            Kind::Cone => {
                let s = count("segments", 32, 3, 128)?;
                let _ = (param("radius1", 1.0), param("depth", 2.0));
                if param("radius2", 0.0) == 0.0 {
                    (s + 1, 2 * s - 2)
                } else {
                    (2 * s, 4 * s - 4)
                }
            }
            Kind::Torus => {
                let major = count("major_segments", 32, 3, 64)?;
                let minor = count("minor_segments", 12, 3, 64)?;
                if param("minor_radius", 0.25) >= param("major_radius", 1.0) {
                    return Err(Error::Invalid(
                        "torus minor_radius must be less than major_radius",
                    ));
                }
                (major * minor, 2 * major * minor)
            }
        };
        Ok(geometry)
    }

    fn write_json(&self, out: &mut String) -> fmt::Result {
        write!(out, "{{\"kind\":\"{}\",\"name\":", self.kind.as_str())?;
        write_string(out, &self.name)?;
        write!(out, ",\"location\":")?;
        write_array(out, &self.location)?;
        write!(out, ",\"rotation\":")?;
        write_array(out, &self.rotation)?;
        write!(out, ",\"scale\":")?;
        write_array(out, &self.scale)?;
        write!(out, ",\"color\":")?;
        write_array(out, &self.color)?;
        for (key, value) in &self.params {
            write!(out, ",\"{key}\":{value}")?;
        }
        for (key, value) in &self.counts {
            write!(out, ",\"{key}\":{value}")?;
        }
        out.push('}');
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Invalid(&'static str),
    Encoding,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(reason) => f.write_str(reason),
            Self::Encoding => f.write_str("scene command exceeds the 256 KiB output limit"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Default)]
pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, object: Object) -> Result<&mut Self, Error> {
        if self.objects.len() == MAX_OBJECTS {
            return Err(Error::Invalid("scene exceeds 64 objects"));
        }
        object.validate()?;
        if self
            .objects
            .iter()
            .any(|existing| existing.name == object.name)
        {
            return Err(Error::Invalid("object names must be unique"));
        }
        let current = self.measure_geometry();
        let added = object.validate()?;
        if current.0 + added.0 > MAX_VERTICES || current.1 + added.1 > MAX_TRIANGLES {
            return Err(Error::Invalid("scene exceeds geometry limits"));
        }
        self.objects.push(object);
        Ok(self)
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Returns the bridge-matched `(vertices, triangles)` estimate for this scene.
    pub fn estimated_geometry(&self) -> (usize, usize) {
        self.measure_geometry()
    }

    pub fn to_json(&self) -> Result<String, Error> {
        let mut out = String::from("{\"schema\":\"blr-scene-commands-v1\",\"objects\":[");
        for (index, object) in self.objects.iter().enumerate() {
            if index != 0 {
                out.push(',');
            }
            object.write_json(&mut out).map_err(|_| Error::Encoding)?;
            if out.len() > MAX_TEXT_BYTES {
                return Err(Error::Encoding);
            }
        }
        out.push_str("]}");
        if out.len() > MAX_TEXT_BYTES {
            return Err(Error::Encoding);
        }
        Ok(out)
    }

    /// Writes one validated JSON command document to stdout for the scene bridge.
    pub fn emit(&self) -> Result<(), Error> {
        let json = self.to_json()?;
        println!("BLR_SCENE_COMMANDS {json}");
        Ok(())
    }

    fn measure_geometry(&self) -> (usize, usize) {
        self.objects
            .iter()
            .filter_map(|obj| obj.validate().ok())
            .fold((0, 0), |sum, item| (sum.0 + item.0, sum.1 + item.1))
    }
}

fn write_array<const N: usize>(out: &mut String, values: &[f64; N]) -> fmt::Result {
    out.push('[');
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            out.push(',');
        }
        write!(out, "{value}")?;
    }
    out.push(']');
    Ok(())
}

fn write_string(out: &mut String, value: &str) -> fmt::Result {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch <= '\u{1f}' => write!(out, "\\u{:04x}", ch as u32)?,
            ch => out.push(ch),
        }
    }
    out.push('"');
    Ok(())
}
