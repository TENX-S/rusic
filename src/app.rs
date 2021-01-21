pub mod model;
pub mod canvas;


use model::Model;
use canvas::Canvas;

#[derive(Debug)]
pub struct App {
    model: Model,
    canvas: Canvas,
}
