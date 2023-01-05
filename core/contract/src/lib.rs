// Find all NEAR documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, env, near_bindgen, AccountId};
use std::collections::BTreeMap;



// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    proposal_count: u128,
    successful_proposal_count: u128,
    rejected_proposal_count: u128,
    proposal_vals: BTreeMap<u128, String>,
    proposal_owners: BTreeMap<u128, AccountId>,
    proposal_votes: BTreeMap<u128, Vec<(AccountId, bool)>>,
    proposal_fate: BTreeMap<u128, bool>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            proposal_count: 0, 
            successful_proposal_count: 0,
            rejected_proposal_count: 0,
            proposal_vals: BTreeMap::new(), 
            proposal_owners: BTreeMap::new(), 
            proposal_votes: BTreeMap::new(), 
            proposal_fate: BTreeMap::new(),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the current number of proposals
    pub fn get_proposal_count(&self) -> u128 {
        return self.proposal_count.clone();
    }

    // Public method - returns all proposals stored
    pub fn get_all_proposals(&self) -> BTreeMap<u128, String> {
        return self.proposal_vals.clone();
    }

    // Public method - get all the votes for given proposal ID
    pub fn get_all_votes(&self, proposal_id: u128) -> Vec<(AccountId, bool)> {
       if  self.proposal_votes.get(&proposal_id).is_none() {
        return Vec::new();
       }
        return self.proposal_votes.get(&proposal_id).clone().unwrap().to_vec();
    }
    
    // Public method - creates a new proposal
    pub fn create_proposal(&mut self, proposal_text: String) {
        let owner: AccountId = env::predecessor_account_id();
        
        log!("Registering New Proposal: {}", proposal_text);
        let new_prop_count: u128 = self.proposal_count.clone() + 1;
        self.proposal_count = new_prop_count;
        self.proposal_vals.insert(new_prop_count, proposal_text);
        self.proposal_owners.insert(new_prop_count, owner);
        self.proposal_votes.insert(new_prop_count, Vec::new());
    }

    // Public method - allows voting on a proposal (currently voting isn't capped to 1)
    pub fn vote_on_proposal(&mut self, proposal_id: u128, vote_choice: bool) {
        let proposal_exists = self.proposal_vals.get(&proposal_id);
        assert!(!proposal_exists.is_none(), "Proposal does not Exist");
        let proposal_status = self.proposal_fate.get(&proposal_id);
        assert!(proposal_status.is_none(), "Proposal has Already Closed!");
        let voter: AccountId = env::predecessor_account_id();
        let mut votes_vec = self.proposal_votes.get(&proposal_id).unwrap().clone();
        votes_vec.push((voter.clone(), vote_choice.clone()));
        self.proposal_votes.remove(&proposal_id);
        self.proposal_votes.insert(proposal_id, (votes_vec).clone().to_vec());

    }

    // Public method - allow the proposal creator to close the proposal
    pub fn close_proposal(&mut self, proposal_id: u128) -> bool{
        let proposal_exists = self.proposal_vals.get(&proposal_id);
        assert!(!proposal_exists.is_none(), "Proposal does not Exist");
        let proposal_status = self.proposal_fate.get(&proposal_id);
        assert!(proposal_status.is_none(), "Proposal has Already Closed!");
        let caller: AccountId = env::predecessor_account_id();
        assert_eq!(&caller, self.proposal_owners.get(&proposal_id.clone()).unwrap());
        log!("Closing Proposal: {}", proposal_id);
        let votes_vec = self.proposal_votes.get(&proposal_id).unwrap().clone();
        let mut upvotes : u128 = 0;
        for item in votes_vec.clone() {
            if item.1 {
                upvotes += 1;
            }
        }
        if 2 * upvotes >= votes_vec.clone().len().try_into().unwrap(){
            self.proposal_fate.insert(proposal_id, true);
            self.successful_proposal_count += 1;
            return true;
        }
        else {
            self.proposal_fate.insert(proposal_id, false);
            self.rejected_proposal_count += 1;
            return false;
        }
        
        
    }

    // Public method - allow the proposal creator to void the proposal if too few votes
    pub fn void_proposal(&mut self, proposal_id: u128) -> bool{
        let proposal_exists = self.proposal_vals.get(&proposal_id);
        assert!(!proposal_exists.is_none(), "Proposal does not Exist");
        let proposal_status = self.proposal_fate.get(&proposal_id);
        assert!(proposal_status.is_none(), "Proposal has Already Closed!");
        let caller: AccountId = env::predecessor_account_id();
        assert_eq!(&caller, self.proposal_owners.get(&proposal_id.clone()).unwrap());
        log!("Voiding Proposal: {}", proposal_id);
        let votes_vec = self.proposal_votes.get(&proposal_id).unwrap().clone();
        let mut upvotes : u128 = 0;
        for item in votes_vec.clone() {
            if item.1 {
                upvotes += 1;
            }
        }
        if upvotes == 0{
            self.proposal_fate.insert(proposal_id, false);
            self.rejected_proposal_count += 1;
            return true;
        }
        else {
            
            
            return false;
        }
        
        
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::Balance;

    //const BENEFICIARY: &str = "beneficiary";
    const NEAR: u128 = 1000000000000000000000000;

    #[test]
    fn test_get_default_proposals() {
        let contract = Contract::default();
        // to check if the initialization is with no proposals already stored
        assert_eq!(
            contract.get_proposal_count(),
            0
        );
    }

    #[test]
    fn test_create_new_proposal() {
        let mut contract = Contract::default();
        let acc: AccountId = "harry.near".parse().unwrap();
        set_context(acc, 10*NEAR);
        contract.create_proposal("Should bears be legal pets?".to_string());
        assert_eq!(
            contract.get_proposal_count(),
            1
        );
    }

    #[test]
    fn test_vote_on_proposal() {
        let mut contract = Contract::default();
        let acc1: AccountId = "harry.near".parse().unwrap();
        set_context(acc1, 10*NEAR);
        contract.create_proposal("Should bears be legal pets?".to_string());
        let acc2: AccountId = "mikky.near".parse().unwrap();
        set_context(acc2, 10*NEAR);
        contract.vote_on_proposal(1, true);
        assert_eq!(
            1,
            1
        );
        // Just checks if the code finishes execution 
        // and the said steps complete without panicking
        // This is taken to imply success.
    }

    #[test]
    fn test_close_proposal() {
        let mut contract = Contract::default();
        let acc1: AccountId = "harry.near".parse().unwrap();
        set_context(acc1.clone(), 10*NEAR);
        contract.create_proposal("Should bears be legal pets?".to_string());
        let acc2: AccountId = "kurt.near".parse().unwrap();
        set_context(acc2, 10*NEAR);
        contract.vote_on_proposal(1, true);
        let acc3: AccountId = "weiler.near".parse().unwrap();
        set_context(acc3, 10*NEAR);
        contract.vote_on_proposal(1, false);
        let acc4: AccountId = "brandon.near".parse().unwrap();
        set_context(acc4, 10*NEAR);
        contract.vote_on_proposal(1, true);
        let acc5: AccountId = "snow.near".parse().unwrap();
        set_context(acc5, 10*NEAR);
        contract.vote_on_proposal(1, true);
        set_context(acc1.clone(), 10*NEAR);
        let result = contract.close_proposal(1);
        assert_eq!(
            result,
            true
        );
       
    }

    #[test]
    fn test_void_proposal() {
        let mut contract = Contract::default();
        let acc1: AccountId = "harry.near".parse().unwrap();
        set_context(acc1.clone(), 10*NEAR);
        contract.create_proposal("Should bears be legal pets?".to_string());
        
        set_context(acc1.clone(), 10*NEAR);
        let result = contract.void_proposal(1);
        assert_eq!(
            result,
            true
        );
       
    }

    

    fn set_context(predecessor: AccountId, amount: Balance) {
        let mut builder = VMContextBuilder::new();
        
        builder.predecessor_account_id(predecessor);
        builder.attached_deposit(amount);
    
        testing_env!(builder.build());
      }
}
