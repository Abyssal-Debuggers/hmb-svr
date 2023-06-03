use async_graphql::{ComplexObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use entity::content;
use entity::prelude::Json;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Content {
    pub seq_no: i32,
    #[graphql(skip)]
    pub raw: Json,
}


#[ComplexObject]
impl Content {
    async fn raw(&self) -> String {
        self.raw.to_string()
    }
    async fn json(&self) -> Value {
        self.raw.clone()
    }
}
