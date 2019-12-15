/// This file explores various ways to control the return type of a function
/// independently of its input types.

// If we do not use type parameterization then we must provide specific
// implementations of each function in order to convert from one type to one of
// several others.
mod explicit_overloading {
    fn convert_f64(a: i16) -> f64 {
        f64::from(a)
    }

    fn convert_f32(a: i16) -> f32 {
        f32::from(a)
    }

    #[test]
    fn test() {
        assert_eq!(convert_f64(4), 4.0f64);
        assert_eq!(convert_f32(4), 4.0f32);
    }
}

// By parameterizing a function we can allow the caller to control the return
// type.
mod generic_return_type {
    // We must constrain the relationship between the return value T and our
    // known input type i16 by stating that T must implement From<i16>.
    fn convert<T: From<i16>>(a: i16) -> T {
        T::from(a)
    }

    #[test]
    fn test() {
        // Despite providing an explicit type to compare to Rust cannot infer
        // the type of T, so we need to add type-annotations to the call.
        assert_eq!(convert::<f64>(4), 4.0f64);
        assert_eq!(convert::<f32>(4), 4.0f32);

        // Let's try an assignment instead. In both cases we need to tell Rust
        // the type of T, either by annotating the variable or the function
        // call.
        let _a: f32 = convert(3);
        let _a = convert::<f64>(3);
    }
}

// This example shows how an instance can control the return type.
// We create a Converter struct that is associated with the return type we want
// our convert function to return. Then anything we pass to convert using a
// a converter of type Converter<U> will be converted to type U, as long as
// the conversion is proven possible at compile time.
mod instance_controls_return_type {
    use std::marker::PhantomData;

    struct Converter<DestType> {
        // We need to use a phantom to convince the compiler we are going to
        // use our parameter type, otherwise it complains that DestType is not
        // used
        _phantom: PhantomData<DestType>,
    }

    impl<U> Converter<U> {
        fn new() -> Self {
            Self { _phantom: PhantomData }
        }

        // Will convert any T into any U as long as T implements Into<U>
        fn convert<T: Into<U>>(&self, a: T) -> U {
            a.into()
        }
    }

    #[test]
    fn test() {
        // We can delay specifying the target type
        let c0 = Converter::new();
        // ... but we will have to specify the target type eventually
        let out: f64 = c0.convert(4);
        // ... otherwise it will not be know in time for this
        assert_eq!(out, 4.0f64);

        // We can specify the target type when we create the converter
        let c1 = Converter::<f64>::new();
        // ... and use it with many different input types
        assert_eq!(c1.convert(4i32), 4.0f64);
        assert_eq!(c1.convert(4i16), 4.0f64);
        assert_eq!(c1.convert(4f32), 4.0f64);

        // Different converters target different return types.
        let c2 = Converter::<f32>::new();
        assert_eq!(c2.convert(4i16), 4.0f32);
    }
}

// Here is an example of how to overload return types using traits. This lets
// you control the mapping between input and output types when you implement
// the trait.
mod trait_implementer_controls_return_type {
    // Here we have a trait that lets implementers choose the return type
    trait Convertable {
        // This "associated type" must be defined when the trait is implemented
        type Output;

        fn convert(&self) -> Self::Output;
    }

    // This is the type we will implement our trait for. The type of T will be
    // deferred to the implementations of the Convertable trait.
    struct Value<T> {
        a: T
    }

    // Here is a specific parameterization of Value when it contains an i16.
    // In this case we have chosen at implementation-time that Value<i16> will
    // only ever be convertable to f32.
    impl Convertable for Value<i16> {
        type Output = f32;

        fn convert(&self) -> Self::Output {
            Self::Output::from(self.a)
        }
    }

    // Here is a specific parameterization of Value when it contains an i32.
    // In this case we have chosen at implementation-time that Value<i32> will
    // only ever be convertable to f64.
    impl Convertable for Value<i32> {
        type Output = f64;

        fn convert(&self) -> Self::Output {
            Self::Output::from(self.a)
        }
    }

    #[test]
    fn test() {
        // We create a Value<i16> so the relevant implementation is selected
        // and we get an f32 returned. No need to specify the return type here.
        // It was decided when we implemented the trait.
        let v1 = Value { a: 4i16 };
        assert_eq!(v1.convert(), 4.0f32);

        // We create a Value<i32> so the relevant implementation is selected
        // and we get an f64 returned
        let v2 = Value { a: 4i32 };
        assert_eq!(v2.convert(), 4.0f64);

        // As expected, this code does not compile. The sole implementation of
        // Converter for Value<i32> will only convert to f64.
        // let v3 = Value { a: 4i32 };
        // let out: u64 = v3.convert();
        //                   ^^^^^^^ expected f64, found u64
        // assert_eq!(out, 4);
    }
}
