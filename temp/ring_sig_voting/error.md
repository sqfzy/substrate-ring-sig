
error[E0277]: the trait bound `pallet_preimage::Pallet<Test>: Callable<Test>` is not satisfied in `UncheckedExtrinsic<u64, RuntimeCall, (), ()>`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ within `UncheckedExtrinsic<u64, RuntimeCall, (), ()>`, the trait `Callable<Test>` is not implemented for `pallet_preimage::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | ^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | --------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `Callable<T>`
   --> pallets/ring_sig_voting/src/lib.rs:191:15
    |
191 |     #[pallet::call]
    |               ^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:688:12
    |
688 |     #[pallet::call]
    |               ^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
note: required because it appears within the type `RuntimeCall`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `UncheckedExtrinsic<u64, RuntimeCall, (), ()>`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-runtime-41.1.0/src/generic/unchecked_extrinsic.rs:229:12
    |
229 | pub struct UncheckedExtrinsic<Address, Call, Signature, Extension> {
    |            ^^^^^^^^^^^^^^^^^^
    = note: required for `UncheckedExtrinsic<u64, RuntimeCall, (), ()>` to implement `Member`
    = note: required for `Block<Header<u64, BlakeTwo256>, UncheckedExtrinsic<u64, RuntimeCall, (), ()>>` to implement `BlockT`
    = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-17738408268004755159.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: Callable<Test>` is not satisfied in `UncheckedExtrinsic<u64, RuntimeCall, (), ()>`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ within `UncheckedExtrinsic<u64, RuntimeCall, (), ()>`, the trait `Callable<Test>` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | ^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | --------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `Callable<T>`
   --> pallets/ring_sig_voting/src/lib.rs:191:15
    |
191 |     #[pallet::call]
    |               ^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:688:12
    |
688 |     #[pallet::call]
    |               ^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
note: required because it appears within the type `RuntimeCall`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `UncheckedExtrinsic<u64, RuntimeCall, (), ()>`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-runtime-41.1.0/src/generic/unchecked_extrinsic.rs:229:12
    |
