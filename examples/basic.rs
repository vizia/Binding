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

    const fn value(&self) -> f32 {
        8.9
    }
}

fn test<L: Lens<Output = f32>>(cx: &Context, lens: impl IntoLens<L>) {
    println!("{:?}", lens.into_lens().view(&cx.data));
    
    // if let Some(input) = cx.data.get(&TypeId::of::<I>()).and_then(|d| d.downcast_ref::<I>()) {
    //     println!("{:?}", lens.into_lens().view(&cx.data));
    // } else if let Some(input) = <dyn Any>::downcast_ref::<I>(&()) {
    //     println!("{:?}", lens.into_lens().view(&cx.data));
    // }
    
}

pub struct TestLens;

impl Lens for TestLens {
    type Input = Model;
    type Output = f32;

    fn view(&self, resources: &HashMap<TypeId, Box<dyn Any>>) -> Self::Output {
        let i: &Model = resources.get(&TypeId::of::<Self::Input>()).unwrap().downcast_ref().unwrap();
        i.stuff
    }
}

impl IntoLensT<Model, f32> for TestLens {
    type Lens = Self;

    fn into_lens(self) -> Self::Lens {
        self
    }
}

const test_lens: TestLens = TestLens;

const VALUE: f32 = 3.89;


fn main() {


    let cx = &mut Context {
        data: HashMap::new(),
    };

    cx.add_data(Model {
        stuff: 3.14,
        other: 2.48,
    });

    test(cx, Model::stuff);
    test(cx, Model::other);
    test(cx, |model: &Model| model.other);
    
    test(cx, 6.8f32);
    test(cx, VALUE);
    test(cx, Model::value);
    test(cx, test_lens);
    
    
    
    
}

