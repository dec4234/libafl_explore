extern crate libafl;
extern crate libafl_bolts;

use log::debug;
use simple_logger::SimpleLogger;
use libafl::corpus::{InMemoryCorpus, OnDiskCorpus};
use libafl::inputs::{BytesInput, HasTargetBytes};
use libafl_bolts::AsSlice;
use libafl::prelude::ExitKind;
use libafl::state::StdState;
use libafl_bolts::rands::StdRand;
use libafl_bolts::current_nanos;
use std::path::PathBuf;
use libafl::events::SimpleEventManager;
use libafl::executors::InProcessExecutor;
use libafl::generators::RandPrintablesGenerator;
use libafl::monitors::SimpleMonitor;
use libafl::observers::StdMapObserver;
use libafl::schedulers::QueueScheduler;
use libafl::StdFuzzer;
use libafl_bolts::tuples::tuple_list;

static mut SIGNALS: [u8; 16] = [0; 16];

fn signals_get(idx: usize) {
	unsafe {
		SIGNALS[idx];
	}
}

fn main() {
	SimpleLogger::new().init().unwrap();

	debug!("Starting");

	let observer = unsafe {
		StdMapObserver::new("signals", &mut SIGNALS)
	};

	let mut harness = |input: &BytesInput| {
		let target = input.target_bytes();
		let buf = target.as_slice();

		if buf.len() > 0 && buf[0] == 'a' as u8 {
			if buf.len() > 1 && buf[1] == 'b' as u8 {
				if buf.len() > 2 && buf[2] == 'c' as u8 {
					panic!("!!!");
				}
			}
		}

		ExitKind::Ok
	};

	let mut state = StdState::new(
		StdRand::with_seed(current_nanos()),
		InMemoryCorpus::new(),
		OnDiskCorpus::new(PathBuf::from("./crashes")).unwrap(),
		&mut (),
		&mut ()
	).unwrap();

	let mon = SimpleMonitor::new(|s| println!("{s}"));
	let mut mgr = SimpleEventManager::new(mon);

	let scheduler = QueueScheduler::new();
	let mut fuzzer = StdFuzzer::new(scheduler, (), ());

	let mut executor = InProcessExecutor::new(
		&mut harness,
		tuple_list!(observer),
		&mut fuzzer,
		&mut state,
		&mut mgr
	).expect("Could not create executor");

	let mut generator = RandPrintablesGenerator::new(32);

	state.generate_initial_inputs(&mut fuzzer, &mut executor, &mut generator, &mut mgr, 8).expect("Failed to generate corpus");
}
