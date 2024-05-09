use soroban_sdk::{contractimpl, Bytes, BytesN, Env, Symbol, Address, IntoVal};

#[derive(Clone)]
struct StudyGroup {
    name: String,
    topic: String,
    creator: Address,
}

#[derive(Clone)]
struct Membership {
    group_id: Bytes,
    member: Address,
    contribution_score: i32, 
}

pub struct StudyGroupContract;

#[contractimpl]
impl StudyGroupContract {
    
    pub fn create_group(e: Env, creator: Address, name: String, topic: String) {
        let group_id = get_next_group_id(&e);
        e.data().set(group_id.clone(), StudyGroup { name, topic, creator });

        
        e.data().set(get_membership_key(group_id, creator), Membership { 
            group_id, 
            member: creator, 
            contribution_score: 0 
        });
    }

    
    pub fn join_group(e: Env, member: Address, group_id: Bytes) {
        
        e.data().set(get_membership_key(group_id, member), Membership { 
            group_id, 
            member, 
            contribution_score: 0 
        });
    }

    // Record a contribution 
    pub fn record_contribution(e: Env, member: Address, group_id: Bytes, amount: i32) {
    

        let mut membership = e.data()
            .get::<Membership>(get_membership_key(group_id, member))
            .unwrap().unwrap();
        membership.contribution_score += amount;
        e.data().set(get_membership_key(group_id, member), membership);
    }

    
    pub fn distribute_rewards(e: Env, group_id: Bytes) {
        let members = get_group_members(&e, group_id);
        let total_tokens = 100; // Example

        for member in members {
            let share = (member.contribution_score / total_contributions(&e, group_id)) * total_tokens; 

            invoke_token_contract(&e, member.member, share);
        }
    }
}


fn get_next_group_id(e: &Env) -> Bytes {
}

fn get_membership_key(group_id: Bytes, member: Address) -> Bytes {

}

fn get_group_members(e: &Env, group_id: Bytes) -> Vec<Membership> {

}

fn total_contributions(e: &Env, group_id: Bytes) -> i32 {
   
}

fn invoke_token_contract(e: &Env, to: &Address, amount: i32) {

}
