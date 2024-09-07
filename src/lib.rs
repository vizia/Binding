use std::{any::{Any, TypeId}, collections::HashMap, marker::PhantomData};

pub struct Context {
    pub data: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn add_data<T: Any>(&mut self, data: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(data));
    }
}


pub struct FunctionLens<Input, Output, F> {
    pub f: F,
    i: PhantomData<Input>,
    o: PhantomData<Output>,
}

impl<I,O,F: Clone> Clone for FunctionLens<I,O,F> {
    fn clone(&self) -> Self {
        Self { f: self.f.clone(), i: Default::default(), o: Default::default() }
    }
}

impl<I,O,F: Copy> Copy for FunctionLens<I,O,F> {
    
}


impl<O, F: Copy + Fn() -> O> Lens for FunctionLens<(), O, F> {
    type Input = ();
    type Output = O;
    fn view(&self, _input: &()) -> O {
        (self.f)()
    }
}

impl<I: 'static, O, F: Copy + Fn(&I) -> O> Lens for FunctionLens<(I,), O, F> {
    type Input = I;
    type Output = O;
    fn view(&self, input: &I) -> O {
        (self.f)(input)
    }
}


pub trait IntoLens<Input, Output> {
    type Lens: Lens;

    fn into_lens(self) -> Self::Lens;
}

// impl<O> IntoLens<(), O> for O {
//     type Lens = FunctionLens<(), O, Self>;

//     fn into_lens(self) -> Self::Lens {
//         FunctionLens {
//             f: self,
//             i: Default::default(),
//             o: Default::default(),
//         }
//     }
// }

impl<O, F: Copy + Fn() -> O> IntoLens<(), O> for F {
    type Lens = FunctionLens<(), O, Self>;

    fn into_lens(self) -> Self::Lens {
        FunctionLens {
            f: self,
            i: Default::default(),
            o: Default::default(),
        }
    }
}

impl<I: 'static, O, F: Copy + Fn(&I) -> O> IntoLens<(I,), O> for F {
    type Lens = FunctionLens<(I,), O, Self>;

    fn into_lens(self) -> Self::Lens {
        FunctionLens {
            f: self,
            i: Default::default(),
            o: Default::default(),
        }
    }
}

pub trait Lens: Copy {
    type Input: 'static;
    type Output;
    fn view(&self, input: &Self::Input) -> Self::Output;
}

// impl<O: Clone> Lens for &O {
//     type Input = ();
//     type Output = O;
//     fn view(&self, _: &Self::Input) -> Self::Output {
//         (*self).clone()
//     }
// }