#![deny(clippy::all)]
use napi::bindgen_prelude::Uint16Array;
use napi_derive::napi;

use enigo::{
  Axis, Button, Coordinate,
  Direction::{Click, Press, Release},
  Enigo, Key, Keyboard, Mouse, Settings,
};

impl Default for EnigoJs {
  fn default() -> Self {
    Self::new()
  }
}

#[napi(js_name = "Enigo")]
pub struct EnigoJs {
  enigo: Enigo,
}

#[napi]
impl EnigoJs {
  #[napi(constructor)]
  pub fn new() -> Self {
    EnigoJs {
      enigo: Enigo::new(&Settings::default()).unwrap(),
    }
  }

  #[napi]
  /// Returns a list of all currently pressed keys.
  ///
  /// ### Example
  /// ```js
  /// import { Enigo } from '@enigo-js/core'
  ///
  /// const enigo = new Enigo();
  /// const keys = enigo.held();
  /// console.log(keys);
  /// ```
  ///
  /// @returns {Array<Key>} keys - The list of currently pressed keys.
  pub fn held(&mut self) -> Uint16Array {
    let (_keys, platform_keys) = self.enigo.held();
    Uint16Array::new(platform_keys)
  }

  #[napi]
  pub fn get_mouse_position(&mut self) -> Result<Point, napi::Error> {
    let (x, y) = self
      .enigo
      .location()
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Point::new(x, y))
  }

  #[napi]
  pub fn set_mouse_position(&mut self, x: i32, y: i32) -> Result<(), napi::Error> {
    self
      .enigo
      .move_mouse(x, y, Coordinate::Abs)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn set_button_click(&mut self, button: MouseButton) -> Result<(), napi::Error> {
    let button = match button {
      MouseButton::Left => Button::Left,
      MouseButton::Middle => Button::Middle,
      MouseButton::Right => Button::Right,
    };
    self
      .enigo
      .button(button, Click)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn set_button_toggle(&mut self, button: MouseButton, down: bool) -> Result<(), napi::Error> {
    let button = match button {
      MouseButton::Left => Button::Left,
      MouseButton::Middle => Button::Middle,
      MouseButton::Right => Button::Right,
    };
    let direction = if down { Press } else { Release };
    self
      .enigo
      .button(button, direction)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn set_mouse_scroll(
    &mut self,
    direction: ScrollDirection,
    clicks: i32,
  ) -> Result<(), napi::Error> {
    let length = match direction {
      ScrollDirection::Down => clicks,
      ScrollDirection::Up => -clicks,
    };
    self
      .enigo
      .scroll(length, Axis::Vertical) // Fix: Use Axis enum
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  // Fix: Remove GetColorFromPosition and GetCurrentPositionColor as they are unrelated to Enigo
  // ...

  #[napi]
  pub fn type_text(&mut self, content: String) -> Result<(), napi::Error> {
    self
      .enigo
      .text(content.as_str())
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn press_key(&mut self, keys: Vec<KeyboardKey>) -> Result<(), napi::Error> {
    for key in keys {
      let key = transform_key(key); // Fix: use snake_case for function name
      self
        .enigo
        .key(key, Press)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    }
    Ok(())
  }

  // Release Key
  #[napi]
  pub fn release_key(&mut self, keys: Vec<KeyboardKey>) -> Result<(), napi::Error> {
    for key in keys {
      let key = transform_key(key); // Fix: use snake_case for function name
      self
        .enigo
        .key(key, Release)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    }
    Ok(())
  }

  // Press then Release Key
  #[napi]
  pub fn press_then_release_key(&mut self, keys: Vec<ToggleKey>) -> Result<(), napi::Error> {
    // Press
    for key in keys {
      let key = transform_key(key.value);
      self
        .enigo
        .key(key, Press)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    }

    // Then Release
    for key in keys {
      let key = transform_key(key.value);
      self
        .enigo
        .key(key, Release)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    }
    Ok(())
  }
}

// Get Active Window
// #[napi]
// pub fn get_active_window() -> Result<String, napi::Error> {
//     let enigo = Enigo::new(&Settings::default()).map_err(|e| napi::Error::from_reason(e.to_string()))?;
//     let window = enigo.().map_err(|e| napi::Error::from_reason(e.to_string()))?;
//     Ok(window)
// }

#[napi]
pub enum ScrollDirection {
  Down = 0,
  Up = 1,
}

#[napi]
pub enum MouseButton {
  Left = 0,
  Middle = 1,
  Right = 2,
}

#[napi]
pub enum KeyboardKey {
  Num0 = 0,
  Num1 = 1,
  Num2 = 2,
  Num3 = 3,
  Num4 = 4,
  Num5 = 5,
  Num6 = 6,
  Num7 = 7,
  Num8 = 8,
  Num9 = 9,
  A = 10,
  B = 11,
  C = 12,
  D = 13,
  E = 14,
  F = 15,
  G = 16,
  H = 17,
  I = 18,
  J = 19,
  K = 20,
  L = 21,
  M = 22,
  N = 23,
  O = 24,
  P = 25,
  Q = 26,
  R = 27,
  S = 28,
  T = 29,
  U = 30,
  V = 31,
  W = 32,
  X = 33,
  Y = 34,
  Z = 35,
  Add = 36,
  Subtract = 37,
  Multiply = 38,
  Divide = 39,
  OEM2 = 40,
  Tab = 41,
  CapsLock = 42,
  Shift = 43,
  Control = 44,
  Alt = 45,
  Space = 46,
  Backspace = 47,
  Return = 48,
  Escape = 49,
  UpArrow = 50,
  DownArrow = 51,
  LeftArrow = 52,
  RightArrow = 53,
  Meta = 54,
}

// Fix: Use snake_case for function name
fn transform_key(key: KeyboardKey) -> Key {
  match key {
    KeyboardKey::Num0 => Key::Unicode('0'),
    KeyboardKey::Num1 => Key::Unicode('1'),
    KeyboardKey::Num2 => Key::Unicode('2'),
    KeyboardKey::Num3 => Key::Unicode('3'),
    KeyboardKey::Num4 => Key::Unicode('4'),
    KeyboardKey::Num5 => Key::Unicode('5'),
    KeyboardKey::Num6 => Key::Unicode('6'),
    KeyboardKey::Num7 => Key::Unicode('7'),
    KeyboardKey::Num8 => Key::Unicode('8'),
    KeyboardKey::Num9 => Key::Unicode('9'),
    KeyboardKey::A => Key::Unicode('a'),
    KeyboardKey::B => Key::Unicode('b'),
    KeyboardKey::C => Key::Unicode('c'),
    KeyboardKey::D => Key::Unicode('d'),
    KeyboardKey::E => Key::Unicode('e'),
    KeyboardKey::F => Key::Unicode('f'),
    KeyboardKey::G => Key::Unicode('g'),
    KeyboardKey::H => Key::Unicode('h'),
    KeyboardKey::I => Key::Unicode('i'),
    KeyboardKey::J => Key::Unicode('j'),
    KeyboardKey::K => Key::Unicode('k'),
    KeyboardKey::L => Key::Unicode('l'),
    KeyboardKey::M => Key::Unicode('m'),
    KeyboardKey::N => Key::Unicode('n'),
    KeyboardKey::O => Key::Unicode('o'),
    KeyboardKey::P => Key::Unicode('p'),
    KeyboardKey::Q => Key::Unicode('q'),
    KeyboardKey::R => Key::Unicode('r'),
    KeyboardKey::S => Key::Unicode('s'),
    KeyboardKey::T => Key::Unicode('t'),
    KeyboardKey::U => Key::Unicode('u'),
    KeyboardKey::V => Key::Unicode('v'),
    KeyboardKey::W => Key::Unicode('w'),
    KeyboardKey::X => Key::Unicode('x'),
    KeyboardKey::Y => Key::Unicode('y'),
    KeyboardKey::Z => Key::Unicode('z'),
    KeyboardKey::Add => Key::Unicode('+'),
    KeyboardKey::Subtract => Key::Unicode('-'),
    KeyboardKey::Multiply => Key::Unicode('*'),
    KeyboardKey::Divide => Key::Unicode('/'),
    KeyboardKey::Tab => Key::Tab,
    KeyboardKey::CapsLock => Key::CapsLock,
    KeyboardKey::Meta => Key::Meta,
    KeyboardKey::Shift => Key::Shift,
    KeyboardKey::Control => Key::Control,
    KeyboardKey::Alt => Key::Alt,
    KeyboardKey::Space => Key::Space,
    KeyboardKey::Backspace => Key::Backspace,
    KeyboardKey::Return => Key::Return,
    KeyboardKey::Escape => Key::Escape,
    KeyboardKey::UpArrow => Key::UpArrow,
    KeyboardKey::DownArrow => Key::DownArrow,
    KeyboardKey::LeftArrow => Key::LeftArrow,
    KeyboardKey::RightArrow => Key::RightArrow,
    KeyboardKey::OEM2 => Key::End, // Use a placeholder or default key
  }
}

#[napi(object)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  pub fn new(x: i32, y: i32) -> Point {
    Point { x, y }
  }
}

#[napi(object)]
pub struct ToggleKey {
  pub value: KeyboardKey,
  pub down: bool,
}
