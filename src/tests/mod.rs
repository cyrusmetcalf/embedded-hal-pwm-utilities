pub mod mock_pwm;

use super::*;
use embedded_hal::PwmPin;
use mock_pwm::FakePwmPin;
use rgb_controller::{RgbController, SixColor};

#[test]
fn can_make_fake_pwm() {
    let (r, g, b) = (
        FakePwmPin::default(),
        FakePwmPin::default(),
        FakePwmPin::default(),
    );
    let _light_controller = RgbController(r, g, b);
}

#[test]
fn can_enable() {
    let (r, g, b) = (
        FakePwmPin::default(),
        FakePwmPin::default(),
        FakePwmPin::default(),
    );
    let mut light_controller = RgbController(r, g, b);
    light_controller.enable();
}

#[test]
fn can_disable() {
    let (r, g, b) = (
        FakePwmPin::default(),
        FakePwmPin::default(),
        FakePwmPin::default(),
    );
    let mut light_controller = RgbController(r, g, b);
    light_controller.disable();
}

#[test]
fn can_set_red() {
    let (r, g, b) = (
        FakePwmPin::default(),
        FakePwmPin::default(),
        FakePwmPin::default(),
    );
    let mut light_controller = RgbController(r, g, b);
    light_controller.enable();
    let (red_max, _, _) = light_controller.get_max_duty();

    assert_eq!((0, 0, 0), light_controller.get_duty());
    light_controller.red();
    assert_eq!((red_max, 0, 0), light_controller.get_duty());
}

#[test]
fn can_set_yellow() {
    let (r, g, b) = (
        FakePwmPin::default(),
        FakePwmPin::default(),
        FakePwmPin::default(),
    );
    let mut light_controller = RgbController(r, g, b);
    light_controller.enable();
    let (red_max, green_max, _) = light_controller.get_max_duty();

    assert_eq!((0, 0, 0), light_controller.get_duty());
    light_controller.yellow();
    assert_eq!((red_max, green_max, 0), light_controller.get_duty());
}

#[test]
fn can_set_green() {
    let (r, g, b) = (
        FakePwmPin::default(),
        FakePwmPin::default(),
        FakePwmPin::default(),
    );
    let mut light_controller = RgbController(r, g, b);
    light_controller.enable();
    let (_, green_max, _) = light_controller.get_max_duty();

    assert_eq!((0, 0, 0), light_controller.get_duty());
    light_controller.green();
    assert_eq!((0, green_max, 0), light_controller.get_duty());
}

#[test]
fn can_set_cyan() {
    let (r, g, b) = (
        FakePwmPin::default(),
        FakePwmPin::default(),
        FakePwmPin::default(),
    );
    let mut light_controller = RgbController(r, g, b);
    light_controller.enable();
    let (_, green_max, blue_max) = light_controller.get_max_duty();

    assert_eq!((0, 0, 0), light_controller.get_duty());
    light_controller.cyan();
    assert_eq!((0, green_max, blue_max), light_controller.get_duty());
}

#[test]
fn can_set_blue() {
    let (r, g, b) = (
        FakePwmPin::default(),
        FakePwmPin::default(),
        FakePwmPin::default(),
    );
    let mut light_controller = RgbController(r, g, b);
    light_controller.enable();
    let (_, _, blue_max) = light_controller.get_max_duty();

    assert_eq!((0, 0, 0), light_controller.get_duty());
    light_controller.blue();
    assert_eq!((0, 0, blue_max), light_controller.get_duty());
}

#[test]
fn can_set_magenta() {
    let (r, g, b) = (
        FakePwmPin::default(),
        FakePwmPin::default(),
        FakePwmPin::default(),
    );
    let mut light_controller = RgbController(r, g, b);
    light_controller.enable();
    let (red_max, _, blue_max) = light_controller.get_max_duty();

    assert_eq!((0, 0, 0), light_controller.get_duty());
    light_controller.magenta();
    assert_eq!((red_max, 0, blue_max), light_controller.get_duty());
}
