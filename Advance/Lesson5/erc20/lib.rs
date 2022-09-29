#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use ink_storage::{ traits::SpreadAllocate, Mapping };
    use scale::{Encode, Decode};
    use ink_prelude::string::String;

    pub type Result<T> = core::result::Result<T, Error>;
    
    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error
    {
        InsufficientBalance,
        InsufficientAllowance,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Erc20
    {
        name: String,
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: ink_storage::Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer
    {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval
    {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    impl Erc20
    {
        #[ink(constructor)]
        pub fn new(token_name: String, initial_supply: Balance) -> Self
        {
            ink_lang::utils::initialize_contract(|contract|{
                Self::new_init(contract, token_name, initial_supply)
            })
        }

        /// Initialize the ERC-20 contract with the specified initial supply.
        fn new_init(&mut self, token_name: String, initial_supply: Balance)
        {
            let caller = Self::env().caller();
            self.name = token_name;
            self.balances.insert(&caller, &initial_supply);
            self.total_supply = initial_supply;
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        #[inline]
        fn balance_of_impl(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance
        ) -> Result<()> {
            let from_balance = self.balance_of_impl(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of_impl(to);
            self.balances.insert(to, &(to_balance + value));
            Self::env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((&owner, &spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_impl(&owner, &spender)
        }

        #[inline]
        fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// Transfers tokens on the behalf of the `from` account to the `to account
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance_impl(&from, &caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            self.transfer_from_to(&from, &to, value)?;
            self.allowances.insert((&from, &caller), &(allowance - value));
            Ok(())
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    use ink_lang as ink;

    use ink_env::AccountId;

    use erc20::{Erc20, Error};

    #[ink::test]
    fn new_works()
    {
        let contract = Erc20::new("MyToken".to_string(), 777);
        assert_eq!(contract.get_name(), "MyToken".to_string());
        assert_eq!(contract.total_supply(), 777);
    }

    #[ink::test]
    fn balance_works()
    {
        let contract = Erc20::new("MyToken".to_string(), 100);
        assert_eq!(contract.total_supply(), 100);
        assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
        assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
    }

    #[ink::test]
    fn transfer_works() {
        let mut erc20 = Erc20::new("MyToken".to_string(), 100);
        assert_eq!(erc20.balance_of(AccountId::from([0x0; 32])), 0);
        assert_eq!(erc20.transfer(AccountId::from([0x0; 32]), 10), Ok(()));
        assert_eq!(erc20.balance_of(AccountId::from([0x0; 32])), 10);
    }

    #[ink::test]
    fn transfer_failed_insufficient_balance(){
        let mut erc20 = Erc20::new("MyToken".to_string(), 100);
        assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 100);
        assert_eq!(erc20.transfer(AccountId::from([0x1; 32]), 200), Err(Error::InsufficientBalance));
    }

    #[ink::test]
    fn allowances_works() {
        let mut contract = Erc20::new("MyToken".to_string(), 100);
        assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
        _ = contract.approve(AccountId::from([0x1; 32]), 200);
        assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x1; 32])), 200);

        _ = contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 50);
        assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 50);
        assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x1; 32])), 150);

        _ = contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 100);
        assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 50);
        assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x1; 32])), 150);
    }

    #[ink::test]
    fn transfer_from_works() {
        let mut contract = Erc20::new("MyToken".to_string(), 100);
        assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
        _ = contract.approve(AccountId::from([0x1; 32]), 20);
        _ = contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 10);
        assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
    }
    
    #[ink::test]
    fn transfer_from_failed_insufficient_allowance() {
        let mut contract = Erc20::new("MyToken".to_string(), 100);
        assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
        _ = contract.approve(AccountId::from([0x1; 32]), 20);
        assert_eq!(contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 30), Err(Error::InsufficientAllowance));
    }
}