use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);

		assert_eq!(KittiesModule::kitty_owner(kitty_id).unwrap(), account_id);
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);
    });
}

#[test]
fn it_works_for_kitty_created_event() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		let kitty = KittiesModule::kitties(kitty_id).unwrap();


		System::assert_has_event(Event::KittyCreated { who: account_id, kitty_id, kitty }.into() );
    });	
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
			Error::<Test>::SameKittyId
		);

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1),
			Error::<Test>::InvalidKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

		assert_ok!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1)
		);

		let breed_kitty_id = 2;
		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id).unwrap(), account_id);
		assert_eq!(
			KittiesModule::kitty_parents(breed_kitty_id).unwrap(),
			(kitty_id, kitty_id + 1)
		);
	});
}

#[test]
fn it_works_for_kitty_breed_event() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		let kitty_1 = KittiesModule::kitties(kitty_id).unwrap();
		let kitty_2 = KittiesModule::kitties(kitty_id + 1).unwrap();

		System::assert_has_event(
			Event::KittyCreated { who: account_id, kitty_id, kitty: kitty_1 }.into() 
		);
		System::assert_has_event(
			Event::KittyCreated { who: account_id, kitty_id: kitty_id + 1, kitty: kitty_2 }.into() 
		);

		assert_ok!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1)
		);
		let breed_kitty = KittiesModule::kitties(kitty_id + 2).unwrap();
		System::assert_has_event(
			Event::KittyBreed { who: account_id, kitty_id: kitty_id + 2, kitty: breed_kitty }.into()
		);
    });	
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id: u32 = 0;
		let account_id: u64 = 1;
		let to_account_id: u64 = 2;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(
			KittiesModule::kitty_owner(kitty_id).unwrap(),
            account_id
		);

		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(account_id), to_account_id, kitty_id + 1),
            Error::<Test>::InvalidKittyId
		);

		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(to_account_id), to_account_id, kitty_id),
            Error::<Test>::NotOwner
		);

		assert_ok!(
			KittiesModule::transfer(RuntimeOrigin::signed(account_id), to_account_id, kitty_id)
		);
		assert_eq!(
			KittiesModule::kitty_owner(kitty_id).unwrap(),
			to_account_id
		);

		assert_ok!(
			KittiesModule::transfer(RuntimeOrigin::signed(to_account_id), account_id, kitty_id)
		);
		assert_eq!(
			KittiesModule::kitty_owner(kitty_id).unwrap(),
			account_id
		);
	});
}

#[test]
fn it_works_for_kitty_transferred_event() {
	new_test_ext().execute_with(|| {
		let kitty_id: u32 = 0;
		let account_id: u64 = 1;
		let to_account_id: u64 = 2;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		let kitty = KittiesModule::kitties(kitty_id).unwrap();
		System::assert_has_event(
			Event::KittyCreated { who: account_id, kitty_id, kitty }.into() 
		);

		assert_ok!(
			KittiesModule::transfer(RuntimeOrigin::signed(account_id), to_account_id, kitty_id)
		);
		System::assert_has_event(
			Event::KittyTransferred { from: account_id, to: to_account_id, kitty_id}.into()
		);

		assert_ok!(
			KittiesModule::transfer(RuntimeOrigin::signed(to_account_id), account_id, kitty_id)
		);
		System::assert_has_event(
			Event::KittyTransferred { from: to_account_id, to: account_id, kitty_id}.into()
		);
	});
}