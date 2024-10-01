use std::{any::{Any, TypeId}, collections::HashMap, marker::PhantomData};

pub struct Context {
    pub data: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn add_data<T: Any>(&mut self, data: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(data));
    }
}

pub struct ValueLens<Output> {
    o: Output,
}

impl<O: Clone> Lens for ValueLens<O> {
    type Input = ();
    type Output = O;
    fn view(&self, _input: &()) -> O {
        self.o.clone()
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


pub trait IntoLensT<Input, Output> {
    type Lens: Lens;

    fn into_lens(self) -> Self::Lens;
}

impl<O, F: Copy + Fn() -> O> IntoLensT<(), O> for F {
    type Lens = FunctionLens<(), O, Self>;

    fn into_lens(self) -> Self::Lens {
        FunctionLens {
            f: self,
            i: Default::default(),
            o: Default::default(),
        }
    }
}

impl<I: 'static, O, F: Copy + Fn(&I) -> O> IntoLensT<(I,), O> for F {
    type Lens = FunctionLens<(I,), O, Self>;

    fn into_lens(self) -> Self::Lens {
        FunctionLens {
            f: self,
            i: Default::default(),
            o: Default::default(),
        }
    }
}

impl IntoLensT<(), f32> for f32 {
    type Lens = ValueLens<f32>;

    fn into_lens(self) -> Self::Lens {
        ValueLens {
            o: self,
        }
    }
}

pub trait IntoLens<L: Lens>: IntoLensT<(L::Input, ), L::Output, Lens = L> {}

impl<L: Lens, T: IntoLensT<(L::Input, ), L::Output, Lens = L>> IntoLens<L> for T {}


pub trait Lens {
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