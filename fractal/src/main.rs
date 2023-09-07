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
    // Activity
    //
    // let options = eframe::NativeOptions::default();
    // eframe::run_simple_native("ProgSoc 2023 Rust", options, move |ctx, _frame| {
    //     egui::CentralPanel::default().show(ctx, |ui| {
    //         ui.centered_and_justified(|ui| {
    //             ui.heading(
    //                 egui::RichText::new("Hello, Desktop!")
    //                     .size(64.0)
    //                     .color(egui::Color32::BLACK),
    //             );
    //         });
    //     });
    // });

    // Activity: Ownership Troubles
    //
    // let mut image = egui::ColorImage::new([256, 256], egui::Color32::BLACK);
    // for (y, row) in image.pixels.chunks_mut(image.width()).enumerate() {
    //     for (x, pixel) in row.iter_mut().enumerate() { }
    // }

    // Activity: Simple Canvas
    // let mut texture: Option<egui::TextureHandle> = None;
    //
    // let options = eframe::NativeOptions::default();
    // eframe::run_simple_native("ProgSoc 2023 Rust", options, move |ctx, _frame| {
    //     egui::CentralPanel::default()
    //         .frame(egui::Frame::none().inner_margin(0.0))
    //         .show(ctx, |ui| {
    //             let texture = texture.get_or_insert_with(|| {
    //                 let mut image = egui::ColorImage::new([256, 256], egui::Color32::BLACK);
    //                 let width = image.width();
    //                 for (y, row) in image.pixels.chunks_mut(width).enumerate() {
    //                     for (x, pixel) in row.iter_mut().enumerate() {
    //                         let r = (255 - (y as i32 - x as i32).abs()) as _;
    //                         let g = x as _;
    //                         let b = y as _;
    //                         *pixel = egui::Color32::from_rgb(r, g, b);
    //                     }
    //                 }
    //                 ctx.load_texture("colour-square", image, Default::default())
    //             });
    //             ctx.layer_painter(ui.layer_id()).image(
    //                 texture.into(),
    //                 ui.available_rect_before_wrap(),
    //                 egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
    //                 egui::Color32::WHITE,
    //             );
    //             ui.centered_and_justified(|ui| {
    //                 ui.heading(
    //                     egui::RichText::new("Hello, Desktop!")
    //                         .size(64.0)
    //                         .color(egui::Color32::BLACK),
    //                 );
    //             });
    //         });
    // })

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
