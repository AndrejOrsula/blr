use crate::{
    bpy,
    enums::Alignment,
    result::Result,
    types::{Mesh, Object},
};
use pyo3::Python;

impl Object {
    pub fn new_mesh_primitive_circle(
        py: Python,
        vertices: u32,
        radius: f32,
        fill_type: &str,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::mesh::primitive_circle_add(
            py,
            vertices,
            radius,
            fill_type,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?;
        Self::from_active(py)
    }

    pub fn new_mesh_primitive_cone(
        py: Python,
        vertices: u32,
        radius1: f32,
        radius2: f32,
        depth: f32,
        end_fill_type: &str,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::mesh::primitive_cone_add(
            py,
            vertices,
            radius1,
            radius2,
            depth,
            end_fill_type,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?;
        Self::from_active(py)
    }

    pub fn new_mesh_primitive_cube(
        py: Python,
        size: f32,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::mesh::primitive_cube_add(
            py,
            size,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?;
        Self::from_active(py)
    }

    pub fn new_mesh_primitive_cylinder(
        py: Python,
        vertices: u32,
        radius: f32,
        depth: f32,
        end_fill_type: &str,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::mesh::primitive_cylinder_add(
            py,
            vertices,
            radius,
            depth,
            end_fill_type,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?;
        Self::from_active(py)
    }

    pub fn new_mesh_primitive_grid(
        py: Python,
        x_subdivisions: u32,
        y_subdivisions: u32,
        size: f32,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::mesh::primitive_grid_add(
            py,
            x_subdivisions,
            y_subdivisions,
            size,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?;
        Self::from_active(py)
    }

    pub fn new_mesh_primitive_ico_sphere(
        py: Python,
        subdivisions: u32,
        radius: f32,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::mesh::primitive_ico_sphere_add(
            py,
            subdivisions,
            radius,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?;
        Self::from_active(py)
    }

    pub fn new_mesh_primitive_monkey(
        py: Python,
        size: f32,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::mesh::primitive_monkey_add(
            py,
            size,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?;
        Self::from_active(py)
    }

    pub fn new_mesh_primitive_plane(
        py: Python,
        size: f32,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::mesh::primitive_plane_add(
            py,
            size,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?;
        Self::from_active(py)
    }

    pub fn new_mesh_primitive_torus(
        py: Python,
        location: [f32; 3],
        rotation: [f32; 3],
        major_segments: u32,
        minor_segments: u32,
        mode: &str,
        major_radius: f32,
        minor_radius: f32,
        abso_major_rad: f32,
        abso_minor_rad: f32,
        generate_uvs: bool,
    ) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::mesh::primitive_torus_add(
            py,
            Alignment::World,
            location,
            rotation,
            major_segments,
            minor_segments,
            mode,
            major_radius,
            minor_radius,
            abso_major_rad,
            abso_minor_rad,
            generate_uvs,
        )?;
        Self::from_active(py)
    }

    pub fn new_mesh_primitive_uv_sphere(
        py: Python,
        segments: u32,

        ring_count: u32,
        radius: f32,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::mesh::primitive_uv_sphere_add(
            py,
            segments,
            ring_count,
            radius,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?;
        Self::from_active(py)
    }
}

impl Mesh {
    pub fn add_primitive_circle(
        &self,
        py: Python,
        vertices: u32,
        radius: f32,
        fill_type: &str,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<()> {
        self.set_edit_mode(py)?;
        Ok(bpy::ops::mesh::primitive_circle_add(
            py,
            vertices,
            radius,
            fill_type,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?)
    }

    pub fn add_primitive_cone(
        &self,
        py: Python,
        vertices: u32,
        radius1: f32,
        radius2: f32,
        depth: f32,
        end_fill_type: &str,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<()> {
        self.set_edit_mode(py)?;
        Ok(bpy::ops::mesh::primitive_cone_add(
            py,
            vertices,
            radius1,
            radius2,
            depth,
            end_fill_type,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?)
    }

    pub fn add_primitive_cube(
        &self,
        py: Python,
        size: f32,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<()> {
        self.set_edit_mode(py)?;
        Ok(bpy::ops::mesh::primitive_cube_add(
            py,
            size,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?)
    }

    pub fn add_primitive_cylinder(
        &self,
        py: Python,
        vertices: u32,
        radius: f32,
        depth: f32,
        end_fill_type: &str,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<()> {
        self.set_edit_mode(py)?;
        Ok(bpy::ops::mesh::primitive_cylinder_add(
            py,
            vertices,
            radius,
            depth,
            end_fill_type,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?)
    }

    pub fn add_primitive_grid(
        &self,
        py: Python,
        x_subdivisions: u32,
        y_subdivisions: u32,
        size: f32,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<()> {
        self.set_edit_mode(py)?;
        Ok(bpy::ops::mesh::primitive_grid_add(
            py,
            x_subdivisions,
            y_subdivisions,
            size,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?)
    }

    pub fn add_primitive_ico_sphere(
        &self,
        py: Python,
        subdivisions: u32,
        radius: f32,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<()> {
        self.set_edit_mode(py)?;
        Ok(bpy::ops::mesh::primitive_ico_sphere_add(
            py,
            subdivisions,
            radius,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?)
    }

    pub fn add_primitive_monkey(
        &self,
        py: Python,
        size: f32,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<()> {
        self.set_edit_mode(py)?;
        Ok(bpy::ops::mesh::primitive_monkey_add(
            py,
            size,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?)
    }

    pub fn add_primitive_plane(
        &self,
        py: Python,
        size: f32,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<()> {
        self.set_edit_mode(py)?;
        Ok(bpy::ops::mesh::primitive_plane_add(
            py,
            size,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?)
    }

    pub fn add_primitive_torus(
        &self,
        py: Python,
        location: [f32; 3],
        rotation: [f32; 3],
        major_segments: u32,
        minor_segments: u32,
        mode: &str,
        major_radius: f32,
        minor_radius: f32,
        abso_major_rad: f32,
        abso_minor_rad: f32,
        generate_uvs: bool,
    ) -> Result<()> {
        self.set_edit_mode(py)?;
        Ok(bpy::ops::mesh::primitive_torus_add(
            py,
            Alignment::World,
            location,
            rotation,
            major_segments,
            minor_segments,
            mode,
            major_radius,
            minor_radius,
            abso_major_rad,
            abso_minor_rad,
            generate_uvs,
        )?)
    }

    pub fn add_primitive_uv_sphere(
        &self,
        py: Python,
        segments: u32,
        ring_count: u32,
        radius: f32,
        calc_uvs: bool,
        location: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    ) -> Result<()> {
        self.set_edit_mode(py)?;
        Ok(bpy::ops::mesh::primitive_uv_sphere_add(
            py,
            segments,
            ring_count,
            radius,
            calc_uvs,
            false,
            Alignment::World,
            location,
            rotation,
            scale,
        )?)
    }
}