229 | pub struct UncheckedExtrinsic<Address, Call, Signature, Extension> {
    |            ^^^^^^^^^^^^^^^^^^
    = note: required for `UncheckedExtrinsic<u64, RuntimeCall, (), ()>` to implement `Member`
    = note: required for `Block<Header<u64, BlakeTwo256>, UncheckedExtrinsic<u64, RuntimeCall, (), ()>>` to implement `BlockT`
    = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-17738408268004755159.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_preimage::Pallet<Test>: Callable<Test>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Callable<Test>` is not implemented for `pallet_preimage::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | ^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | --------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `Callable<T>`
   --> pallets/ring_sig_voting/src/lib.rs:191:15
    |
191 |     #[pallet::call]
    |               ^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:688:12
    |
688 |     #[pallet::call]
    |               ^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: Callable<Test>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Callable<Test>` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | ^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | --------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `Callable<T>`
   --> pallets/ring_sig_voting/src/lib.rs:191:15
    |
191 |     #[pallet::call]
    |               ^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:688:12
    |
688 |     #[pallet::call]
    |               ^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_preimage::Pallet<Test>: Callable<Test>` is not satisfied in `UncheckedExtrinsic<u64, RuntimeCall, (), ()>`
   --> pallets/ring_sig_voting/src/mock.rs:46:18
    |
 46 |     type Block = MockBlock<Test>;
    |                  ^^^^^^^^^^^^^^^ within `UncheckedExtrinsic<u64, RuntimeCall, (), ()>`, the trait `Callable<Test>` is not implemented for `pallet_preimage::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | ^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | --------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `Callable<T>`
   --> pallets/ring_sig_voting/src/lib.rs:191:15
    |
191 |     #[pallet::call]
    |               ^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:688:12
    |
688 |     #[pallet::call]
    |               ^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
note: required because it appears within the type `RuntimeCall`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `UncheckedExtrinsic<u64, RuntimeCall, (), ()>`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-runtime-41.1.0/src/generic/unchecked_extrinsic.rs:229:12
    |
229 | pub struct UncheckedExtrinsic<Address, Call, Signature, Extension> {
    |            ^^^^^^^^^^^^^^^^^^
    = note: required for `UncheckedExtrinsic<u64, RuntimeCall, (), ()>` to implement `Member`
    = note: required for `Block<Header<u64, BlakeTwo256>, UncheckedExtrinsic<u64, RuntimeCall, (), ()>>` to implement `BlockT`
    = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-17738408268004755159.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: Callable<Test>` is not satisfied in `UncheckedExtrinsic<u64, RuntimeCall, (), ()>`
   --> pallets/ring_sig_voting/src/mock.rs:46:18
    |
 46 |     type Block = MockBlock<Test>;
    |                  ^^^^^^^^^^^^^^^ within `UncheckedExtrinsic<u64, RuntimeCall, (), ()>`, the trait `Callable<Test>` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | ^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | --------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `Callable<T>`
   --> pallets/ring_sig_voting/src/lib.rs:191:15
    |
191 |     #[pallet::call]
    |               ^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:688:12
    |
688 |     #[pallet::call]
    |               ^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
note: required because it appears within the type `RuntimeCall`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `UncheckedExtrinsic<u64, RuntimeCall, (), ()>`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-runtime-41.1.0/src/generic/unchecked_extrinsic.rs:229:12
    |
229 | pub struct UncheckedExtrinsic<Address, Call, Signature, Extension> {
    |            ^^^^^^^^^^^^^^^^^^
    = note: required for `UncheckedExtrinsic<u64, RuntimeCall, (), ()>` to implement `Member`
    = note: required for `Block<Header<u64, BlakeTwo256>, UncheckedExtrinsic<u64, RuntimeCall, (), ()>>` to implement `BlockT`
    = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-17738408268004755159.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied in `RuntimeEvent`
   --> pallets/ring_sig_voting/src/mock.rs:44:1
    |
 44 | #[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: within `RuntimeEvent`, the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
note: required because it appears within the type `pallet_preimage::Event<Test>`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:157:11
    |
157 |     pub enum Event<T: Config> {
    |              ^^^^^
note: required because it appears within the type `RuntimeEvent`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: required for `<Test as polkadot_sdk_frame::prelude::frame_system::Config>::RuntimeEvent` to implement `Member`
note: required by a bound in `polkadot_sdk_frame::prelude::frame_system::Config::RuntimeEvent`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:487:6
    |
486 |         type RuntimeEvent: Parameter
    |              ------------ required by a bound in this associated type
487 |             + Member
    |               ^^^^^^ required by this bound in `Config::RuntimeEvent`
    = note: this error originates in the macro `frame_system::config_preludes::TestDefaultConfig` which comes from the expansion of the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, ru
n with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:56:25
    |
 56 |     type RuntimeEvent = RuntimeEvent;
    |                         ^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:58:21
    |
 58 |     type Currency = ();
    |                     ^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:59:26
    |
 59 |     type ManagerOrigin = EnsureRoot<u64>;
    |                          ^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:60:26
    |
 60 |     type Consideration = ();
    |                          ^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:55:34
    |
 55 | impl pallet_preimage::Config for Test {
    |                                  ^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:68:25
    |
 68 |     type RuntimeEvent = RuntimeEvent;
    |                         ^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:69:26
    |
 69 |     type RuntimeOrigin = RuntimeOrigin;
    |                          ^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:70:26
    |
 70 |     type PalletsOrigin = OriginCaller;
    |                          ^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `RuntimeCall: From<frame_system::pallet::Call<Test>>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:71:24
    |
 71 |     type RuntimeCall = RuntimeCall;
    |                        ^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `From<frame_system::pallet::Call<Test>>` is not implemented for `RuntimeCall`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
help: the following other types implement trait `From<T>`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    | |
    | `RuntimeCall` implements `From<<pallet_preimage::Pallet<Test> as Callable<Test>>::RuntimeCall>`
    | `RuntimeCall` implements `From<<pallet_scheduler::Pallet<Test> as Callable<Test>>::RuntimeCall>`
    | `RuntimeCall` implements `From<pallet::Call<Test>>`
    | `RuntimeCall` implements `From<polkadot_sdk_frame::prelude::frame_system::Call<Test>>`
note: required by a bound in `pallet_scheduler::Config::RuntimeCall`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:286:6
    |
281 |         type RuntimeCall: Parameter
    |              ----------- required by a bound in this associated type
...
286 |             + From<system::Call<Self>>;
    |               ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Config::RuntimeCall`
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `RuntimeCall: frame_support::dispatch::GetDispatchInfo` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:71:24
    |
 71 |     type RuntimeCall = RuntimeCall;
    |                        ^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `frame_support::dispatch::GetDispatchInfo` is not implemented for `RuntimeCall`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:274:1
    |
274 | pub trait GetDispatchInfo {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:280:1
    |
280 | pub trait GetDispatchInfo {
    | ------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = help: the following other types implement trait `frame_support::dispatch::GetDispatchInfo`:
              ()
              frame_system::pallet::Call<T>
              pallet_preimage::Call<T>
              pallet_scheduler::Call<T>
              sp_runtime::generic::checked_extrinsic::CheckedExtrinsic<AccountId, Call, Extension>
              sp_runtime::generic::unchecked_extrinsic::UncheckedExtrinsic<Address, Call, Signature, Extension>
note: required by a bound in `pallet_scheduler::Config::RuntimeCall`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:285:8
    |
281 |         type RuntimeCall: Parameter
    |              ----------- required by a bound in this associated type
...
285 |             > + GetDispatchInfo
    |                 ^^^^^^^^^^^^^^^ required by this bound in `Config::RuntimeCall`
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `RuntimeCall: sp_runtime::traits::Dispatchable` is not satisfied
    --> pallets/ring_sig_voting/src/mock.rs:71:24
     |
  71 |     type RuntimeCall = RuntimeCall;
     |                        ^^^^^^^^^^^ unsatisfied trait bound
     |
help: the trait `sp_runtime::traits::Dispatchable` is not implemented for `RuntimeCall`
    --> pallets/ring_sig_voting/src/mock.rs:10:1
     |
  10 | #[frame_construct_runtime]
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `sp_runtime` in the dependency graph
    --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-runtime-44.0.0/src/traits/mod.rs:1534:1
     |
1534 | pub trait Dispatchable {
     | ^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
     |
    ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-runtime-41.1.0/src/traits/mod.rs:1525:1
     |
1525 | pub trait Dispatchable {
     | ---------------------- this is the found trait
     = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `sp_runtime::traits::Dispatchable`
    --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-runtime-44.0.0/src/traits/mod.rs:1567:1
     |
1567 | impl Dispatchable for () {
     | ^^^^^^^^^^^^^^^^^^^^^^^^ `()`
...
1600 | impl<Inner> Dispatchable for FakeDispatchable<Inner> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `sp_runtime::traits::FakeDispatchable<Inner>`
     |
    ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-runtime-44.0.0/src/testing.rs:290:1
     |
 290 | impl Dispatchable for MockCallU64 {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `sp_runtime::testing::MockCallU64`
note: required by a bound in `pallet_scheduler::Config::RuntimeCall`
    --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:282:6
     |
 281 |           type RuntimeCall: Parameter
     |                ----------- required by a bound in this associated type
 282 |               + Dispatchable<
     |  _______________^
 283 | |                 RuntimeOrigin = <Self as Config>::RuntimeOrigin,
 284 | |                 PostInfo = PostDispatchInfo,
 285 | |             > + GetDispatchInfo
     | |_____________^ required by this bound in `Config::RuntimeCall`
     = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `MaximumSchedulerWeight: bounded_collections::Get<sp_weights::weight_v2::Weight>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:72:26
    |
 72 |     type MaximumWeight = MaximumSchedulerWeight;
    |                          ^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `bounded_collections::Get<sp_weights::weight_v2::Weight>` is not implemented for `MaximumSchedulerWeight`
   --> pallets/ring_sig_voting/src/mock.rs:63:1
    |
 63 | / parameter_types! {
 64 | |     pub MaximumSchedulerWeight: Weight = Weight::from_parts(10_000_000, 0);
 65 | | }
    | |_^
note: there are multiple different versions of crate `bounded_collections` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bounded-collections-0.3.2/src/lib.rs:44:1
    |
 44 | pub trait Get<T> {
    | ^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bounded-collections-0.2.4/src/lib.rs:43:1
    |
 43 | pub trait Get<T> {
    | ---------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = help: the following other types implement trait `bounded_collections::Get<T>`:
              `()` implements `bounded_collections::Get<T>`
              `bounded_collections::ConstBool<T>` implements `bounded_collections::Get<bool>`
              `bounded_collections::ConstBool<T>` implements `bounded_collections::Get<std::option::Option<bool>>`
              `bounded_collections::ConstI128<T>` implements `bounded_collections::Get<i128>`
              `bounded_collections::ConstI128<T>` implements `bounded_collections::Get<std::option::Option<i128>>`
              `bounded_collections::ConstI16<T>` implements `bounded_collections::Get<i16>`
              `bounded_collections::ConstI16<T>` implements `bounded_collections::Get<std::option::Option<i16>>`
              `bounded_collections::ConstI32<T>` implements `bounded_collections::Get<i32>`
            and 50 others
note: required by a bound in `pallet_scheduler::Config::MaximumWeight`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:290:23
    |
290 |         type MaximumWeight: Get<Weight>;
    |                             ^^^^^^^^^^^ required by this bound in `Config::MaximumWeight`
    = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-8594780773490826105.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `parameter_types` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:73:27
    |
 73 |     type ScheduleOrigin = EnsureRoot<u64>;
    |                           ^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `polkadot_sdk_frame::prelude::ConstU32<50>: bounded_collections::Get<u32>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:77:33
    |
 77 |     type MaxScheduledPerBlock = ConstU32<50>;
    |                                 ^^^^^^^^^^^^ the trait `bounded_collections::Get<u32>` is not implemented for `polkadot_sdk_frame::prelude::ConstU32<50>`
    |
note: there are multiple different versions of crate `bounded_collections` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bounded-collections-0.3.2/src/lib.rs:44:1
    |
 44 | pub trait Get<T> {
    | ^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bounded-collections-0.2.4/src/lib.rs:43:1
    |
 43 | pub trait Get<T> {
    | ----------------
    | |
    | this is the found trait
    | this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = help: the following other types implement trait `bounded_collections::Get<T>`:
              `()` implements `bounded_collections::Get<T>`
              `bounded_collections::ConstBool<T>` implements `bounded_collections::Get<bool>`
              `bounded_collections::ConstBool<T>` implements `bounded_collections::Get<std::option::Option<bool>>`
              `bounded_collections::ConstI128<T>` implements `bounded_collections::Get<i128>`
              `bounded_collections::ConstI128<T>` implements `bounded_collections::Get<std::option::Option<i128>>`
              `bounded_collections::ConstI16<T>` implements `bounded_collections::Get<i16>`
              `bounded_collections::ConstI16<T>` implements `bounded_collections::Get<std::option::Option<i16>>`
              `bounded_collections::ConstI32<T>` implements `bounded_collections::Get<i32>`
            and 50 others
note: required by a bound in `pallet_scheduler::Config::MaxScheduledPerBlock`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:310:30
    |
310 |         type MaxScheduledPerBlock: Get<u32>;
    |                                    ^^^^^^^^ required by this bound in `Config::MaxScheduledPerBlock`
    = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-8594780773490826105.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0277]: the trait bound `EqualPrivilegeOnly: frame_support::traits::misc::PrivilegeCmp<OriginCaller>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:79:31
    |
 79 |     type OriginPrivilegeCmp = EqualPrivilegeOnly;
    |                               ^^^^^^^^^^^^^^^^^^ the trait `frame_support::traits::misc::PrivilegeCmp<OriginCaller>` is not implemented for `EqualPrivilegeOnly`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/misc.rs:857:1
    |
857 | pub trait PrivilegeCmp<Origin> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/misc.rs:843:1
    |
843 | pub trait PrivilegeCmp<Origin> {
    | ------------------------------ this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the trait `frame_support::traits::misc::PrivilegeCmp<Origin>` is implemented for `frame_support::traits::misc::EqualPrivilegeOnly`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/misc.rs:870:1
    |
870 | impl<Origin: PartialEq> PrivilegeCmp<Origin> for EqualPrivilegeOnly {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `pallet_scheduler::Config::OriginPrivilegeCmp`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:302:28
    |
302 |         type OriginPrivilegeCmp: PrivilegeCmp<Self::PalletsOrigin>;
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Config::OriginPrivilegeCmp`

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:80:22
    |
 80 |     type Preimages = pallet_preimage::Pallet<Test>;
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:67:35
    |
 67 | impl pallet_scheduler::Config for Test {
    |                                   ^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: polkadot_sdk_frame::traits::schedule::v3::Named<u64, RuntimeCall, RuntimeOrigin>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:87:22
    |
 87 |     type Scheduler = Scheduler;
    |                      ^^^^^^^^^ the trait `polkadot_sdk_frame::traits::schedule::v3::Named<u64, RuntimeCall, RuntimeOrigin>` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/schedule.rs:461:2
    |
461 |     pub trait Named<BlockNumber, Call, Origin> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/schedule.rs:195:2
    |
195 |     pub trait Named<BlockNumber, Call, RuntimeOrigin> {
    |     ------------------------------------------------- this is the found trait
...
366 |     pub trait Named<BlockNumber, Call, RuntimeOrigin> {
    |     ------------------------------------------------- this is the found trait
...
461 |     pub trait Named<BlockNumber, Call, Origin> {
    |     ------------------------------------------ this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
note: required by a bound in `pallet::Config::Scheduler`
   --> pallets/ring_sig_voting/src/lib.rs:50:17
    |
 45 |           type Scheduler: schedule::v3::Anon<
    |                --------- required by a bound in this associated type
...
 50 |               > + schedule::v3::Named<
    |  _________________^
 51 | |                 BlockNumberFor<Self>,
 52 | |                 RuntimeCallFor<Self>,
 53 | |                 Self::RuntimeOrigin,
 54 | |                 Hasher = Self::Hashing,
 55 | |             >;
    | |_____________^ required by this bound in `Config::Scheduler`

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: polkadot_sdk_frame::traits::schedule::v3::Anon<u64, RuntimeCall, RuntimeOrigin>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:87:22
    |
 87 |     type Scheduler = Scheduler;
    |                      ^^^^^^^^^ the trait `polkadot_sdk_frame::traits::schedule::v3::Anon<u64, RuntimeCall, RuntimeOrigin>` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/schedule.rs:412:2
    |
412 |     pub trait Anon<BlockNumber, Call, Origin> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/schedule.rs:150:2
    |
150 |     pub trait Anon<BlockNumber, Call, RuntimeOrigin> {
    |     ------------------------------------------------ this is the found trait
...
319 |     pub trait Anon<BlockNumber, Call, RuntimeOrigin> {
    |     ------------------------------------------------ this is the found trait
...
412 |     pub trait Anon<BlockNumber, Call, Origin> {
    |     ----------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
note: required by a bound in `pallet::Config::Scheduler`
   --> pallets/ring_sig_voting/src/lib.rs:45:25
    |
 45 |           type Scheduler: schedule::v3::Anon<
    |  _________________________^
 46 | |                 BlockNumberFor<Self>,
 47 | |                 RuntimeCallFor<Self>,
 48 | |                 Self::RuntimeOrigin,
 49 | |                 Hasher = Self::Hashing,
 50 | |             > + schedule::v3::Named<
    | |_____________^ required by this bound in `Config::Scheduler`

error[E0277]: the trait bound `pallet_preimage::Pallet<Test>: StorePreimage` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:88:22
    |
 88 |     type Preimages = Preimage;
    |                      ^^^^^^^^ the trait `StorePreimage` is not implemented for `pallet_preimage::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/preimages.rs:240:1
    |
240 | pub trait StorePreimage: QueryPreimage {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/preimages.rs:240:1
    |
240 | pub trait StorePreimage: QueryPreimage {
    | -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the trait `StorePreimage` is implemented for `()`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/preimages.rs:289:1
    |
289 | impl StorePreimage for () {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `pallet::Config::Preimages`
   --> pallets/ring_sig_voting/src/lib.rs:57:58
    |
 57 |             type Preimages: QueryPreimage<H = Self::Hashing> + StorePreimage;
    |                                                                ^^^^^^^^^^^^^ required by this bound in `Config::Preimages`

error[E0277]: the trait bound `pallet_preimage::Event<Test>: parity_scale_codec::Encode` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `parity_scale_codec::Encode` is not implemented for `pallet_preimage::Event<Test>`
    |
note: there are multiple different versions of crate `parity_scale_codec` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parity-scale-codec-3.7.5/src/codec.rs:226:1
    |
226 | pub trait Encode {
    | ^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/der-0.7.10/src/encode.rs:23:1
    |
 23 | pub trait Encode {
    | ---------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/jam-codec-0.1.1/src/codec.rs:226:1
    |
226 | pub trait Encode {
    | ---------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the trait `parity_scale_codec::Encode` is implemented for `pallet_preimage::Event<T>`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:122:1
    |
122 | #[frame_support::pallet]
    | ^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this error originates in the derive macro `self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::codec::Encode` which comes from the expansion of the derive macro `frame_support::
__private::codec::Encode` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_preimage::Event<Test>: Decode` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Decode` is not implemented for `pallet_preimage::Event<Test>`
    |
note: there are multiple different versions of crate `parity_scale_codec` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parity-scale-codec-3.7.5/src/codec.rs:296:1
    |
296 | pub trait Decode: Sized {
    | ^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/der-0.7.10/src/decode.rs:19:1
    |
 19 | pub trait Decode<'a>: Sized {
    | --------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/jam-codec-0.1.1/src/codec.rs:296:1
    |
296 | pub trait Decode: Sized {
    | ----------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the trait `Decode` is implemented for `pallet_preimage::Event<T>`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:122:1
    |
122 | #[frame_support::pallet]
    | ^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` which comes from the expansion of the derive macro `frame_support::__private::codec::Decode` (in Nightly builds, run 
with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the trait `DecodeWithMemTracking` is implemented for `pallet_preimage::Event<T>`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:122:1
    |
122 | #[frame_support::pallet]
    | ^^^^^^^^^^^^^^^^^^^^^^^^
    = note: required for `pallet_preimage::Event<Test>` to implement `DecodeWithMemTracking`
note: required by a bound in `mock::_::check_struct::check_field`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `check_field`
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` which comes from the expansion of the derive macro `self::sp_api_hidden_includes_construct_runtime::hidden_include::_
_private::codec::DecodeWithMemTracking` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    | |
    | unsatisfied trait bound
    | required by a bound introduced by this call
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the trait `scale_info::TypeInfo` is implemented for `pallet_preimage::Event<T>`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:122:1
    |
122 | #[frame_support::pallet]
    | ^^^^^^^^^^^^^^^^^^^^^^^^
    = note: required for `pallet_preimage::Event<Test>` to implement `scale_info::TypeInfo`
note: required by a bound in `FieldBuilder::<MetaForm, N>::ty`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/scale-info-2.11.6/src/build.rs:426:13
    |
424 |     pub fn ty<TY>(self) -> FieldBuilder<MetaForm, N, field_state::TypeAssigned>
    |            -- required by a bound in this associated function
425 |     where
426 |         TY: TypeInfo + 'static + ?Sized,
    |             ^^^^^^^^ required by this bound in `FieldBuilder::<MetaForm, N>::ty`
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` which comes from the expansion of the derive macro `frame_support::__private::scale_info::TypeInfo` (in Nightly build
s, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_preimage::Pallet<Test>: polkadot_sdk_frame::prelude::PalletInfoAccess` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `polkadot_sdk_frame::prelude::PalletInfoAccess` is not implemented for `pallet_preimage::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/metadata.rs:60:1
    |
 60 | pub trait PalletInfoAccess {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/metadata.rs:60:1
    |
 60 | pub trait PalletInfoAccess {
    | -------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `polkadot_sdk_frame::prelude::PalletInfoAccess`
   --> pallets/ring_sig_voting/src/lib.rs:30:15
    |
 30 |     #[pallet::pallet]
    |               ^^^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:677:12
    |
677 |     #[pallet::pallet]
    |               ^^^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: polkadot_sdk_frame::prelude::PalletInfoAccess` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `polkadot_sdk_frame::prelude::PalletInfoAccess` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/metadata.rs:60:1
    |
 60 | pub trait PalletInfoAccess {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/metadata.rs:60:1
    |
 60 | pub trait PalletInfoAccess {
    | -------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `polkadot_sdk_frame::prelude::PalletInfoAccess`
   --> pallets/ring_sig_voting/src/lib.rs:30:15
    |
 30 |     #[pallet::pallet]
    |               ^^^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:677:12
    |
677 |     #[pallet::pallet]
    |               ^^^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_preimage::Pallet<Test>: Callable<Test>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Callable<Test>` is not implemented for `pallet_preimage::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | ^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | --------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `Callable<T>`
   --> pallets/ring_sig_voting/src/lib.rs:191:15
    |
191 |     #[pallet::call]
    |               ^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:688:12
    |
688 |     #[pallet::call]
    |               ^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: Callable<Test>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Callable<Test>` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | ^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | --------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `Callable<T>`
   --> pallets/ring_sig_voting/src/lib.rs:191:15
    |
191 |     #[pallet::call]
    |               ^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:688:12
    |
688 |     #[pallet::call]
    |               ^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`

error: this function depends on never type fallback being `()`
  --> pallets/ring_sig_voting/src/mock.rs:10:1
   |
10 | #[frame_construct_runtime]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in Rust 2024 and in a future release in all editions!
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/never-type-fallback.html>
   = help: specify the types explicitly
note: in edition 2024, the requirement `!: WrapperTypeDecode` will fail
  --> pallets/ring_sig_voting/src/mock.rs:10:1
   |
10 | #[frame_construct_runtime]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: `#[deny(dependency_on_unit_never_type_fallback)]` (part of `#[deny(rust_2024_compatibility)]`) on by default
   = note: this error originates in the derive macro `self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::codec::Decode` which comes from the expansion of the attribute macro `frame::deps::
frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_preimage::Pallet<Test>: Callable<Test>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Callable<Test>` is not implemented for `pallet_preimage::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | ^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | --------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `Callable<T>`
   --> pallets/ring_sig_voting/src/lib.rs:191:15
    |
191 |     #[pallet::call]
    |               ^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:688:12
    |
688 |     #[pallet::call]
    |               ^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
    = note: this error originates in the derive macro `self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::RuntimeDebug` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: Callable<Test>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Callable<Test>` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | ^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | --------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `Callable<T>`
   --> pallets/ring_sig_voting/src/lib.rs:191:15
    |
191 |     #[pallet::call]
    |               ^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:688:12
    |
688 |     #[pallet::call]
    |               ^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
    = note: this error originates in the derive macro `self::sp_api_hidden_includes_construct_runtime::hidden_include::__private::RuntimeDebug` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_preimage::Pallet<Test>: Callable<Test>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:34:14
    |
 34 |     pub type Preimage = pallet_preimage;
    |              ^^^^^^^^ the trait `Callable<Test>` is not implemented for `pallet_preimage::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | ^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | --------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `Callable<T>`
   --> pallets/ring_sig_voting/src/lib.rs:191:15
    |
191 |     #[pallet::call]
    |               ^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:688:12
    |
688 |     #[pallet::call]
    |               ^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: Callable<Test>` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:37:14
    |
 37 |     pub type Scheduler = pallet_scheduler;
    |              ^^^^^^^^^ the trait `Callable<Test>` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | ^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/dispatch.rs:52:1
    |
 52 | pub trait Callable<T> {
    | --------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `Callable<T>`
   --> pallets/ring_sig_voting/src/lib.rs:191:15
    |
191 |     #[pallet::call]
    |               ^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:688:12
    |
688 |     #[pallet::call]
    |               ^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`

error[E0282]: type annotations needed
  --> pallets/ring_sig_voting/src/mock.rs:10:1
   |
10 | #[frame_construct_runtime]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
   |
   = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_preimage::Pallet<Test>: ViewFunctionIdPrefix` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:34:14
    |
 34 |     pub type Preimage = pallet_preimage;
    |              ^^^^^^^^ the trait `ViewFunctionIdPrefix` is not implemented for `pallet_preimage::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/view_functions.rs:91:1
    |
 91 | pub trait ViewFunctionIdPrefix {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/view_functions.rs:91:1
    |
 91 | pub trait ViewFunctionIdPrefix {
    | ------------------------------ this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `ViewFunctionIdPrefix`
   --> pallets/ring_sig_voting/src/lib.rs:17:1
    |
 17 | #[frame::pallet]
    | ^^^^^^^^^^^^^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:292:1
    |
292 | #[frame_support::pallet]
    | ^^^^^^^^^^^^^^^^^^^^^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
    = note: this error originates in the attribute macro `frame::pallet` which comes from the expansion of the attribute macro `frame_support::pallet` (in Nightly builds, run with -Z macro-backtrace for more info
)

error[E0277]: the trait bound `pallet_preimage::Pallet<Test>: DispatchViewFunction` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:34:14
    |
 34 |     pub type Preimage = pallet_preimage;
    |              ^^^^^^^^ the trait `DispatchViewFunction` is not implemented for `pallet_preimage::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/view_functions.rs:72:1
    |
 72 | pub trait DispatchViewFunction {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/view_functions.rs:72:1
    |
 72 | pub trait DispatchViewFunction {
    | ------------------------------ this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `DispatchViewFunction`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/view_functions.rs:80:1
    |
 80 | impl DispatchViewFunction for () {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()`
    |
   ::: pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ `mock::RuntimeViewFunction`
    |
   ::: pallets/ring_sig_voting/src/lib.rs:17:1
    |
 17 | #[frame::pallet]
    | ^^^^^^^^^^^^^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:292:1
    |
292 | #[frame_support::pallet]
    | ^^^^^^^^^^^^^^^^^^^^^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
    = note: this error originates in the attribute macro `frame::pallet` which comes from the expansion of the attribute macro `frame_support::pallet` (in Nightly builds, run with -Z macro-backtrace for more info
)

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: ViewFunctionIdPrefix` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:37:14
    |
 37 |     pub type Scheduler = pallet_scheduler;
    |              ^^^^^^^^^ the trait `ViewFunctionIdPrefix` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/view_functions.rs:91:1
    |
 91 | pub trait ViewFunctionIdPrefix {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/view_functions.rs:91:1
    |
 91 | pub trait ViewFunctionIdPrefix {
    | ------------------------------ this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `ViewFunctionIdPrefix`
   --> pallets/ring_sig_voting/src/lib.rs:17:1
    |
 17 | #[frame::pallet]
    | ^^^^^^^^^^^^^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:292:1
    |
292 | #[frame_support::pallet]
    | ^^^^^^^^^^^^^^^^^^^^^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
    = note: this error originates in the attribute macro `frame::pallet` which comes from the expansion of the attribute macro `frame_support::pallet` (in Nightly builds, run with -Z macro-backtrace for more info
)

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: DispatchViewFunction` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:37:14
    |
 37 |     pub type Scheduler = pallet_scheduler;
    |              ^^^^^^^^^ the trait `DispatchViewFunction` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/view_functions.rs:72:1
    |
 72 | pub trait DispatchViewFunction {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/view_functions.rs:72:1
    |
 72 | pub trait DispatchViewFunction {
    | ------------------------------ this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the following other types implement trait `DispatchViewFunction`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/view_functions.rs:80:1
    |
 80 | impl DispatchViewFunction for () {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()`
    |
   ::: pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ `mock::RuntimeViewFunction`
    |
   ::: pallets/ring_sig_voting/src/lib.rs:17:1
    |
 17 | #[frame::pallet]
    | ^^^^^^^^^^^^^^^^ `pallet::Pallet<T>`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:292:1
    |
292 | #[frame_support::pallet]
    | ^^^^^^^^^^^^^^^^^^^^^^^^ `polkadot_sdk_frame::prelude::frame_system::Pallet<T>`
    = note: this error originates in the attribute macro `frame::pallet` which comes from the expansion of the attribute macro `frame_support::pallet` (in Nightly builds, run with -Z macro-backtrace for more info
)

error[E0308]: mismatched types
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    | |
    | expected `PalletStorageMetadataIR`, found `sp_metadata_ir::types::PalletStorageMetadataIR`
    | arguments to this enum variant are incorrect
    |
note: there are multiple different versions of crate `sp_metadata_ir` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.10.0/src/types.rs:315:1
    |
315 | pub struct PalletStorageMetadataIR<T: Form = MetaForm> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected type
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.12.0/src/types.rs:315:1
    |
315 | pub struct PalletStorageMetadataIR<T: Form = MetaForm> {
    | ------------------------------------------------------ this is the found type
    = help: you can use `cargo tree` to explore your dependency tree
help: the type constructed contains `sp_metadata_ir::types::PalletStorageMetadataIR` due to the type of the argument passed
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ this argument influences the type of `Some`
note: tuple variant defined here
   --> /home/sqfzy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:608:5
    |
608 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    | |
    | expected `PalletCallMetadataIR`, found `sp_metadata_ir::types::PalletCallMetadataIR`
    | arguments to this enum variant are incorrect
    |
note: there are multiple different versions of crate `sp_metadata_ir` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.10.0/src/types.rs:432:1
    |
432 | pub struct PalletCallMetadataIR<T: Form = MetaForm> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected type
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.12.0/src/types.rs:432:1
    |
432 | pub struct PalletCallMetadataIR<T: Form = MetaForm> {
    | --------------------------------------------------- this is the found type
    = help: you can use `cargo tree` to explore your dependency tree
help: the type constructed contains `sp_metadata_ir::types::PalletCallMetadataIR` due to the type of the argument passed
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ this argument influences the type of `Some`
note: tuple variant defined here
   --> /home/sqfzy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:608:5
    |
608 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `PalletViewFunctionMetadataIR`, found `sp_metadata_ir::types::PalletViewFunctionMetadataIR`
    |
note: there are multiple different versions of crate `sp_metadata_ir` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.10.0/src/types.rs:126:1
    |
126 | pub struct PalletViewFunctionMetadataIR<T: Form = MetaForm> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected type
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.12.0/src/types.rs:126:1
    |
126 | pub struct PalletViewFunctionMetadataIR<T: Form = MetaForm> {
    | ----------------------------------------------------------- this is the found type
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
help: the trait `scale_info::TypeInfo` is implemented for `pallet_preimage::Event<T>`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:122:1
    |
122 | #[frame_support::pallet]
    | ^^^^^^^^^^^^^^^^^^^^^^^^
    = note: required for `pallet_preimage::Event<Test>` to implement `scale_info::TypeInfo`
note: required by a bound in `pallet_preimage::Event::<T>::event_metadata`
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:122:1
    |
122 | #[frame_support::pallet]
    | ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Event::<T>::event_metadata`
...
155 |     #[pallet::event]
    |               ----- required by a bound in this associated function
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` which comes from the expansion of the attribute macro `frame_support::pallet` (in Nightly builds, run with -Z macro-b
acktrace for more info)

error[E0308]: mismatched types
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    | |
    | expected `PalletEventMetadataIR`, found `sp_metadata_ir::types::PalletEventMetadataIR`
    | arguments to this enum variant are incorrect
    |
note: there are multiple different versions of crate `sp_metadata_ir` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.10.0/src/types.rs:452:1
    |
452 | pub struct PalletEventMetadataIR<T: Form = MetaForm> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected type
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.12.0/src/types.rs:452:1
    |
452 | pub struct PalletEventMetadataIR<T: Form = MetaForm> {
    | ---------------------------------------------------- this is the found type
    = help: you can use `cargo tree` to explore your dependency tree
help: the type constructed contains `sp_metadata_ir::types::PalletEventMetadataIR` due to the type of the argument passed
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ this argument influences the type of `Some`
note: tuple variant defined here
   --> /home/sqfzy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:608:5
    |
608 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `PalletConstantMetadataIR`, found `sp_metadata_ir::types::PalletConstantMetadataIR`
    |
note: there are multiple different versions of crate `sp_metadata_ir` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.10.0/src/types.rs:472:1
    |
472 | pub struct PalletConstantMetadataIR<T: Form = MetaForm> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected type
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.12.0/src/types.rs:472:1
    |
472 | pub struct PalletConstantMetadataIR<T: Form = MetaForm> {
    | ------------------------------------------------------- this is the found type
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `PalletErrorMetadataIR`, found `sp_metadata_ir::types::PalletErrorMetadataIR`
    |
note: there are multiple different versions of crate `sp_metadata_ir` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.10.0/src/types.rs:501:1
    |
501 | pub struct PalletErrorMetadataIR<T: Form = MetaForm> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected type
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.12.0/src/types.rs:501:1
    |
501 | pub struct PalletErrorMetadataIR<T: Form = MetaForm> {
    | ---------------------------------------------------- this is the found type
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `PalletAssociatedTypeMetadataIR`, found `sp_metadata_ir::types::PalletAssociatedTypeMetadataIR`
    |
note: there are multiple different versions of crate `sp_metadata_ir` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.10.0/src/types.rs:268:1
    |
268 | pub struct PalletAssociatedTypeMetadataIR<T: Form = MetaForm> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected type
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-metadata-ir-0.12.0/src/types.rs:268:1
    |
268 | pub struct PalletAssociatedTypeMetadataIR<T: Form = MetaForm> {
    | ------------------------------------------------------------- this is the found type
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
  --> pallets/ring_sig_voting/src/mock.rs:10:1
   |
10 | #[frame_construct_runtime]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `DeprecationStatusIR`, found `ItemDeprecationInfoIR`
   |
   = note: expected enum `DeprecationStatusIR`
              found enum `sp_metadata_ir::types::ItemDeprecationInfoIR`
   = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_preimage::Pallet<Test>: OnGenesis` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `OnGenesis` is not implemented for `pallet_preimage::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/hooks.rs:162:1
    |
162 | pub trait OnGenesis {
    | ^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/hooks.rs:152:2
    |
152 |     pub trait OnGenesis {
    |     ------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = help: the following other types implement trait `OnGenesis`:
              ()
              (TupleElement0, TupleElement1)
              (TupleElement0, TupleElement1, TupleElement2)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5, TupleElement6)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5, TupleElement6, TupleElement7)
            and 59 others
    = note: required for `(Pallet<Test>, Pallet<Test>, Pallet<Test>, Pallet<Test>)` to implement `OnGenesis`
    = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-12223421509191747680.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: OnGenesis` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `OnGenesis` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/hooks.rs:162:1
    |
162 | pub trait OnGenesis {
    | ^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/hooks.rs:152:2
    |
152 |     pub trait OnGenesis {
    |     ------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = help: the following other types implement trait `OnGenesis`:
              ()
              (TupleElement0, TupleElement1)
              (TupleElement0, TupleElement1, TupleElement2)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5, TupleElement6)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5, TupleElement6, TupleElement7)
            and 59 others
    = note: required for `(Pallet<Test>, Pallet<Test>, Pallet<Test>, Pallet<Test>)` to implement `OnGenesis`
    = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-12223421509191747680.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: the method `extrinsics` exists for reference `&Block<Header<u64, BlakeTwo256>, UncheckedExtrinsic<u64, RuntimeCall, (), ()>>`, but its trait bounds were not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ field, not a method
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-runtime-41.1.0/src/generic/block.rs:87:1
    |
 87 | pub struct Block<Header, Extrinsic> {
    | ----------------------------------- doesn't satisfy `_: Block`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-runtime-41.1.0/src/generic/unchecked_extrinsic.rs:229:1
    |
229 | pub struct UncheckedExtrinsic<Address, Call, Signature, Extension> {
    | ------------------------------------------------------------------ doesn't satisfy `_: Member`
    |
    = note: the following trait bounds were not satisfied:
            `UncheckedExtrinsic<u64, RuntimeCall, (), ()>: Member`
            which is required by `polkadot_sdk_frame::deps::sp_runtime::generic::Block<polkadot_sdk_frame::deps::sp_runtime::generic::Header<u64, BlakeTwo256>, UncheckedExtrinsic<u64, RuntimeCall, (), ()>>: Block
T`
    = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-13701518103073082482.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: the method `extrinsics` exists for reference `&Block<Header<u64, BlakeTwo256>, UncheckedExtrinsic<u64, RuntimeCall, (), ()>>`, but its trait bounds were not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ field, not a method
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-runtime-41.1.0/src/generic/block.rs:87:1
    |
 87 | pub struct Block<Header, Extrinsic> {
    | ----------------------------------- doesn't satisfy `_: Block`
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sp-runtime-41.1.0/src/generic/unchecked_extrinsic.rs:229:1
    |
229 | pub struct UncheckedExtrinsic<Address, Call, Signature, Extension> {
    | ------------------------------------------------------------------ doesn't satisfy `_: Member`
    |
    = note: the following trait bounds were not satisfied:
            `UncheckedExtrinsic<u64, RuntimeCall, (), ()>: Member`
            which is required by `polkadot_sdk_frame::deps::sp_runtime::generic::Block<polkadot_sdk_frame::deps::sp_runtime::generic::Header<u64, BlakeTwo256>, UncheckedExtrinsic<u64, RuntimeCall, (), ()>>: Block
T`
    = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-8736837854635821839.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no variant or associated item named `VARIANT_COUNT` found for enum `HoldReason` in the current scope
  --> pallets/ring_sig_voting/src/mock.rs:10:1
   |
10 | #[frame_construct_runtime]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ variant or associated item not found in `HoldReason`
   |
note: there are multiple different versions of crate `frame_support` in the dependency graph
  --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/misc.rs:45:1
   |
45 | pub trait VariantCount {
   | ^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
   |
  ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/misc.rs:42:1
   |
42 | pub trait VariantCount {
   | ---------------------- this is the trait that was imported
   = help: you can use `cargo tree` to explore your dependency tree
   = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_preimage::Pallet<Test>: IntegrityTest` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `IntegrityTest` is not implemented for `pallet_preimage::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/hooks.rs:341:1
    |
341 | pub trait IntegrityTest {
    | ^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/hooks.rs:328:2
    |
328 |     pub trait IntegrityTest {
    |     ----------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = help: the following other types implement trait `IntegrityTest`:
              ()
              (TupleElement0, TupleElement1)
              (TupleElement0, TupleElement1, TupleElement2)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5, TupleElement6)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5, TupleElement6, TupleElement7)
            and 59 others
    = note: required for `(Pallet<Test>, Pallet<Test>, Pallet<Test>, Pallet<Test>)` to implement `IntegrityTest`
    = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-12223421509191747680.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_scheduler::Pallet<Test>: IntegrityTest` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `IntegrityTest` is not implemented for `pallet_scheduler::Pallet<Test>`
    |
note: there are multiple different versions of crate `frame_support` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/hooks.rs:341:1
    |
341 | pub trait IntegrityTest {
    | ^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/hooks.rs:328:2
    |
328 |     pub trait IntegrityTest {
    |     ----------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = help: the following other types implement trait `IntegrityTest`:
              ()
              (TupleElement0, TupleElement1)
              (TupleElement0, TupleElement1, TupleElement2)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5, TupleElement6)
              (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5, TupleElement6, TupleElement7)
            and 59 others
    = note: required for `(Pallet<Test>, Pallet<Test>, Pallet<Test>, Pallet<Test>)` to implement `IntegrityTest`
    = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-12223421509191747680.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_preimage::Error<Test>: PalletError` is not satisfied
  --> pallets/ring_sig_voting/src/mock.rs:10:1
   |
10 | #[frame_construct_runtime]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `PalletError` is not implemented for `pallet_preimage::Error<Test>`
   |
note: there are multiple different versions of crate `frame_support` in the dependency graph
  --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/error.rs:32:1
   |
32 | pub trait PalletError: Encode + Decode {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
   |
  ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/error.rs:32:1
   |
32 | pub trait PalletError: Encode + Decode {
   | -------------------------------------- this is the found trait
   = help: you can use `cargo tree` to explore your dependency tree
   = help: the following other types implement trait `PalletError`:
             ()
             (TupleElement0, TupleElement1)
             (TupleElement0, TupleElement1, TupleElement2)
             (TupleElement0, TupleElement1, TupleElement2, TupleElement3)
             (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4)
             (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5)
             (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5, TupleElement6)
             (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5, TupleElement6, TupleElement7)
           and 36 others
   = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-12223421509191747680.txt'
   = note: consider using `--verbose` to print the full type name to the console
   = note: this error originates in the macro `self::sp_api_hidden_includes_construct_runtime::hidden_include::assert_error_encoded_size` which comes from the expansion of the attribute macro `frame::deps::frame_
support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `pallet_scheduler::Error<Test>: PalletError` is not satisfied
  --> pallets/ring_sig_voting/src/mock.rs:10:1
   |
10 | #[frame_construct_runtime]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `PalletError` is not implemented for `pallet_scheduler::Error<Test>`
   |
note: there are multiple different versions of crate `frame_support` in the dependency graph
  --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-40.1.0/src/traits/error.rs:32:1
   |
32 | pub trait PalletError: Encode + Decode {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
   |
  ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-support-43.0.0/src/traits/error.rs:32:1
   |
32 | pub trait PalletError: Encode + Decode {
   | -------------------------------------- this is the found trait
   = help: you can use `cargo tree` to explore your dependency tree
   = help: the following other types implement trait `PalletError`:
             ()
             (TupleElement0, TupleElement1)
             (TupleElement0, TupleElement1, TupleElement2)
             (TupleElement0, TupleElement1, TupleElement2, TupleElement3)
             (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4)
             (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5)
             (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5, TupleElement6)
             (TupleElement0, TupleElement1, TupleElement2, TupleElement3, TupleElement4, TupleElement5, TupleElement6, TupleElement7)
           and 36 others
   = note: the full name for the type has been written to '/home/sqfzy/work_space/work_code/rust/parachain-template/target/debug/deps/ring_sig_voting-16f0ef09a7cd0a33.long-type-12223421509191747680.txt'
   = note: consider using `--verbose` to print the full type name to the console
   = note: this error originates in the macro `self::sp_api_hidden_includes_construct_runtime::hidden_include::assert_error_encoded_size` which comes from the expansion of the attribute macro `frame::deps::frame_
support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no function or associated item named `on_initialize` found for struct `pallet_scheduler::Pallet<T>` in the current scope
   --> pallets/ring_sig_voting/src/mock.rs:118:20
    |
118 |         Scheduler::on_initialize(System::block_number());
    |                    ^^^^^^^^^^^^^ function or associated item not found in `pallet_scheduler::Pallet<Test>`

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the derive macro `Clone` which comes from the expansion of the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more i
nfo)

error[E0277]: the trait bound `Test: frame_system::pallet::Config` is not satisfied
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `frame_system::pallet::Config` is not implemented for `Test`
   --> pallets/ring_sig_voting/src/mock.rs:10:1
    |
 10 | #[frame_construct_runtime]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: there are multiple different versions of crate `frame_system` in the dependency graph
   --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-43.0.1/src/lib.rs:499:2
    |
499 |       pub trait Config: 'static + Eq + Clone {
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the expected trait
    |
   ::: pallets/ring_sig_voting/src/lib.rs:35:5
    |
 35 |       pub trait Config: frame_system::Config {
    |       -------------------------------------- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-preimage-43.0.0/src/lib.rs:131:2
    |
131 | /     pub trait Config: frame_system::Config {
132 | |         /// The overarching event type.
133 | |         #[allow(deprecated)]
134 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
148 | |         type Consideration: Consideration<Self::AccountId, Footprint>;
149 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pallet-scheduler-44.0.0/src/lib.rs:265:2
    |
265 | /     pub trait Config: frame_system::Config {
266 | |         /// The overarching event type.
267 | |         #[allow(deprecated)]
268 | |         type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
...   |
345 | |         type BlockNumberProvider: BlockNumberProvider;
346 | |     }
    | |_____- this is the found trait
    |
   ::: /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/frame-system-40.1.0/src/lib.rs:483:2
    |
483 |       pub trait Config: 'static + Eq + Clone {
    |       -------------------------------------- this is the found trait
    = help: you can use `cargo tree` to explore your dependency tree
    = note: this error originates in the attribute macro `frame::deps::frame_support::runtime` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0277, E0282, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
warning: `ring_sig_voting` (lib test) generated 8 warnings (6 duplicates)
error: could not compile `ring_sig_voting` (lib test) due to 126 previous errors; 8 warnings emitted
error: command `/home/sqfzy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo test --no-run --message-format json-render-diagnostics --package ring_sig_voting` exited with code 101
