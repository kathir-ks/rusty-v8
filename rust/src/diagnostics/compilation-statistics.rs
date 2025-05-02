use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Default, Clone)]
pub struct BasicStats {
    pub delta_: Duration,
    pub total_allocated_bytes_: usize,
    pub max_allocated_bytes_: usize,
    pub absolute_max_allocated_bytes_: usize,
    pub function_name_: String,
    pub input_graph_size_: usize,
    pub output_graph_size_: usize,
    pub count_: usize,
}

impl BasicStats {
    pub fn accumulate(&mut self, stats: &BasicStats) {
        self.delta_ += stats.delta_;
        self.total_allocated_bytes_ += stats.total_allocated_bytes_;
        if stats.absolute_max_allocated_bytes_ > self.absolute_max_allocated_bytes_ {
            self.absolute_max_allocated_bytes_ = stats.absolute_max_allocated_bytes_;
            self.max_allocated_bytes_ = stats.max_allocated_bytes_;
            self.function_name_ = stats.function_name_.clone();
        }
        self.input_graph_size_ += stats.input_graph_size_;
        self.output_graph_size_ += stats.output_graph_size_;
    }

    pub fn as_json(&self) -> String {
        format!(
            r#"{{"function_name":"{}","total_allocated_bytes":{},"max_allocated_bytes":{},"absolute_max_allocated_bytes":{}}}"#,
            self.function_name_.replace("\"", "\\\""),
            self.total_allocated_bytes_,
            self.max_allocated_bytes_,
            self.absolute_max_allocated_bytes_
        )
    }
}

#[derive(Clone)]
struct PhaseStats {
    insert_order_: usize,
    phase_kind_name_: String,
    basic_stats_: BasicStats,
}

impl PhaseStats {
    pub fn new(insert_order: usize, phase_kind_name: &str) -> Self {
        PhaseStats {
            insert_order_: insert_order,
            phase_kind_name_: phase_kind_name.to_string(),
            basic_stats_: BasicStats::default(),
        }
    }

    pub fn accumulate(&mut self, stats: &BasicStats) {
        self.basic_stats_.accumulate(stats);
    }
}

impl std::ops::Deref for PhaseStats {
    type Target = BasicStats;
    fn deref(&self) -> &Self::Target {
        &self.basic_stats_
    }
}

impl std::ops::DerefMut for PhaseStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.basic_stats_
    }
}

#[derive(Clone)]
struct PhaseKindStats {
    insert_order_: usize,
    basic_stats_: BasicStats,
}

impl PhaseKindStats {
    pub fn new(insert_order: usize) -> Self {
        PhaseKindStats {
            insert_order_: insert_order,
            basic_stats_: BasicStats::default(),
        }
    }

    pub fn accumulate(&mut self, stats: &BasicStats) {
        self.basic_stats_.accumulate(stats);
    }
}

impl std::ops::Deref for PhaseKindStats {
    type Target = BasicStats;
    fn deref(&self) -> &Self::Target {
        &self.basic_stats_
    }
}

impl std::ops::DerefMut for PhaseKindStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.basic_stats_
    }
}

pub struct CompilationStatistics {
    access_mutex_: Arc<Mutex<CompilationStatisticsInternal>>,
}

struct CompilationStatisticsInternal {
    phase_map_: HashMap<String, PhaseStats>,
    phase_kind_map_: HashMap<String, PhaseKindStats>,
    total_stats_: BasicStats,
}

impl CompilationStatistics {
    pub fn new() -> Self {
        CompilationStatistics {
            access_mutex_: Arc::new(Mutex::new(CompilationStatisticsInternal {
                phase_map_: HashMap::new(),
                phase_kind_map_: HashMap::new(),
                total_stats_: BasicStats::default(),
            })),
        }
    }

    pub fn record_phase_stats(&self, phase_kind_name: &str, phase_name: &str, stats: &BasicStats) {
        let mut guard = self.access_mutex_.lock().unwrap();
        let phase_name_str = phase_name.to_string();
        let phase_map = &mut guard.phase_map_;
        let phase_kind_map = &guard.phase_kind_map_;

        let it = phase_map.get(&phase_name_str);
        if it.is_none() {
            let phase_stats = PhaseStats::new(phase_map.len(), phase_kind_name);
            phase_map.insert(phase_name_str.clone(), phase_stats);
        }

        phase_map.get_mut(&phase_name_str).unwrap().accumulate(stats);
    }

    pub fn record_phase_kind_stats(&self, phase_kind_name: &str, stats: &BasicStats) {
        let mut guard = self.access_mutex_.lock().unwrap();
        let phase_kind_name_str = phase_kind_name.to_string();
        let phase_kind_map = &mut guard.phase_kind_map_;

        let it = phase_kind_map.get(&phase_kind_name_str);
        if it.is_none() {
            let phase_kind_stats = PhaseKindStats::new(phase_kind_map.len());
            phase_kind_map.insert(phase_kind_name_str.clone(), phase_kind_stats);
        }
        phase_kind_map.get_mut(&phase_kind_name_str).unwrap().accumulate(stats);
    }

