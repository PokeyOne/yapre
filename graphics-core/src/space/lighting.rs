// TODO: Most of this trait was just what CoPilot suggested, and not sure exactly
//       how lighting will be implemented.
pub trait Light {
    fn get_color(&self) -> Color;
    fn get_intensity(&self) -> f64;
    fn get_position(&self) -> Point;
    fn get_direction(&self) -> Vector; // TODO: Not sure if this is needed
    fn get_type(&self) -> LightType;
}