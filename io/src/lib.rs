
#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{In,Out,InOut,Metadata};



// 1. Create your own Actions
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Action {
    
    // Add Actions
    FirstAction,
    SecondAction,
    ThirdAction,
    
}


// 2. Create your own Events
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum  Event {
    
    // Add Events(Example)
    FirstEvent,
    SecondEvent,
    ThirdEvent,
}


// 3. Create your own Struct
#[derive(Default, Encode, Decode, Clone, TypeInfo)]
pub struct Proposal {
    pub proposer: ActorId,
    pub applicant: ActorId,
    pub yes_votes: u128,
    pub no_votes: u128,
    pub quorum: u128,
    pub amount: u128,
    pub processed: bool,
    pub did_pass: bool,
    pub details: String,
    pub starting_period: u64,
    pub ended_at: u64,
    pub votes_by_member: Vec<(ActorId, Vote)>,
}

pub struct Member {
    pub shares: u128,
    pub highest_index_yes_vote: Option<u128>,
}


// 4. Create your init Struct
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitEdgewareDao {
    pub approved_token_program_id: ActorId,
    pub voting_period_length: u64,
    pub period_duration: u64,
    pub grace_period_length: u64,
}


pub struct ContractMetadata;

// 5. Define the structure of actions, events and state for your metadata.
impl Metadata for ContractMetadata{
     type Init = In<InitStruct>;
     type Handle = InOut<Action,Event>;
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Out<CustomStruct>;

}