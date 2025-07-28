// use crate::authentication::Authentication;

// pub struct CookieOption {}

// pub trait CookieAuthenticationExt {
//     fn add_cookie<F>(self, func: F) -> Self
//     where
//         F: Fn(&Authentication) -> CookieOption;
// }

// impl CookieAuthenticationExt for Authentication {
//     fn add_cookie<F>(self, func: F) -> Self
//     where
//         F: Fn(&Authentication) -> CookieOption,
//     {
//         let opt = func(&self);
//         todo!()
//     }
// }
use crate::{
    Context,
    claim::Claim,
    handler::{sign_in::SignInHandler, sign_out::SignOutHandler},
};

pub struct Cookie {}

impl<Request> crate::authentication::handler::Handler<Request> for Cookie
where
    Request: Sync,
{
    type Error = ();

    const NAME: &'static str = "Cookie";

    async fn authenticate(&self, contex: &Context<Request>, claim: &Claim) -> Result<(), Self::Error> {
        todo!()
    }

    async fn forbid(&self, contex: &Context<Request>, claim: &Claim) -> Result<(), Self::Error> {
        todo!()
    }

    async fn challenge(&self, contex: &Context<Request>, claim: &Claim) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<Request> SignOutHandler<Request> for Cookie
where
    Request: Sync + Send,
{
    async fn sign_out(&self, ctx: &Context<Request>, claim: &Claim) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<Request> SignInHandler<Request> for Cookie
where
    Request: Sync + Send,
{
    type Success = ();

    async fn sign_in(&self, contex: &Context<Request>, claim: &Claim) -> Result<Self::Success, Self::Error> {
        todo!()
    }
}
