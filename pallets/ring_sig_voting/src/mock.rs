use crate as ring_sig_voting;
use frame::{runtime::prelude::*, testing_prelude::*, prelude::*};
use frame::{
    traits::{schedule,EqualPrivilegeOnly},
};
use polkadot_sdk::{pallet_preimage, pallet_scheduler};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.  
#[frame_construct_runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask,
        RuntimeViewFunction
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system;

    #[runtime::pallet_index(1)]
    pub type Preimage = pallet_preimage;

    #[runtime::pallet_index(2)]
    pub type Scheduler = pallet_scheduler;

    #[runtime::pallet_index(3)]
    pub type RingSigVoting = ring_sig_voting;
}

parameter_types! {
	pub MaxWeight: Weight = Weight::from_parts(2_000_000_000_000, u64::MAX);
}

// System pallet configuration
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = MockBlock<Test>;
}

impl pallet_preimage::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Currency = ();
	type ManagerOrigin = EnsureRoot<u64>;
	type Consideration = ();
}
impl pallet_scheduler::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaxWeight;
	type ScheduleOrigin = EnsureRoot<u64>;
	type MaxScheduledPerBlock = ConstU32<100>;
	type WeightInfo = ();
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type Preimages = Preimage;
	type BlockNumberProvider = frame_system::Pallet<Test>;
}

impl ring_sig_voting::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type Scheduler = Scheduler;
    type Preimages = Preimage;
    type MaxDescriptionLength = ConstU32<256>;
    type MaxRingSize = ConstU32<16>;
    type MaxVkLength = ConstU32<2048>;
    type MaxCiphertextLength = ConstU32<128>;
    type MaxVoteNum = ConstU32<1000>;
    type AdminOrigin = frame_system::EnsureRoot<u64>;
}

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHARLIE: u64 = 3;

// Test externalities initialization
pub fn new_test_ext() -> TestExternalities {
    let storage = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    storage.into()
}

// Helper function to run to block n
pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        if System::block_number() > 0 {
            System::on_finalize(System::block_number());
        }
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        Scheduler::on_initialize(System::block_number());
    }
}
