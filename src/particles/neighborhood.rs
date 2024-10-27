use super::Particle;

pub struct Neighborhood<'a>(pub Vec<Vec<&'a Option<Box<dyn Particle>>>>);

impl<'a> Neighborhood<'a> {
    pub fn up(&self) -> &Option<Box<dyn Particle>> {
        self.0[0][1]
    }

    pub fn down(&self) -> &Option<Box<dyn Particle>> {
        self.0[2][1]
    }

    pub fn left(&self) -> &Option<Box<dyn Particle>> {
        self.0[1][0]
    }

    pub fn right(&self) -> &Option<Box<dyn Particle>> {
        self.0[1][2]
    }

    pub fn up_left(&self) -> &Option<Box<dyn Particle>> {
        self.0[0][0]
    }

    pub fn up_right(&self) -> &Option<Box<dyn Particle>> {
        self.0[0][2]
    }

    pub fn down_left(&self) -> &Option<Box<dyn Particle>> {
        self.0[2][0]
    }

    pub fn down_right(&self) -> &Option<Box<dyn Particle>> {
        self.0[2][2]
    }

    pub fn iter(
        &self,
    ) -> std::iter::Flatten<std::slice::Iter<'_, Vec<&Option<Box<dyn Particle>>>>> {
        self.0.iter().flatten()
    }
}
