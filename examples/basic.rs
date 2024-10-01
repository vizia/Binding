use std::{any::{Any, TypeId}, collections::HashMap};

use binding::*;

pub struct Model {
    stuff: f32,
    other: f32,
}

impl Model {
    fn stuff(&self) -> f32 {
        self.stuff
    }

    fn other(&self) -> f32 {
        self.other
    }

    fn value(&self) -> f32 {
        8.9
    }
}

fn test<L: Lens<Output = f32>>(cx: &Context, lens: impl IntoLens<L>) {
    
    
    if let Some(input) = cx.data.get(&TypeId::of::<L::Input>()).and_then(|d| d.downcast_ref::<L::Input>()) {
        println!("{:?}", lens.into_lens().view(input));
    } else if let Some(input) = <dyn Any>::downcast_ref::<L::Input>(&()) {
        println!("{:?}", lens.into_lens().view(input));
    }
    
}

fn main() {
    let cx = &mut Context {
        data: HashMap::new(),
    };

    cx.add_data(Model {
        stuff: 3.14,
        other: 2.48,
    });

    test(cx, Model::stuff);
    test(cx, |model: &Model| model.other);
    
    // test(cx, 6.8f32);
    // test(cx, value);
    test(cx, Model::other);
    test(cx, Model::value);
    
}

