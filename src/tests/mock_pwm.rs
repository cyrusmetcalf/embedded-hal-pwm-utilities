#[derive(Debug, PartialEq)]
pub struct FakePwmPin {
    enabled: bool,
    duty: u16,
}

impl FakePwmPin {
    pub const MAX_DUTY: u16 = 65535_u16;
}

impl Default for FakePwmPin {
    fn default() -> Self {
        Self {
            enabled: false,
            duty: 0,
        }
    }
}

impl embedded_hal::PwmPin for FakePwmPin {
    type Duty = u16;
    fn disable(&mut self) {
        self.enabled = false;
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn get_duty(&self) -> Self::Duty {
        self.duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        Self::MAX_DUTY
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        self.duty = duty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal::PwmPin;

    #[test]
    fn can_get_default_fake_pwm() {
        let fake_pwm = FakePwmPin::default();
        assert_eq!(
            fake_pwm,
            FakePwmPin {
                enabled: false,
                duty: 0_u16
            }
        );
    }

    #[test]
    fn fake_pwm_can_be_enabled_and_disabled() {
        let mut fake_pwm = FakePwmPin::default();
        fake_pwm.enable();

        assert_eq!(
            fake_pwm,
            FakePwmPin {
                enabled: true,
                duty: 0_u16
            }
        );
    }

    #[test]
    fn can_get_max_duty() {
        let fake_pwm = FakePwmPin::default();
        assert_eq!(fake_pwm.get_max_duty(), FakePwmPin::MAX_DUTY)
    }

    #[test]
    fn can_set_duty() {
        let duty = 1234_u16;
        let mut fake_pwm = FakePwmPin::default();
        fake_pwm.set_duty(duty);

        assert_eq!(duty, fake_pwm.get_duty());
    }
}
