use crate::chat_server::config::AppConfig;
use crate::chat_server::error::AppError;
use anyhow::Result;
use std::{ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>, // 做成Arc的间接引用, 方便多处共享
}

#[allow(unused)]
#[derive(Debug)]
pub struct AppStateInner {
    config: AppConfig,
    // pub(crate) dk: DecodingKey,
    // pub(crate) ek: EncodingKey,
    // pub(crate) pool: PgPool,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        // 为AppState实现Detef Trait, 使其自动暴露内部属性
        // state.config => state.inner.config 对Arc进行deref, 自动deref
        &self.inner
    }
}

impl AppState {
    /** 实例化 */
    pub async fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner { config }),
        }
    }

    /** 尝试实例化(考虑异常) */
    pub async fn try_new(_config: AppConfig) -> Result<Self, AppError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}