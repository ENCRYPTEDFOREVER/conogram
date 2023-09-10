#[macro_export]
macro_rules! impl_into_future {
    ($struct: ty) => {
        impl<'a> IntoFuture for $struct {
            type IntoFuture = Pin<
                Box<
                    dyn Future<Output = Result<<$struct as RequestT>::ReturnType, ConogramError>>
                        + Send
                        + 'a,
                >,
            >;
            type Output = <Pin<
                Box<
                    dyn Future<Output = Result<<$struct as RequestT>::ReturnType, ConogramError>>
                        + Send
                        + 'a,
                >,
            > as Future>::Output;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send())
            }
        }

        impl<'a, 'b> IntoFuture for &'b $struct {
            type IntoFuture = Pin<
                Box<
                    dyn Future<Output = Result<<$struct as RequestT>::ReturnType, ConogramError>>
                        + Send
                        + 'b,
                >,
            >;
            type Output = <Pin<
                Box<
                    dyn Future<Output = Result<<$struct as RequestT>::ReturnType, ConogramError>>
                        + Send
                        + 'b,
                >,
            > as Future>::Output;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send_ref())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_future_multipart {
    ($struct: ty) => {
        impl<'a> IntoFuture for $struct {
            type IntoFuture = Pin<
                Box<
                    dyn Future<Output = Result<<$struct as RequestT>::ReturnType, ConogramError>>
                        + Send
                        + 'a,
                >,
            >;
            type Output = <Pin<
                Box<
                    dyn Future<Output = Result<<$struct as RequestT>::ReturnType, ConogramError>>
                        + Send
                        + 'a,
                >,
            > as Future>::Output;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send_multipart())
            }
        }

        impl<'a, 'b> IntoFuture for &'b $struct {
            type IntoFuture = Pin<
                Box<
                    dyn Future<Output = Result<<$struct as RequestT>::ReturnType, ConogramError>>
                        + Send
                        + 'b,
                >,
            >;
            type Output = <Pin<
                Box<
                    dyn Future<Output = Result<<$struct as RequestT>::ReturnType, ConogramError>>
                        + Send
                        + 'b,
                >,
            > as Future>::Output;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send_ref_multipart())
            }
        }
    };
}
