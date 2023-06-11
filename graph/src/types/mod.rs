pub use auth_token::*;
pub use content::*;
pub use credential::*;
pub use federated_identity::*;
pub use story::*;
pub use tag::*;
pub use tag_state::*;
pub use user::*;
pub use user_consent::*;

mod content;
mod story;
mod tag;
mod user;
mod user_consent;
mod credential;
mod federated_identity;
mod auth_token;
mod tag_state;

pub trait ConvertMut {
    type Mut;
    fn into_mut(self) -> Self::Mut;
}

pub trait ConvertUnmut {
    type Unmut;
    fn into_mut(self) -> Self::Unmut;
}
