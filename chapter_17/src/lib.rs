#![allow(dead_code)]

pub mod other_post;
pub mod post;

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        // the standard way to perform side effects is with a for loop:
        // for component in components...
        //
        // this is the functional style for performing side effects
        self.components
            .iter()
            .for_each(|component| component.draw());
        // also notice the deref coercion that converts
        // &Box<dyn Draw> to &impl Draw in component.draw()
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
