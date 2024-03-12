use std::{collections::BTreeMap, sync::Mutex};

use instant::{Duration, Instant};
use kono::tabstops::Lines;
use once_cell::sync::Lazy;

static METRICS_COUNTER: Lazy<Mutex<MetricsCounter>> = Lazy::new(Default::default);

#[inline]
pub fn record_start_of_phase(phase: Phase) {
    METRICS_COUNTER.lock().unwrap().start_phase(phase);
}

#[inline]
pub fn print_metrics_to_stdout() {
    println!(
        "{}",
        Lines::new(METRICS_COUNTER.lock().unwrap().to_string())
    );
}

#[inline]
pub fn reset_metrics() {
    METRICS_COUNTER.lock().unwrap().reset()
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Phase {
    TopLevel(&'static str),
    SecondLevel(&'static str),
}

impl Phase {
    fn string(&self) -> &str {
        match self {
            Phase::TopLevel(name) => name,
            Phase::SecondLevel(name) => name,
        }
    }
}

struct OngoingPhase {
    phase: Phase,
    start_time: Instant,
}

struct PhaseStats {
    count: u64,
    total_time: Duration,
    max_time: Duration,
    min_time: Duration,
}

impl Default for PhaseStats {
    fn default() -> Self {
        Self {
            count: 0,
            total_time: Duration::ZERO,
            max_time: Duration::ZERO,
            min_time: Duration::MAX,
        }
    }
}

impl PhaseStats {
    fn increment(&mut self, phase_time: Duration) {
        self.count += 1;
        self.total_time += phase_time;
        self.max_time = self.max_time.max(phase_time);
        self.min_time = self.min_time.min(phase_time);
    }
}

const ORPHAN_PHASE: Phase = Phase::TopLevel("orphan phase");

#[derive(Default)]
struct MetricsCounter {
    current_top_phase: Option<OngoingPhase>,
    current_second_phase: Option<OngoingPhase>,
    top_phase_times: BTreeMap<Phase, PhaseStats>,
    second_phase_time: BTreeMap<Phase, BTreeMap<Phase, PhaseStats>>,
}

impl MetricsCounter {
    #[inline]
    fn start_phase(&mut self, phase: Phase) {
        let current_time = Instant::now();

        match phase {
            Phase::TopLevel(_) => {
                if let Some(ongoing_phase) = &self.current_top_phase {
                    let phase_time = current_time - ongoing_phase.start_time;
                    let stats = self
                        .top_phase_times
                        .entry(ongoing_phase.phase.clone())
                        .or_default();
                    stats.increment(phase_time);
                }
                // second level
                if let Some(ongoing_second_phase) = &self.current_second_phase {
                    let top_phase = match &self.current_top_phase {
                        Some(phase) => phase.phase.clone(),
                        None => ORPHAN_PHASE,
                    };
                    let phase_time = current_time - ongoing_second_phase.start_time;
                    let stats = self
                        .second_phase_time
                        .entry(top_phase.clone())
                        .or_default()
                        .entry(ongoing_second_phase.phase.clone())
                        .or_default();
                    stats.increment(phase_time);
                }
                self.current_top_phase.replace(OngoingPhase {
                    phase: phase.clone(),
                    start_time: current_time,
                });
                self.current_second_phase = None;
            }
            Phase::SecondLevel(_) => {
                let top_phase = match &self.current_top_phase {
                    Some(phase) => phase.phase.clone(),
                    None => ORPHAN_PHASE,
                };
                if let Some(ongoing_phase) = &self.current_second_phase {
                    let phase_time = current_time - ongoing_phase.start_time;
                    let phase_times = self.second_phase_time.entry(top_phase).or_default();
                    let stats = phase_times.entry(ongoing_phase.phase.clone()).or_default();
                    stats.increment(phase_time);
                }
                self.current_second_phase.replace(OngoingPhase {
                    phase,
                    start_time: current_time,
                });
            }
        }
    }

    fn reset(&mut self) {
        self.current_top_phase = None;
        self.current_second_phase = None;
        self.top_phase_times = BTreeMap::new();
        self.second_phase_time = BTreeMap::new();
    }

    fn top_phase_string(&self, top_phase: Phase) -> String {
        let mut result = String::new();
        if let Some(top_stats) = self.top_phase_times.get(&top_phase) {
            result.push_str(&format!(
                "{}\tcount:{}\ttotal:{}ms\tavg:{}ms\tmax:{}ms\tmin:{}ms\n",
                top_phase.string(),
                top_stats.count,
                top_stats.total_time.as_millis(),
                top_stats.total_time.as_millis() / top_stats.count as u128,
                top_stats.max_time.as_millis(),
                top_stats.min_time.as_millis(),
            ));
            if let Some(second_time) = self.second_phase_time.get(&top_phase) {
                for (second_phase, second_stats) in second_time {
                    result.push_str(&format!(
                        "  {}\tcount:{}\ttotal:{}ms\tavg:{}ms\tmax:{}ms\tmin:{}ms\n",
                        second_phase.string(),
                        second_stats.count,
                        second_stats.total_time.as_millis(),
                        second_stats.total_time.as_millis() / second_stats.count as u128,
                        second_stats.max_time.as_millis(),
                        second_stats.min_time.as_millis(),
                    ));
                }
            }
        }
        result
    }
}

impl ToString for MetricsCounter {
    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.top_phase_string(ORPHAN_PHASE));
        self.top_phase_times.iter().for_each(|(top_phase, _)| {
            result.push_str(&self.top_phase_string(top_phase.clone()));
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;

    #[test]
    fn basic() {
        let mut counter = MetricsCounter::default();
        counter.start_phase(Phase::TopLevel("hello"));
        sleep(Duration::from_millis(10));
        counter.start_phase(Phase::TopLevel("world"));
        sleep(Duration::from_millis(20));
        counter.start_phase(Phase::SecondLevel("world"));
        sleep(Duration::from_millis(30));
        counter.start_phase(Phase::TopLevel("arrow"));
        sleep(Duration::from_millis(40));
        counter.start_phase(Phase::TopLevel("world"));
        sleep(Duration::from_millis(50));
        counter.start_phase(Phase::TopLevel("finish!"));
        println!("{}", counter.to_string())
    }

    #[test]
    fn has_orphan() {
        let mut counter = MetricsCounter::default();
        counter.start_phase(Phase::SecondLevel("world"));
        sleep(Duration::from_millis(10));
        counter.start_phase(Phase::TopLevel("hello"));
        sleep(Duration::from_millis(20));
        counter.start_phase(Phase::TopLevel("arrow"));
        counter.start_phase(Phase::SecondLevel("world"));
        sleep(Duration::from_millis(30));
        counter.start_phase(Phase::TopLevel("world"));
        sleep(Duration::from_millis(40));
        counter.start_phase(Phase::TopLevel("finish!"));
        println!("{}", counter.to_string())
    }
}
