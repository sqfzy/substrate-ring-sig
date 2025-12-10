use frame::{
    deps::{frame_support::weights::constants::RocksDbWeight, frame_system::GenesisConfig},
    prelude::*,
    runtime::prelude::*,
    testing_prelude::*,
};

// Configure a mock runtime to test the pallet.
#[frame_construct_runtime]
mod test_runtime {
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
    pub type Template = crate;

    #[runtime::pallet_index(2)]
    pub type Preimage = pallet_preimage;

    #[runtime::pallet_index(3)]
    pub type Scheduler = pallet_scheduler;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Nonce = u64;
    type Block = MockBlock<Test>;
    type BlockHashCount = ConstU64<250>;
    type DbWeight = RocksDbWeight;
}

impl crate::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl pallet_preimage::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Currency = ();
    type ManagerOrigin = EnsureRoot<u64>;
    type Consideration = ();
}

parameter_types! {
    pub MaximumSchedulerWeight: Weight = Weight::from_parts(10_000_000, 0);
}

impl pallet_scheduler::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<u64>;
    #[cfg(feature = "runtime-benchmarks")]
    type MaxScheduledPerBlock = ConstU32<512>;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type MaxScheduledPerBlock = ConstU32<50>;
    type WeightInfo = ();
    type OriginPrivilegeCmp = frame::traits::EqualPrivilegeOnly;
    type Preimages = pallet_preimage::Pallet<Test>;
    type BlockNumberProvider = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> TestState {
    GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
