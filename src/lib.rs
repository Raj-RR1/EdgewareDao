
#![no_std]
use gstd::{async_main, msg , prelude::*,ActorId};
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));


// 1. Create the main state as a static variable.
static mut EDGEWAREDAO:Option<EdgewareDao> = None;

// 1.1 Create the init state.
static mut INIT:Option<InitStruct> = None;



// 2. Create the mutability function for your state.
fn state_mut() -> &'static mut CustomStruct {

    let state = unsafe {  STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }

}

fn init_state_mut() -> &'static mut InitStruct {

    let initstruct = unsafe { INIT.as_mut()};

    unsafe { initstruct.unwrap_unchecked() }

}

// Create a public State
#[derive(Default, Encode, Decode, TypeInfo)]
struct EdgewareDao {
    approved_token_program_id: ActorId,
    period_duration: u64,
    voting_period_length: u64,
    grace_period_length: u64,
    total_shares: u128,
    members: HashMap<ActorId, Member>,
    proposal_id: u128,
    locked_funds: u128,
    proposals: HashMap<u128, Proposal>,
}

//static mut EDGEWAREDAO:Option<EdgewareDao> = None;

// Create a implementation on State
impl CustomStruct {
    #[allow(dead_code)]
    async fn firstmethod(&mut self) {}
    #[allow(dead_code)]
    async fn secondmethod(&mut self) { }
    #[allow(dead_code)]
    async fn thirdmethod(&mut self) {}
}


// 3. Create the init() function of your contract.
#[no_mangle]
extern "C" fn init () {

    let config: InitEdgewareDao = msg::load().expect("Unable to decode InitEdgewareDAO");

    // if config.ft_program_id.is_zero() {
    //     panic!("InitStruct program address can't be 0");
    // }

    let edgewaredao = EdgewareDao {
        approved_token_program_id: config.approved_token_program_id,
        voting_period_length: config.voting_period_length,
        period_duration: config.period_duration,
        ..EdgewareDao::default()

    };

    

    unsafe {
        EDGEWAREDAO = Some(edgewaredao);
    }



    let state = CustomStruct {
        ..Default::default()
    };

    unsafe { STATE = Some(state) };


}


// 4.Create the main() function of your contract.
#[async_main]
async fn main(){

        // We load the input message
        let action: Action = msg::load().expect("Could not load Action");

        // We receive an action from the user and update the state. Example:
        match &action {
            Action::FirstAction => {

                // Create a variable with mutable state.
                let currentstate = state_mut();

                // Update your state.
                currentstate.firstfield = "Update".to_string();


                 // Generate your event.
                let _ = msg::reply(Event::FirstEvent,0);


            }
            Action::SecondAction => {

                let currentstate = state_mut();

                currentstate.firstfield = "Update".to_string();

                let _ = msg::reply(Event::SecondEvent,0);
               

            }
            Action::ThirdAction => {
               
                let currentstate = state_mut();

                currentstate.firstfield = "Update".to_string();

                let _ =  msg::reply(Event::ThirdEvent,0);
            }
        };
    }

        


// 5. Create the state() function of your contract.
#[no_mangle]
extern "C" fn state() {
    let state = unsafe {
       EDGEWAREDAO.take().expect("Unexpected error in taking state")
    };
    msg::reply(state.into(), 0).expect("Failed to share state");
}