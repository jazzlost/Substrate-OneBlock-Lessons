use crate::mock::{Event as TestEvent, new_test_ext, Kitties as KittiesMod, Origin, System, Test};
use frame_support::{assert_noop, assert_ok};
// use super::*;
use crate::*;

/****************************************CREATE***************************************************/

/* balance: 100_000_000_000 */
const ACCOUNT_ID_1: u64 = 1;
/* balance: 100_000_000_000 */
const ACCOUNT_ID_2: u64 = 2;
/* balance: 9_999 */
const ACCOUNT_ID_3: u64 = 3;
/* balance: 20_000 */
const ACCOUNT_ID_4: u64 = 4;


#[test]
fn create_works()
{
	new_test_ext().execute_with(|| {		
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_eq!(KittyCount::<Test>::get(), 1);
		assert_eq!(KittyOwner::<Test>::get(0), Some(ACCOUNT_ID_1));
		
		assert_has_event!(Event::<Test>::KittyCreated(ACCOUNT_ID_1, 0, Kitties::<Test>::get(0).unwrap()));
	});
}

#[test]
fn create_failed_not_enough_balance_reserved()
{
	new_test_ext().execute_with(|| {
		assert_noop!(KittiesMod::create(Origin::signed(ACCOUNT_ID_3)), Error::<Test>::NotEnoughBalanceReserved);
	});
}

/****************************************BREED***************************************************/
#[test]
fn breed_works()
{
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_2)));
		assert_ok!(KittiesMod::breed(Origin::signed(ACCOUNT_ID_1), 0, 1));
		assert_eq!(KittyCount::<Test>::get(), 3);
		assert_eq!(KittyOwner::<Test>::get(2), Some(ACCOUNT_ID_1));
		
		assert_has_event!(Event::<Test>::KittyBreed(ACCOUNT_ID_1, 2, Kitties::<Test>::get(2).unwrap()));
	});
}

#[test]
fn breed_failed_same_kitty_id()
{
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_2)));
		assert_noop!(KittiesMod::breed(Origin::signed(ACCOUNT_ID_1), 1, 1), Error::<Test>::SameKittyId);
	})
}

#[test]
fn breed_failed_invalid_kitty_id()
{
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_2)));
		assert_noop!(KittiesMod::breed(Origin::signed(ACCOUNT_ID_1), 0, 2), Error::<Test>::InvalidKittyId);
	})
}

#[test]
fn breed_failed_not_enough_balance_reserved()
{
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_2)));
		assert_noop!(KittiesMod::breed(Origin::signed(ACCOUNT_ID_3), 0, 1), Error::<Test>::NotEnoughBalanceReserved);
	})
}

/****************************************TRANSFER***************************************************/
#[test]
fn transfer_works()
{
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_eq!(KittyOwner::<Test>::get(0), Some(ACCOUNT_ID_1));
		assert_ok!(KittiesMod::transfer(Origin::signed(ACCOUNT_ID_1), 0, ACCOUNT_ID_2));
		assert_eq!(KittyOwner::<Test>::get(0), Some(ACCOUNT_ID_2));
		
		assert_has_event!(Event::<Test>::KittyTransferred(ACCOUNT_ID_1, ACCOUNT_ID_2, 0));
	});
}

#[test]
fn transfer_failed_not_owner()
{
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_noop!(KittiesMod::transfer(Origin::signed(ACCOUNT_ID_2), 0, ACCOUNT_ID_1), Error::<Test>::NotOwner);
	})
}

#[test]
fn transfer_failed_invalid_kitty_id()
{
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_noop!(KittiesMod::transfer(Origin::signed(ACCOUNT_ID_1), 1, ACCOUNT_ID_2), Error::<Test>::InvalidKittyId);
	})
}

#[test]
fn transfer_failed_not_enough_balance_reserved()
{
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_noop!(KittiesMod::transfer(Origin::signed(ACCOUNT_ID_1), 0, ACCOUNT_ID_3), Error::<Test>::NotEnoughBalanceReserved);
	})
}
/****************************************SELL***************************************************/
#[test]
fn sell_works()
{
	new_test_ext().execute_with(|| {
		let price: u128 = 2_000;

		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_ok!(KittiesMod::sell(Origin::signed(ACCOUNT_ID_1), 0, Some(price)));
		assert_eq!(SaleList::<Test>::get(0), Some(price));

		assert_has_event!(Event::<Test>::KittyOnSale(ACCOUNT_ID_1, 0, Some(price)));
	});
}

#[test]
fn sell_failed_not_owner()
{
	let price: u128 = 2_000;

	new_test_ext().execute_with(|| {
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_noop!(KittiesMod::sell(Origin::signed(ACCOUNT_ID_2), 0, Some(price)), Error::<Test>::NotOwner);
	})
}


/****************************************BUY***************************************************/
#[test]
fn buy_works()
{
	new_test_ext().execute_with(|| {
		let price: u128 = 2_000;
	
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_ok!(KittiesMod::sell(Origin::signed(ACCOUNT_ID_1), 0, Some(price)));
		assert_ok!(KittiesMod::buy(Origin::signed(ACCOUNT_ID_2), 0));
		assert_eq!(KittyOwner::<Test>::get(0), Some(ACCOUNT_ID_2));
	
		assert_has_event!(Event::<Test>::KittySaled(ACCOUNT_ID_1, ACCOUNT_ID_2, 0, Some(price)));
	});
}

#[test]
fn buy_failed_already_owned()
{
	new_test_ext().execute_with(|| {
		let price: u128 = 2_000;

		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_ok!(KittiesMod::sell(Origin::signed(ACCOUNT_ID_1), 0, Some(price)));	
		assert_noop!(KittiesMod::buy(Origin::signed(ACCOUNT_ID_1), 0), Error::<Test>::AlreadyOwned);
	});
}

#[test]
fn buy_failed_not_for_sale()
{
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_noop!(KittiesMod::buy(Origin::signed(ACCOUNT_ID_2), 0), Error::<Test>::NotForSale);
	});
}

#[test]
fn buy_failed_not_enought_balance_buy()
{
	new_test_ext().execute_with(|| {
		let price:u128 = 11_000;
		
		assert_ok!(KittiesMod::create(Origin::signed(ACCOUNT_ID_1)));
		assert_ok!(KittiesMod::sell(Origin::signed(ACCOUNT_ID_1), 0, Some(price)));	
		assert_noop!(KittiesMod::buy(Origin::signed(ACCOUNT_ID_4), 0), Error::<Test>::NotEnoughBalanceBuy);
	});
}