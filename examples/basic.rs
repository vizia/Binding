use std::{any::{Any, TypeId}, collections::HashMap};

use binding::*;

pub struct Model {
    stuff: f32,
    other: f32,
}

impl Model {
    fn stuffy(&self) -> f32 {
        self.stuff
    }
}

fn test<I, L: Lens<Output = f32>>(cx: &Context, lens: impl IntoLens<I, L::Output, Lens = L>) {
    
    
    if let Some(input) = cx.data.get(&TypeId::of::<L::Input>()).and_then(|d| d.downcast_ref::<L::Input>()) {
        println!("{}", lens.into_lens().view(input));
    } else if let Some(input) = <dyn Any>::downcast_ref::<L::Input>(&()) {
        println!("{}", lens.into_lens().view(input));
    }
    
}

fn test2<L: Lens<Output = f32>>(cx: &Context, lens: L) {
    if let Some(input) = <dyn Any>::downcast_ref::<L::Input>(&()) {
        println!("{}", lens.view(input));
    }
    
    if let Some(input) = cx.data.get(&TypeId::of::<L::Input>()).and_then(|d| d.downcast_ref::<L::Input>()) {
        println!("{}", lens.view(input));
    }
    
}

fn value() -> f32 {
    4.90
}

fn main() {
    let mut cx = &mut Context {
        data: HashMap::new(),
    };

    cx.add_data(Model {
        stuff: 3.14,
        other: 2.48,
    });

    test(cx, Model::stuffy);
    test(cx, |model: &Model| model.other);
    // test(cx, &6.8);
    test(cx, value);

    test2(cx, Model::stuffy.into_lens());
    test2(cx, (|model: &Model| model.other).into_lens());
    // test2(cx, &6.8);
    test2(cx, value.into_lens());
}

