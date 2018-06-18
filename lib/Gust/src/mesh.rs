use attribute;
use std::string::String;
use glm;

#[derive(Debug)]
pub enum Error {
    FailedToFindCustomAttribute {message: String},
    WrongSizeOfAttribute {message: String},
    Attribute(attribute::Error)
}

impl From<attribute::Error> for Error {
    fn from(other: attribute::Error) -> Self {
        Error::Attribute(other)
    }
}

pub struct Mesh {
    pub no_vertices: usize,
    pub indices: Option<Vec<u16>>,
    pub attributes: Vec<attribute::Attribute>
}


impl Mesh
{
    pub fn create(positions: Vec<glm::Vec3>) -> Result<Mesh, Error>
    {
        let no_vertices = positions.len();
        let mut mesh = Mesh { no_vertices, indices: None, attributes: Vec::new() };
        mesh.add_custom_vec3_attribute("position", positions)?;
        Ok(mesh)
    }

    pub fn create_indexed(indices: Vec<u32>, positions: Vec<glm::Vec3>) -> Result<Mesh, Error>
    {
        let no_vertices = positions.len();
        let mut indices_u16 = Vec::with_capacity(indices.len());
        for i in 0..indices.len() {
            indices_u16.push(indices[i] as u16);
        }

        let mut mesh = Mesh { no_vertices, indices: Some(indices_u16), attributes: Vec::new() };
        mesh.add_custom_vec3_attribute("position", positions)?;
        Ok(mesh)
    }

    pub fn positions(&self) -> Result<&attribute::Attribute, Error>
    {
        self.get("position")
    }

    pub fn get(&self, name: &str) -> Result<&attribute::Attribute, Error>
    {
        for attribute in self.attributes.iter() {
            if attribute.name() == name
            {
                return Ok(attribute)
            }
        }
        Err(Error::FailedToFindCustomAttribute{message: format!("Failed to find {} attribute", name)})
    }

    pub fn add_custom_vec2_attribute(&mut self, name: &str, data: Vec<glm::Vec2>) -> Result<(), Error>
    {
        if self.no_vertices != data.len() {
            return Err(Error::WrongSizeOfAttribute {message: format!("The data for {} does not have the correct size, it should be {}", name, self.no_vertices)})
        }
        let custom_attribute = attribute::Attribute::create_vec2_attribute(name, data)?;
        self.attributes.push(custom_attribute);
        Ok(())
    }

    pub fn add_custom_vec3_attribute(&mut self, name: &str, data: Vec<glm::Vec3>) -> Result<(), Error>
    {
        if self.no_vertices != data.len() {
            return Err(Error::WrongSizeOfAttribute {message: format!("The data for {} does not have the correct size, it should be {}", name, self.no_vertices)})
        }
        let custom_attribute = attribute::Attribute::create_vec3_attribute(name, data)?;
        self.attributes.push(custom_attribute);
        Ok(())
    }

    pub fn add_custom_int_attribute(&mut self, name: &str, data: &Vec<u32>) -> Result<(), Error>
    {
        if self.no_vertices != data.len() {
            return Err(Error::WrongSizeOfAttribute {message: format!("The data for {} does not have the correct size, it should be {}", name, self.no_vertices)})
        }
        let custom_attribute = attribute::Attribute::create_int_attribute(name, data)?;
        self.attributes.push(custom_attribute);
        Ok(())
    }
}