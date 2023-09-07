use std::ops::{Add, Sub, Mul};

use eframe::egui;

#[derive(Debug, Clone, Copy)]
struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    fn abs(&self) -> f32 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

fn main() -> eframe::Result<()> {
    let mut texture: Option<egui::TextureHandle> = None;

    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("ProgSoc 2023 Rust", options, move |ctx, _frame| {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().inner_margin(0.0))
            .show(ctx, |ui| {
                let texture = texture.get_or_insert_with(|| {
                    let mut image = egui::ColorImage::new([1024, 1024], egui::Color32::BLACK);
                    let width = image.width();
                    for (y, row) in image.pixels.chunks_mut(width).enumerate() {
                        for (x, pixel) in row.iter_mut().enumerate() {
                            let c = Complex { re: (x as f32 - 512.0) / 256.0 - 0.5, im: (y as f32 - 512.0) / 256.0 };
                            let mut z = c;
                            let mut i: u8 = 0;
                            while i < 255 && z.abs() < 4.0 {
                                z = z * z + c;
                                i += 1;
                            }
                            *pixel = egui::Color32::from_gray(i);
                        }
                    }
                    ctx.load_texture("colour-square", image, Default::default())
                });
                ui.image(texture, ui.available_size());
            });
    })
}
