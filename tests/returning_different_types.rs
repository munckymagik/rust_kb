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

mod generic_overloading {
    fn convert<T: From<i16>>(a: i16) -> T {
        T::from(a)
    }

    #[test]
    fn test() {
        // Despite providing an explicit type to compare to Rust cannot infer the
        // type of T, so we need to add type-annotations.
        assert_eq!(convert::<f64>(4), 4.0f64);
        assert_eq!(convert::<f32>(4), 4.0f32);

        // Let's try an assignment instead. In both cases we need to tell Rust the
        // type of T.
        let _a: f32 = convert(3);
        let _a = convert::<f64>(3);
    }
}

mod struct_generic {
    struct Converter<T> {
        a: i16,
        // Needed so the struct is parameterized by T. Otherwise we get a
        // > error[E0207]: the type parameter `T` is not constrained by the impl
        // > trait, self type, or predicates
        phantom: std::marker::PhantomData<T>,
    }

    impl<T: From<i16>> Converter<T> {
        fn convert(&self) -> T {
            T::from(self.a)
        }
    }

    #[test]
    fn test() {
        let c1 = Converter::<f64> { a: 4, phantom: std::marker::PhantomData };
        let c2 = Converter::<f32> { a: 4, phantom: std::marker::PhantomData };
        assert_eq!(c1.convert(), 4.0f64);
        assert_eq!(c2.convert(), 4.0f32);
    }
}

mod specific_parameterizations_of_a_trait {
    trait Convertable {
        type Output;

        fn convert(&self) -> Self::Output;
    }

    struct Value<T> {
        a: T
    }

    // For specific parameterizations of Value
    impl Convertable for Value<i16> {
        type Output = f32;

        fn convert(&self) -> Self::Output {
            Self::Output::from(self.a)
        }
    }

    impl Convertable for Value<i32> {
        type Output = f64;

        fn convert(&self) -> Self::Output {
            Self::Output::from(self.a)
        }
    }

    #[test]
    fn test() {
        // We create a Value<i16> so the relevant implementation is selected
        // and we get an f32 returned
        let v1 = Value { a: 4i16 };
        assert_eq!(v1.convert(), 4.0f32);

        // We create a Value<i32> so the relevant implementation is selected
        // and we get an f64 returned
        let v2 = Value { a: 4 };
        assert_eq!(v2.convert(), 4.0f64);
    }
}

// Blanket specialisation using a trait bound on the conversion from T to Output
// impl<T> Convertable for Value<T>
// where
//     f32: std::convert::From<T>,
//     T: Copy,
// {
//     type Output = f32;

//     fn convert(&self) -> Self::Output {
//         Self::Output::from(self.a)
//     }
// }

// #[test]
// fn test_trait_associated_type() {
//     let v1 = Value { a: 4i16 };
//     assert_eq!(v1.convert(), 4.0f32);

//     // There is not conversion from i64 to i32 so we need a specialisation
//     // let v2 = Value { a: 4i64 };
//     // assert_eq!(v2.convert(), 4.0f64);
// }
