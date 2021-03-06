use core::convert::From;
use embedded_hal::PwmPin;

pub trait RgbLed: embedded_hal::PwmPin {
    fn set_color(&mut self, color: rgb::RGB<u16>);
}

pub trait SixColor: embedded_hal::PwmPin {
    fn red(&mut self);
    fn blue(&mut self);
    fn green(&mut self);
    fn yellow(&mut self);
    fn magenta(&mut self);
    fn cyan(&mut self);
}

impl<
        R: embedded_hal::PwmPin<Duty = T>,
        G: embedded_hal::PwmPin<Duty = T>,
        B: embedded_hal::PwmPin<Duty = T>,
        T,
    > RgbLed for RgbController<R, G, B>
where
    T: From<u16>,
{
    fn set_color(&mut self, color: rgb::RGB<u16>) {
        let color = (T::from(color.r), T::from(color.g), T::from(color.b));
        self.set_duty(color);
    }
}
pub struct RgbController<R: embedded_hal::PwmPin, G: embedded_hal::PwmPin, B: embedded_hal::PwmPin>(
    pub R,
    pub G,
    pub B,
);

impl<
        R: embedded_hal::PwmPin<Duty = T>,
        G: embedded_hal::PwmPin<Duty = T>,
        B: embedded_hal::PwmPin<Duty = T>,
        T,
    > SixColor for RgbController<R, G, B>
where
    T: From<u16>,
{
    fn red(&mut self) {
        let RgbController(r, _, _) = self;
        let r = r.get_max_duty();
        self.set_duty((r, T::from(0), T::from(0)));
    }

    fn blue(&mut self) {
        let RgbController(_, _, b) = self;
        let b = b.get_max_duty();
        self.set_duty((T::from(0), T::from(0), b));
    }

    fn green(&mut self) {
        let RgbController(_, g, _) = self;
        let g = g.get_max_duty();
        self.set_duty((T::from(0), g, T::from(0)));
    }

    fn yellow(&mut self) {
        let RgbController(r, g, _) = self;
        let r = r.get_max_duty();
        let g = g.get_max_duty();
        self.set_duty((r, g, T::from(0)));
    }

    fn magenta(&mut self) {
        let RgbController(r, _, b) = self;
        let r = r.get_max_duty();
        let b = b.get_max_duty();
        self.set_duty((r, T::from(0), b));
    }

    fn cyan(&mut self) {
        let RgbController(_, g, b) = self;
        let g = g.get_max_duty();
        let b = b.get_max_duty();
        self.set_duty((T::from(0), g, b));
    }
}

impl<R: PwmPin, G: PwmPin, B: PwmPin> embedded_hal::PwmPin for RgbController<R, G, B> {
    type Duty = (R::Duty, G::Duty, B::Duty);
    fn disable(&mut self) {
        let RgbController(red, green, blue) = self;
        red.disable();
        green.disable();
        blue.disable();
    }

    fn enable(&mut self) {
        let RgbController(red, green, blue) = self;
        red.enable();
        green.enable();
        blue.enable();
    }

    fn get_duty(&self) -> Self::Duty {
        let RgbController(red, green, blue) = self;
        (red.get_duty(), green.get_duty(), blue.get_duty())
    }

    fn get_max_duty(&self) -> Self::Duty {
        let RgbController(red, green, blue) = self;
        (
            red.get_max_duty(),
            green.get_max_duty(),
            blue.get_max_duty(),
        )
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        let (r, g, b) = duty;
        let RgbController(red, green, blue) = self;
        red.set_duty(r);
        green.set_duty(g);
        blue.set_duty(b);
    }
}
