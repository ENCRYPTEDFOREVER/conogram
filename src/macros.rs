#[macro_export]
macro_rules! impl_into_future {
    ($struct: ty) => {
        impl<'a> std::future::IntoFuture for $struct {
            type IntoFuture = std::pin::Pin<
                Box<
                    dyn std::future::Future<
                            Output = Result<
                                <$struct as RequestT>::ReturnType,
                                $crate::errors::ConogramError,
                            >,
                        > + Send
                        + 'a,
                >,
            >;
            type Output = <Self::IntoFuture as std::future::Future>::Output;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send())
            }
        }

        impl<'a, 'b> std::future::IntoFuture for &'b $struct {
            type IntoFuture = std::pin::Pin<
                Box<
                    dyn std::future::Future<
                            Output = Result<
                                <$struct as RequestT>::ReturnType,
                                $crate::errors::ConogramError,
                            >,
                        > + Send
                        + 'b,
                >,
            >;
            type Output = <Self::IntoFuture as std::future::Future>::Output;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send_ref())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_future_multipart {
    ($struct: ty) => {
        impl<'a> std::future::IntoFuture for $struct {
            type IntoFuture = std::pin::Pin<
                Box<
                    dyn std::future::Future<
                            Output = Result<
                                <$struct as $crate::request::RequestT>::ReturnType,
                                $crate::errors::ConogramError,
                            >,
                        > + Send
                        + 'a,
                >,
            >;
            type Output = <Self::IntoFuture as std::future::Future>::Output;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send_multipart())
            }
        }

        impl<'a, 'b> std::future::IntoFuture for &'b $struct {
            type IntoFuture = std::pin::Pin<
                Box<
                    dyn std::future::Future<
                            Output = Result<
                                <$struct as $crate::request::RequestT>::ReturnType,
                                $crate::errors::ConogramError,
                            >,
                        > + Send
                        + 'b,
                >,
            >;
            type Output = <Self::IntoFuture as std::future::Future>::Output;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send_multipart_ref())
            }
        }
    };
}
