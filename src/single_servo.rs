trait Servo: embedded_hal::PwmPin {
    type SetpointType;
    fn set_position(&mut self, position: Self::SetpointType);
    fn position(&self) -> Self::Duty;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
