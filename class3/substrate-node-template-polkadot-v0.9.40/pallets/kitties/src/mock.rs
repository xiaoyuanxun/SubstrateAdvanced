use crate as pallet_kitties;
use frame_support::traits::{
	ConstU16, ConstU32, ConstU64, ConstU128
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use pallet_insecure_randomness_collective_flip;
use pallet_balances;
use frame_support::{parameter_types, PalletId};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		KittiesModule: pallet_kitties,
		Randomness: pallet_insecure_randomness_collective_flip,
		Balances: pallet_balances,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	// type AccountData = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 500;

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
}

/// Balance of an account.
pub type Balance = u128;

parameter_types! {
	pub KittyPalletId: PalletId = PalletId(*b"py/kitty");
	pub KittyPrice: Balance = EXISTENTIAL_DEPOSIT * 10;
}

impl pallet_kitties::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Randomness = Randomness;
	type Currency = Balances;
	type KittyPrice = KittyPrice;
	type PalletId = KittyPalletId;
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

// Build genesis storage according to the mock runtime.
use frame_support::storage::{StorageMap, StorageValue};

pub fn new_test_ext() -> sp_io::TestExternalities {
	// let mut ext: sp_io::TestExternalities = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	// ext.execute_with(|| System::set_block_number(1));
	// // pallet_balances::GenesisConfig::<Test> {
	// // 	balances: vec![(1, 10000), (2, 10000), (3, 10000)],
	// // }.assimilate_storage(&mut ext).unwrap();
	// pallet_balances::GenesisConfig::<Test> {
	// 	balances: vec![(1, 10000), (2, 10000), (3, 10000)],
	// }.build_storage()?;
	// ext

    // 设置初始余额
    let balances_config = pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 10000000), (2, 10000000), (3, 10000000)],
    };

    // 构建 GenesisConfig
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
	
    // 合并 Balances 模块的 GenesisConfig
    balances_config.assimilate_storage(&mut t).unwrap();

    // 创建测试外部环境
    let mut ext: sp_io::TestExternalities = t.into();

    // 在测试外部环境中执行其他操作
    ext.execute_with(|| {
        // 在这里执行其他操作
		System::set_block_number(1)
    });

	ext
}
