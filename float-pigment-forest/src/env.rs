use float_pigment_css::num_traits::Zero;
use float_pigment_layout::ScreenQuery;

use crate::Len;
#[derive(Debug, Clone, Copy)]
pub struct Env {
    pub screen_width: Len,
    pub screen_height: Len,
}

impl ScreenQuery<Len> for Env {
    fn screen_height(&self) -> Len {
        self.screen_height
    }
    fn screen_width(&self) -> Len {
        self.screen_width
    }
}

impl Default for Env {
    fn default() -> Self {
        Self {
            screen_width: Len::zero(),
            screen_height: Len::zero(),
        }
    }
}
