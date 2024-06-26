use libafl::corpus::{InMemoryCorpus, OnDiskCorpus};
use libafl::events::SimpleEventManager;
use libafl::executors::{ExitKind, InProcessExecutor};
use libafl::feedbacks::{ConstFeedback, CrashFeedback, TimeFeedback};
use libafl::generators::{RandBytesGenerator, RandPrintablesGenerator};
use libafl::inputs::{BytesInput, HasMutatorBytes};
use libafl::monitors::SimpleMonitor;
use libafl::mutators::havoc_mutations;
use libafl::observers::{HitcountsMapObserver, StdMapObserver};
use libafl::prelude::{MaxMapFeedback, StdScheduledMutator, TimeObserver};
use libafl::schedulers::{IndexesLenTimeMinimizerScheduler, QueueScheduler, RandScheduler};
use libafl::stages::StdMutationalStage;
use libafl::state::StdState;
use libafl::{feedback_and_fast, feedback_or, Fuzzer, StdFuzzer};
use libafl_bolts::{AsSliceMut, current_nanos};
use libafl_bolts::prelude::{ShMem, ShMemProvider, StdShMemProvider};
use libafl_bolts::rands::StdRand;
use libafl_bolts::tuples::tuple_list;

mod tutorial;

fn main() {
	//tutorial::main();
	execute();
}

fn execute() {
	let mut monitor = SimpleMonitor::new(|s| println!("{s}"));

	let mut harness = |input : &BytesInput| {
		crash_test(input.bytes());
		ExitKind::Ok
	};

	let input_corpus = InMemoryCorpus::<BytesInput>::new();
	let output_corpus = OnDiskCorpus::new("./crashes").unwrap();

	let time_observer = TimeObserver::new("time");

	const MAP_SIZE: usize = 65536;
	let mut shmem = StdShMemProvider::new().unwrap().new_shmem(MAP_SIZE).unwrap();

	shmem.write_to_env("__AFL_SHM_ID").expect("couldn't write shared memory ID");

	let shmem_map = shmem.as_slice_mut();

	let edges_observer = unsafe {
		HitcountsMapObserver::new(StdMapObserver::new("shared_mem", shmem_map))
	};

	let mut feedback = feedback_or!(
		MaxMapFeedback::new(&edges_observer),
		TimeFeedback::new(&time_observer)
	);

	let mut objective = feedback_and_fast!(
		CrashFeedback::new(),
	);

	let mut state = StdState::new(
		StdRand::with_seed(current_nanos()),
		input_corpus,
		output_corpus,
		&mut feedback,
		&mut objective
	).unwrap();

	let mut manager = SimpleEventManager::new(monitor);

	let scheduler = QueueScheduler::new();

	let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);
	
	let mut executor = InProcessExecutor::new(
		&mut harness,
		tuple_list!(time_observer),
		&mut fuzzer,
		&mut state,
		&mut manager
	).unwrap();

	let mut generator = RandBytesGenerator::new(128);
	
	state.generate_initial_inputs(&mut fuzzer, &mut executor, &mut generator, &mut manager, 8).unwrap(); // TODO: crashes here if it has to execute more than once
	
	let mutator = StdScheduledMutator::new(havoc_mutations());
	let mut stages = tuple_list!(StdMutationalStage::new(mutator));
	
	fuzzer.fuzz_loop(&mut stages, &mut executor, &mut state, &mut manager).unwrap();
}

fn crash_test(input : &[u8]) {
	println!("Input: {:?}", input);
	if input.len() > 20 {
		panic!("Crash!");
	}
}