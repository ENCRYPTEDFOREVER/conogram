use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{api::API, entities::misc::input_file::GetFiles, errors::ConogramError};
use std::fmt::Debug;

#[async_trait]
pub trait RequestT
where
    Self: Sized + Send + Sync,
    Self::ParamsType: Serialize + Debug + Sync,
    Self::ReturnType: DeserializeOwned + Debug,
{
    type ParamsType;
    type ReturnType;

    fn get_name() -> &'static str;
    fn get_api_ref(&self) -> &API;
    fn get_params_ref(&self) -> &Self::ParamsType;
    fn is_multipart() -> bool;

    async fn send(self) -> Result<Self::ReturnType, ConogramError> {
        self.send_ref().await
    }

    async fn send_ref(&self) -> Result<Self::ReturnType, ConogramError> {
        self.get_api_ref()
            .method_json(Self::get_name(), Some(self.get_params_ref()))
            .await
    }

    async fn send_multipart(self) -> Result<Self::ReturnType, ConogramError>
    where
        Self::ParamsType: GetFiles,
    {
        self.send_ref_multipart().await
    }

    async fn send_ref_multipart(&self) -> Result<Self::ReturnType, ConogramError>
    where
        Self::ParamsType: GetFiles,
    {
        self.get_api_ref()
            .method_multipart_form(Self::get_name(), Some(self.get_params_ref()))
            .await
    }
}
