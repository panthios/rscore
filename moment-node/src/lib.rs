use async_trait::async_trait;


#[async_trait]
pub trait Moment: Clone + Send + Sync {
    type Output: Moment;

    async fn resolve(&self) -> Self::Output;
}

macro_rules! literal_moments {
    ($($name:ident),*) => {
        $(
            #[async_trait]
            impl Moment for $name {
                type Output = $name;

                async fn resolve(&self) -> Self::Output {
                    *self
                }
            }
        )*
    }
}

literal_moments![
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
    bool,
    char
];

#[async_trait]
impl<T> Moment for Option<T>
where
    T: Moment,
{
    type Output = Option<T::Output>;

    async fn resolve(&self) -> Self::Output {
        if let Some(value) = self {
            Some(value.resolve().await)
        } else {
            None
        }
    }
}