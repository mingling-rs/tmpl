use mingling::macros::{buffer, import_type, r_println, renderer};

import_type!(<<<pascal_case>>> = /* external::type::Path */);

#[renderer(buffer)]
pub fn render_<<<snake_case>>>(external: <<<pascal_case>>>) {
    r_println!("");
}
