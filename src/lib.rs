use std::{any::{Any, TypeId}, collections::HashMap, marker::PhantomData};

pub struct Context {
    pub data: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn add_data<T: Any>(&mut self, data: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(data));
    }
}

trait LensParam {
    type Item<'new>;

    fn retrieve<'r>(resources: &'r HashMap<TypeId, Box<dyn Any>>) -> Self::Item<'r>;
}

impl<'a, T: 'static> LensParam for &'a T {
    type Item<'new> = &'new T;

    fn retrieve<'r>(resources: &'r HashMap<TypeId, Box<dyn Any>>) -> Self::Item<'r> {
        resources.get(&TypeId::of::<T>()).unwrap().downcast_ref().unwrap()
    }
}

#[derive(Clone, Copy)]
pub struct ValueLens<Output> {
    o: Output,
}

impl<O: Copy> Lens for ValueLens<O> {
    type Input = ();
    type Output = O;
    fn view(&self, _input: &HashMap<TypeId, Box<dyn Any>>) -> O {
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

impl<I: LensParam, O, F> Lens for FunctionLens<I, O, F> 
where
    for<'a, 'b> &'a F:
        Fn(I) -> O +
        Fn(<I as LensParam>::Item<'b>) -> O,
    F: Copy
{
    type Input = I;
    type Output = O;
    fn view(&self, resources: &HashMap<TypeId, Box<dyn Any>>) -> O {
        fn call_inner<I,O>(
            f: impl Fn(I) -> O,
            _0: I,
        ) -> O {
            f(_0)
        }

        let _0 = I::retrieve(resources);
        call_inner(&self.f, _0)

    }
}


pub trait IntoLensT<Input, Output> {
    type Lens: Lens;

    fn into_lens(self) -> Self::Lens;
}

impl<I: LensParam, O, F> IntoLensT<I, O> for F 
where
    for<'a, 'b> &'a F:
        Fn(I) -> O +
        Fn(<I as LensParam>::Item<'b>) -> O,
    F: Copy,
{
    type Lens = FunctionLens<I, O, Self>;

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



pub trait IntoLens<L: Lens>: IntoLensT<L::Input, L::Output, Lens = L> {}

impl<L: Lens, T: IntoLensT<L::Input, L::Output, Lens = L>> IntoLens<L> for T {}


pub trait Lens: Copy {
    type Input;
    type Output;
    fn view(&self, input: &HashMap<TypeId, Box<dyn Any>>) -> Self::Output;
}