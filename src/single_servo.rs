use embedded_hal::PwmPin;

trait Servo: embedded_hal::PwmPin {
    type SetpointType;
    fn set_position(&mut self, position: Self::SetpointType);
    fn position(&self) -> Self::Duty;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
