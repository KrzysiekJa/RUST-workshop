#[derive(Debug, Clone)]
pub struct Rocket {
    weight: f32,
    fuel_capacity: f32,
    position: (u32, u32),
    velocity: f32
}

impl Rocket
{
    pub fn new ( 
        weight: f32,
        fuel_capacity: f32,
        position: (u32, u32),
        velocity: f32
     ) -> Rocket
    {
       return Rocket{
        weight: weight,
        fuel_capacity: fuel_capacity,
        position: position,
        velocity: velocity
       };
    }

    pub fn get_weight( &self ) -> f32 
    {
       return self.weight;
    }

    pub fn set_weight( &mut self ) -> &mut f32
    {
        return &mut self.weight;
    }

    pub fn get_fuel_capacity( &self ) -> f32
    {
       return self.fuel_capacity;
    }

    pub fn set_fuel_capacity( &mut self ) -> &mut f32
    {
        return &mut self.fuel_capacity;
    }

    pub fn get_position( &self ) -> (u32, u32)
    {
       return self.position;
    }

    pub fn set_position( &mut self ) -> &mut (u32, u32)
    {
        return &mut self.position;
    }

    pub fn get_velocity( &self ) -> f32
    {
       return self.velocity;
    }

    pub fn set_velocity( &mut self ) -> &mut f32
    {
        return &mut self.velocity;
    }
}
