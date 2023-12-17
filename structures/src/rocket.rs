#[derive(Debug, Clone)]
pub struct Rocket {
    weight: f32,
    fuel_capacity: f32,
    position: (u32, u32),
    velocity: f32
}

impl Rocket
{
    fn get_weight(&mut self ) -> &mut f32
    {
       return &mut self.weight;
    }

    fn set_weight( &self ) -> f32
    {
        return self.weight;
    }

    fn get_fuel_capacity(&mut self ) -> &mut f32
    {
       return &mut self.fuel_capacity;
    }

    fn set_fuel_capacity( &self ) -> f32
    {
        return self.fuel_capacity;
    }

    fn get_position(&mut self ) -> &mut (u32, u32)
    {
       return &mut self.position;
    }

    fn set_position( &self ) -> (u32, u32)
    {
        return self.position;
    }

    fn get_velocity(&mut self ) -> &mut f32
    {
       return &mut self.velocity;
    }

    fn set_velocity( &self ) -> f32
    {
        return self.velocity;
    }
}
