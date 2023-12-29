use super::*;
use frame_support::BoundedVec;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(PoeModule::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(PoeModule::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			PoeModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let input: Vec<u8> = vec![0, 1];
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), input.clone()));
		let bounded_input =
			BoundedVec::<u8, <Test as Config>::ProofSizeLimit>::try_from(input.clone()).unwrap();
		assert_eq!(
			PoeModule::proofs(&bounded_input),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	});
}
