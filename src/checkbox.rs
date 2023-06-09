use crate::Rect;
use allegro::{Color, Core, Event};
use allegro_font::{Font, FontAlign, FontDrawing};
use allegro_primitives::PrimitivesAddon;

const BORDER_WIDTH: f32 = 3.0;
const MARGIN: f32 = 10.0;

#[derive(Clone)]
pub struct CheckBox<'a> {
  rect: Rect,
  text: &'a str,
  curr_state: bool,
  text_pos_x: f32,
  is_hovering: bool,
}

impl<'a> CheckBox<'a> {
  pub fn new(x: f32, y: f32, size: f32, text: &'a str) -> CheckBox<'a> {
    let mut checkbox = CheckBox {
      rect: Rect::new(x, y, size, size),
      text,
      curr_state: false,
      text_pos_x: 0.0,
      is_hovering: false,
    };
    checkbox.text_pos_x = checkbox.rect.x + checkbox.rect.width + BORDER_WIDTH + 10.0;
    checkbox
  }

  pub fn draw(&self, core: &Core, primitives: &PrimitivesAddon, font: &Font) {
    let text_color = Color::from_rgb(130, 130, 143);
    primitives.draw_rounded_rectangle(
      self.rect.x - BORDER_WIDTH,
      self.rect.y - BORDER_WIDTH,
      self.rect.x + self.rect.width + BORDER_WIDTH,
      self.rect.height + self.rect.y + BORDER_WIDTH,
      3.0,
      3.0,
      if self.is_hovering {
        Color::from_rgb(177, 177, 185)
      } else {
        text_color
      },
      BORDER_WIDTH,
    );
    primitives.draw_filled_rounded_rectangle(
      self.rect.x,
      self.rect.y,
      self.rect.x + self.rect.width,
      self.rect.y + self.rect.height,
      3.0,
      3.0,
      Color::from_rgb(43, 42, 51),
    );
    core.draw_text(
      font,
      text_color,
      self.text_pos_x,
      self.rect.y,
      FontAlign::Left,
      self.text,
    );
    if self.curr_state {
      primitives.draw_filled_rounded_rectangle(
        self.rect.x + BORDER_WIDTH,
        self.rect.y + BORDER_WIDTH,
        self.rect.width - BORDER_WIDTH + self.rect.x,
        self.rect.height - BORDER_WIDTH + self.rect.y,
        3.0,
        3.0,
        Color::from_rgb(67, 111, 157),
      );
    }
  }

  pub fn event_listener(&mut self, event: &Event) -> bool {
    match event {
      Event::MouseAxes { x, y, .. } => {
        self.is_hovering = self.rect.contains_point(*x as f32, *y as f32);
        return self.is_hovering;
      }
      Event::MouseButtonDown { .. } => {
        if self.is_hovering {
          self.curr_state = !self.curr_state;
        }
        return self.is_hovering;
      }
      _ => {}
    }
    false
  }

  pub fn is_checked(&self) -> bool {
    self.curr_state
  }
}

pub struct CheckBoxGroup<'a> {
  components: Vec<CheckBox<'a>>,
  label: &'a str,
  label_pos: (f32, f32),
}

impl<'a> CheckBoxGroup<'a> {
  pub fn new(
    label: &'a str,
    x: f32,
    y: f32,
    box_size: f32,
    texts: Vec<&'a str>,
    font: &Font,
  ) -> CheckBoxGroup<'a> {
    let th = font.get_line_height() as f32;
    let mut curr_x = x;
    let mut group = CheckBoxGroup {
      label,
      components: Vec::new(),
      label_pos: (x, y),
    };
    for text in texts.into_iter() {
      group
        .components
        .push(CheckBox::new(curr_x, y + th + MARGIN, box_size, text));
      curr_x += (font.get_text_width(text) as f32) + MARGIN + box_size + 50.0;
    }
    group
  }

  pub fn draw(&self, core: &Core, primitives: &PrimitivesAddon, font: &Font) {
    core.draw_text(
      font,
      Color::from_rgb(154, 153, 153),
      self.label_pos.0,
      self.label_pos.1,
      FontAlign::Left,
      self.label,
    );
    for component in self.components.iter() {
      component.draw(core, primitives, font);
    }
  }

  pub fn event_listener(&mut self, event: &Event) -> bool {
    for component in self.components.iter_mut() {
      if component.event_listener(event) {
        return true;
      }
    }
    false
  }

  pub fn get_next_y(&self) -> f32 {
    match self.components.last() {
      Some(last_el) => last_el.rect.height + last_el.rect.y + BORDER_WIDTH + 30.0,
      None => 0.0,
    }
  }

  pub fn get_values(&mut self) -> (bool, bool) {
    let n: Vec<bool> = self
      .components
      .clone()
      .into_iter()
      .map(|iter| -> bool { iter.is_checked() })
      .collect();
    if n.len() == 2 {
      (n[0], n[1])
    } else {
      (false, false)
    }
  }
}