    pub fn record_total_stats(&self, stats: &BasicStats) {
        let mut guard = self.access_mutex_.lock().unwrap();
        guard.total_stats_.accumulate(stats);
        guard.total_stats_.count_ += 1;
    }
}

pub struct AsPrintableStatistics<'a> {
    pub s: &'a CompilationStatistics,
    pub compiler: &'a str,
    pub machine_output: bool,
}

impl<'a> fmt::Display for AsPrintableStatistics<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = self.s.access_mutex_.lock().unwrap();
        let s = &*guard;

        let mut sorted_phase_kinds: Vec<_> = s.phase_kind_map_.iter().collect();
        sorted_phase_kinds.sort_by_key(|(_, v)| v.insert_order_);

        let mut sorted_phases: Vec<_> = s.phase_map_.iter().collect();
        sorted_phases.sort_by_key(|(_, v)| v.insert_order_);

        if !self.machine_output {
            write_header(f, self.compiler)?;
        }

        for (phase_kind_name, phase_kind_stats) in &sorted_phase_kinds {
            if !self.machine_output {
                for (phase_name, phase_stats) in &sorted_phases {
                    if phase_stats.phase_kind_name_ != *phase_kind_name {
                        continue;
                    }
                    write_line(
                        f,
                        self.machine_output,
                        phase_name.as_str(),
                        self.compiler,
                        &phase_stats.basic_stats_,
                        &s.total_stats_,
                    )?;
                }
                write_phase_kind_break(f)?;
            }
            write_line(
                f,
                self.machine_output,
                phase_kind_name.as_str(),
                self.compiler,
                &phase_kind_stats.basic_stats_,
                &s.total_stats_,
            )?;
            writeln!(f)?;
        }

        if !self.machine_output {
            write_full_line(f)?;
        }
        write_line(
            f,
            self.machine_output,
            "totals",
            self.compiler,
            &s.total_stats_,
            &s.total_stats_,
        )?;

        if self.machine_output {
            writeln!(f)?;
            write!(f, "\"{}_totals_count\"={}", self.compiler, s.total_stats_.count_)?;
        }

        Ok(())
    }
}

fn write_line(
    f: &mut fmt::Formatter<'_>,
    machine_format: bool,
    name: &str,
    compiler: &str,
    stats: &BasicStats,
    total_stats: &BasicStats,
) -> fmt::Result {
    let ms = stats.delta_.as_secs_f64() * 1000.0;
    let percent = if total_stats.delta_.as_nanos() > 0 {
        (stats.delta_.as_nanos() as f64 / total_stats.delta_.as_nanos() as f64) * 100.0
    } else {
        0.0
    };

    let size_percent = if total_stats.total_allocated_bytes_ > 0 {
        (stats.total_allocated_bytes_ as f64 * 100.0) / (total_stats.total_allocated_bytes_ as f64)
    } else {
        0.0
    };

    let growth = if stats.input_graph_size_ > 0 {
        stats.output_graph_size_ as f64 / stats.input_graph_size_ as f64
    } else {
        0.0
    };
    let mops_per_s = (stats.output_graph_size_ as f64 / 1000000.0) / (ms / 1000.0);

    if machine_format {
        writeln!(
            f,
            "\"{}_{}_time\"={:.3}\n\"{}_{}_space\"={}",
            compiler, name, ms, compiler, name, stats.total_allocated_bytes_
        )?;
    } else {
        if stats.output_graph_size_ != 0 {
            write!(f, "{:34} {:10.3} ({:4.1}%)  {:10} ({:4.1}%) {:10} {:10}   {:5.3} {:6.2}",
                name, ms, percent, stats.total_allocated_bytes_, size_percent,
                stats.max_allocated_bytes_, stats.absolute_max_allocated_bytes_,
                growth, mops_per_s)?;
        } else {
            write!(f, "{:34} {:10.3} ({:4.1}%)  {:10} ({:4.1}%) {:10} {:10}               ",
                name, ms, percent, stats.total_allocated_bytes_, size_percent,
                stats.max_allocated_bytes_, stats.absolute_max_allocated_bytes_)?;
        }

        if !stats.function_name_.is_empty() {
            write!(f, "  {}", stats.function_name_)?;
        }
        writeln!(f)?;
    }

    Ok(())
}

fn write_full_line(f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(
        f,
        "-----------------------------------------------------------------------------------------------------------"
    )
}

fn write_header(f: &mut fmt::Formatter<'_>, compiler: &str) -> fmt::Result {
    write_full_line(f)?;
    writeln!(
        f,
        "{:24} phase            Time (ms)                      Space (bytes)            Growth MOps/s Function",
        compiler
    )?;
    writeln!(
        f,
        "                                                                       Total         Max.     Abs. max."
    )?;
    write_full_line(f)?;
    Ok(())
}

fn write_phase_kind_break(f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(
        f,
        "                                   ---------------------------------------------------------------------------"
    )
}