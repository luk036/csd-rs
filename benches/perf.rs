use criterion::profiler::Profiler;
use pprof::ProfilerGuard;
use std::fs::File;
use std::os::raw::c_int;
use std::path::Path;

pub struct FlamegraphProfiler<'a> {
    frequency: c_int,
    active_profiler: Option<ProfilerGuard<'a>>,
}

impl<'a> FlamegraphProfiler<'a> {
    pub fn new(frequency: c_int) -> Self {
        FlamegraphProfiler {
            frequency,
            active_profiler: None,
        }
    }
}

impl<'a> Profiler for FlamegraphProfiler<'a> {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        self.active_profiler = Some(pprof::ProfilerGuard::new(self.frequency).unwrap());
    }

    fn stop_profiling(&mut self, _benchmark_id: &str, benchmark_dir: &Path) {
        let report = self
            .active_profiler
            .take()
            .unwrap()
            .report()
            .build()
            .unwrap();
        let file = File::create(benchmark_dir.join("flamegraph.svg")).unwrap();
        report.flamegraph(file).unwrap();
    }
}
