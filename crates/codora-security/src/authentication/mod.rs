use claim::Claim;
use std::sync::{Arc, RwLock};

pub mod claim;
pub mod handler;

/**
 * In authentication
 * we have:
 * Authentication Methods  
 *      - Each methods has event i guess
 *      -
 * Authentication Scheme + Handler
 *      - Each scheme handler has what again
*/

// Create your signin
// pub struct Signin;

// impl Authentication for Signin {
//     type Methods = [EmailPassword, MFA];
//     type Events = [Pass in the event];
//     type Scheme = [Cookie, Jwt];
// }
// Because Signin implement Authentication it will get some auto trait

pub trait Authentication {}
// Authentication State
pub struct State {
    // THis will hold all the state that comes with the request
}

// Hot context created on every Request so each Request has it's own context
#[derive(Clone)]
pub struct Context<Request> {
    state: Arc<State>,
    claim: Arc<RwLock<Claim>>,
    // Mostly contain request with empty body
    request_parts: Request,
}

impl<Request> Context<Request> {
    pub fn new(request_parts: Request) -> Self {
        let state = Arc::new(State {});
        let claim = Arc::new(RwLock::new(Claim::default()));

        Context {
            state,
            claim,
            request_parts,
        }
    }
}

#[cfg(test)]
mod test {
    // Replace this with state! we want something that could store any T like extension used in http::Extension!
    /*
    All the state added in this layer are transient which means it's shared among request
    tapped into request run authenticate which populate the context claim!
    let auth = AuthenticationBuilder::new(/Some State/)
        .add_cookie()
        .add_jwt()
        .add_state(|state| state.add(String::new()))
     */
    // THis state should be shared among Context you see

    #[tokio::test]
    async fn test_context() {}
}
