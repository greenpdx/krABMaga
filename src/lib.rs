#![doc(html_logo_url = "https://krABMaga.github.io/images/krabmaga.png")]

//!
//![krABMaga](https://github.com/krABMaga/krABMaga) is a discrete events simulation engine for developing ABM simulation
//!written in the [Rust language](https://www.rust-lang.org/).
//!
//![krABMaga](https://github.com/krABMaga/krABMaga) is designed to be a ready-to-use tool for the ABM community and for this
//!reason the architectural concepts of the well-adopted [MASON library](https://cs.gmu.edu/~eclab/projects/mason/) were
//!re-engineered to exploit the Rust peculiarities and programming model, in particular by keeping the visualization and the
//!simulation subsystems fully separated.
//!
//!---
//!
//!## Table of contents
//!<!-- no toc -->
//!- [Table of contents](#table-of-contents)
//!- [Dependencies](#dependencies)
//!- [How to run your first example simulaton](#how-to-run-your-first-example-simulaton)
//!- [How to write your first model](#how-to-write-your-first-model)
//!- [Available features](#available-features)
//!- [Macros for playing with Simulation Terminal](#macros-for-playing-with-simulation-terminal)
//!- [How to contribute](#how-to-contribute)
//!- [Architecture](#architecture)
//!  - [Agents](#agents)
//!  - [Simulation state](#simulation-state)
//!  - [Schedule](#schedule)
//!  - [Data structures](#data-structures)
//!
//!---
//!
//!# Dependencies
//!
//!The visualization framework requires certain dependencies to run the simulation properly.
//!- 💻 Windows: [VS2019 build tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
//!- 🍎 MacOS: No dependencies needed.
//!- 🐧 Linux: A few dependencies are needed. Check [here](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) for a list based on your distribution.
//!---
//!# How to run your first example simulaton
//!First of all, install latest version of [Rust](https://www.rust-lang.org/tools/install). Follow steps to setup Rust toolchain (*cargo*, *rustc* and *rustup*).
//!
//!Now, you can download/clone all available krABMaga examples from our github repository called [examples](https://github.com/krABMaga/examples).
//!
//!To run a simulation, go to root directory of a model, for example `/path/to/examples/flockers`. With command `ls`, you should be able to see a typcal krABMaga simulation struct:
//!- `src`: main folder with code. It contains `main.rs` file and two directories for model and visulization components.
//!- `Cargo.toml`: Configuration file for Rust project, with dependencies and features.
//!- `assets`: an images folder. It contains all the icons that can be used for visualization.
//!- `Makefile.toml`: another configuration file, necessary to a correct execution of visualization.
//!
//!Inside the root directory of model that you choose, you can run a models with or without visualization. 
//!
//!To simply run your simulation, with no visualization:
//!```sh
//!cargo run --release
//!```
//!Running in this way, you can see our `Simulation Terminal` (better known as `Simulation Monitor`)) based on [tui-rs](https://github.com/fdehau/tui-rs), a rust library that provides components to create terminal with an interface. As a modelist, you can use krABMaga macros to create several plots, print logs and add a model description (shown using a popup)
//!
//!
//!<style>
//!* {
//!  box-sizing: border-box;
//!}
//!.column {
//!  height: auto;
//!  min-height: 100%;
//!  /* width: 45.0%; */
//!  min-width: 200px;
//!  padding: 5px;
//!  display:inline-block;
//!  text-align: center;
//!  vertical-align:middle; 
//!}
//!
//!  @media screen and (max-width: 400px) {
//!    .column{
//!        width: 45%;
//!    }
//!  }
//!
//!
//!/* Clearfix (clear floats) */
//!.row::after {  
//!  content: "";
//!  clear: both;
//!  display: table;
//!}
//!
//!.row{
//!    text-align: center;
//!
//!}
//!</style>
//!
//!
//!<div class="row">
//!  <div class="column" >
//!    <img style="margin-left: auto;" src="https://krabmaga.github.io/images/tui-wsg.gif"/>
//!  </div>
//!  <div class="column">
//!    <img style="margin-left: auto;" src="https://krabmaga.github.io/images/ant.gif"/>
//!  </div>
//!</div>
//!
//!
//!Based on [Bevy game engine](https://bevyengine.org/), it's possible to run simulation with visualization. It's also available a menu to start and stop simulations and a slider to set simulation speed.
//!To run a model with visualization enabled, you have to start the simulation with the command:
//!```sh
//!cargo run --release --features  visualization
//!
//!# Alternative command. Requires 'cargo make' installed
//!cargo make run --release 
//!```
//!
//!In addition to the classical visualization, you can run your krABMaga simulation inside your browser using (*Web Assembly*)[https://webassembly.org]. 
//!This is possible with the command:
//!```sh
//!# Requires 'cargo make' installed
//!cargo make serve --release 
//!```
//!
//!
//!---
//!# How to write your first model
//!
//!If you don't start from our [Template](https://github.com/krABMaga/examples/tree/main/template), add this to your `Cargo.toml`:
//!```toml
//![dependencies]
//!krABMaga = { git="https://github.com/krABMaga/krABMaga.git" }
//!
//![features]
//!visualization = ["krABMaga/visualization"]
//!visualization_wasm = ["krABMaga/visualization_wasm"]
//!```
//!
//!We **strongly** recommend to use [Template](https://github.com/krABMaga/examples/tree/main/template) or any other example as base of a new project, especially if you want to provide any visualization.
//!
//!Each krABMaga model needs structs that implements our *Traits*, one for *State* and the other for *Agent*. In the *State* struct you have to put *Agent* field(s), because it represents the ecosystem of a simulation. More details for each krABMaga componenet are in the [Architecture](#architecture) section.
//!
//!The simplest part is `main.rs`, because is similar for each example.
//!You can define two *main* functions using **cfg** directive, that can remove code based on which features are (not) enabled.  
//!Without visualization, you have only to use *simulate!* to run simulation, passing a state, step number and how may time repeat your simulation. 
//!With visualization, you have to set graphical settings (like dimension or background) and call *start* method.
//!```rs
//!// Main used when only the simulation should run, without any visualization.
//!#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
//!fn main() {
//!  let dim = (200., 200.);
//!  let state = Flocker::new(dim, num_agents);
//!  let step = 10;
//!  let reps = 1;
//!  let num_agents = 100;  
//!  let _ = simulate!(state, step, reps);
//!}
//!
//!// Main used when a visualization feature is applied.
//!#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
//!fn main() {
//!  let dim = (200., 200.);
//!  let num_agents = 100;
//!  let state = Flocker::new(dim, num_agents);
//!  Visualization::default()
//!      .with_window_dimensions(1000., 700.)
//!      .with_simulation_dimensions(dim.0 as f32, dim.1 as f32)
//!      .with_background_color(Color::rgb(0., 0., 0.))
//!      .with_name("Flockers")
//!      .start::<VisState, Flocker>(VisState, state);
//!}
//!
//!```
//!---
//!
//!# Available features
//!
//!<style>
//!  table{
//!    word-wrap: break-word;
//!    table-layout: auto;
//!    width: 100%;
//!    
//!  }
//!</style>
//!
//!This library offers some features to make your simulation more interesting and to avoid to install many dependencies that are not needed for basic simulation.
//!```sh
//!cargo run --release --features <name_feature>
//!```
//!
//!<div  style="overflow-x:auto;">
//!
//!| Compilation Feature  | Description |  Experimental | Release Candidate  | Stable  |
//!|:------:|:-------:|:---:|:---:|:---:|
//!| **No Features** | Possibility to run model using `Simulation Terminal` and setup model-exploration experiments (Parameter Sweeping, Genetic and Random) in sequential/parallel mode. It's enough to create your base simulations. |   |   | 🦀 |
//!| **visualization**  | Based on `Bevy engine`, it makes possible to visualize your model elements, to understand better the behavior of your simulation. |   | 🦀 |   |
//!| **visualization-wasm** | Based on `Web Assembly`, give you the possibility to execute your visualized simulation inside your own browser. |   | 🦀 |   |
//!| **distributed-mpi** | Enable distributed model exploration using MPI. At each iteration, the amount of configurations are balanced among your nodes.  |   |  🦀 |   |
//!| **bayesian**  | Use ML Rust libraries to use/create function to use `Bayesian Optimization`.|   | 🦀  |   |
//!| **parallel**  | Speed-up a single simulation parallelizing agent scheduling during a step.| 🦀  |   |   |
//!
//!</div>
//!
//!---
//!# Macros for playing with Simulation Terminal
//!
//!`Simulation Terminal` is enabled by default using macro `simulate!`, so can be used passing a state, step number and how may time repeat your simulation..
//!That macro has a fourth optional parameter, a boolean. When `false` is passed, `Simulation Terminal` is disabled.
//!```rs
//!($s:expr, $step:expr, $reps:expr $(, $flag:expr)?) => {{
//!      // Macro code 
//!}}
//!```
//!
//!You can create tabs and plot your data using two macro:
//!- `addplot!` let you create a new plot that will be displayed in its own tab.
//!```rs
//!addplot!(String::from("Chart Name"), String::from("xxxx"), String::from("yyyyy"));
//!```
//!- `plot!` to add a point to a plot. Points can be added during simulation execution, for example inside `after_step` method.
//!  You have to pass plot name, series name, x value and y value. Coordinate values need to be `f64`.
//!```rs
//!plot!(String::from("Chart name"), String::from("s1"), x, y);
//!```
//!
//!On Terminal home page there is also a *log section*, you can plot log messages when some event needs to be noticed.
//!You can navigate among all logs using ↑↓ arrows.
//!To add a log use the macro `log!`, passing a `LogType` (an enum) and the log message.
//!```rs
//! log!(LogType::Info, String::from("Log Message"));
//!```
//!
//!Are available four type of Logs:
//!```rs
//!pub enum LogType {
//!    Info,
//!    Warning,
//!    Error,
//!    Critical,
//!}
//!```
//!
//!---
//!# How to contribute
//!
//!If you want to test, add or change something inside krABMaga engine, you can clone [main repo](https://github.com/krABMaga/krABMaga) locally, and change dependecy inside `Cargo.toml` of your examples:
//!
//!```toml
//![dependencies]
//!# krABMaga = { git="https://github.com/krABMaga/krABMaga.git" }
//!krABMaga = { path="path/to/krABMaga"}
//!```
//!
//!---
//!# Architecture
//!
//!## Agents
//!
//!The krABMaga framework defines a trait `Agent` that can be implemented on a struct to define `Agent` specific functionalities,
//!mainly the `step` method which specifies how the agent behaves for each simulation step, and the `get_id` method,
//!to uniquely identify an agent. There are also other methods, with default implementation, to improve agent control:
//!
//!- `is_stopped` notify the scheduler if a specific agent should be removed or not, based on some condition.
//!- `before_step` and `after_step` to implement some operations before/after a step.
//!
//!The krABMaga framework allow multi-agent implementations: you can define multiple 'Agent' that
//!implement the trait, and [Wolf, Sheep & Grass](https://krABMaga.github.io/wolfsheepgrass/) is the main example of this feature.
//!
//!---
//!## Simulation state
//!
//!The simulation state can be considered as the single source of truth of the simulation, where data resides and is updated.
//!Like `Agent`, krABMaga exposes a `State` trait to let the user mark a particular structure as a simulation state, along with
//!exposing an `update` method to define logic to execute once for each simulation step. The simulation state is the perfect
//!structure to put field definitions on (such as 2D continuous fields, grids and so on). An important effect of the state being
//!the single source of truth forces agents to update (and most importantly read) their own location by interacting with the
//!state, even though they can store their own location locally in the agent structure too. Although, to be sure one is interacting
//!with the latest computed data, it is considered a good practice to update both an agent own location field and its copy on the
//!state structure.
//!
//!---
//!## Schedule
//!
//!The simulation timeline is controlled by a Schedule structure that takes care of notifying all the scheduled agents, and the
//!simulation state that a step has been taken. For this reason, agents should be scheduled so that they can be notified when
//!a step has been taken.
//!The scheduler works as a priority queue, where the agents are sorted according to their scheduled time
//!and a priority value - an integer. The simulation time - a real value - starts from the scheduling time of the first agent.
//!The schedule structure exposed by the krABMaga framework provides two methods to do so:
//!- `schedule_once` to insert an agent in the schedule for a specific simulation step. The scheduling time and the
//!  priority are given as parameters. The priority is used to sort all agents within the same simulation time.
//!  
//!- `schedule_repeating` which acts like schedule once, with the difference that the agent will be scheduled for all
//!  subsequent simulation steps.
//!
//!The schedule provides the `step` method which allows executing one simulation step. In this way, the programmer can
//!easily design his/her simulation by looping for a certain number of step or for a given amount of CPU time.
//!
//!---
//!
//!## Data structures
//!
//!<!-- The krABMaga framework exposes a few data structures based on the `DBDashMap`, a customized version of the 
//![Rust HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) that implements a double
//!buffering technique to avoid indeterminism caused by the lack of knowledge of the agents' step execution order within a step.
//!The `DBDashMap` implements the interior mutability pattern, which allows the user to safely write in it without having an actual
//!mutable reference to the structure, because the reads are done on a different memory block than the writes. Only the `update`
//!method actually requires a mutable reference, to swap the read and the write buffers and commit the changes. -->
//!
//!The currently implemented structures are:
//!
//!- `Field2D`, a sparse matrix structure modelling agent interactions on a
//!  2D real space with coordinates represented by 2D f64 tuples (`Real2D`).
//!  
//!- `Grid2D`, a discrete field representing agents locations as 2D i64 tuples (`Int2D`). This structure keeps two copies of a DBDashMap in sync,
//!  one the inverse of the other, to allow constant time access both by key (agent) and by value (position). There are two kind of Grid based on density, `SparseGrid2D` and `DenseGrid2D`.
//!  
//!- `NumberGrid2D`, a simpler version of the `Grid2D` to use with simpler values. This is useful to represent simulation spaces
//!  covered by a simple entity that can be represented with a non-agent structure. This data structure can be used with any
//!  structure that can be cloned, most notably simple primitive values such as f64s. As the previous grid, there are two implementations: `SparseNumberGrid2D` and `DenseNumberGrid2D`.
//!  
//!- `Network` and `HNetwork` to connect any kind of nodes using `Edge`/`HEdge`. With `Network` you can define both directed and undirected graphs and connect a couple of nodes with an edge with label and/or weight. `HNetwork` is a generalization of a `Network` to represent hypergraph. In this case, `HEdge` is an `HashSet` of nodes.
//!  With this fields you can reproduce any kind of graph or network, such as for our example [`Virus on a Network`](/virusnetwork).
//!
//!---
//!
//!# Support conference paper
//!
//!If you find this code useful in your research, please consider citing:
//!
//!```
//!@ARTICLE{AntelmiASIASIM2019,
//!  author={Antelmi, A. and Cordasco, G. and D’Auria, M. and De Vinco, D. and Negro, A. and Spagnuolo, C.},
//!  title={On Evaluating Rust as a Programming Language for the Future of Massive Agent-Based Simulations},
//!  journal={Communications in Computer and Information Science},
//!  note={Conference of 19th Asia Simulation Conference, AsiaSim 2019 ; Conference Date: 30 October 2019 Through 1 November 2019;  Conference Code:233729},
//!  year={2019},
//!  volume={1094},
//!  pages={15-28},
//!  doi={10.1007/978-981-15-1078-6_2},
//!  issn={18650929},
//!  isbn={9789811510779},
//!}
//!
//!```
//!

/// Main module, with structs for Agents, Fields and Schedule
pub mod engine;

#[doc(hidden)]
/// Module for model exploration
pub mod explore;

#[doc(hidden)]
pub mod utils;

#[doc(hidden)]
pub use {
    ::lazy_static::*,
    core::fmt,
    csv::{Reader, Writer},
    hashbrown,
    indicatif::ProgressBar,
    rand, rand_pcg, rayon,
    rayon::prelude::*,
    std::collections::HashMap,
    std::error::Error,
    std::fs::File,
    std::fs::OpenOptions,
    std::io,
    std::io::prelude::*,
    std::io::Write,
    std::process::{Command, Stdio},
    std::sync::{Arc, Mutex},
    std::thread,
    std::time::Duration,
    std::time::Instant,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm",))]
pub mod visualization;

#[cfg(any(feature = "visualization", feature = "visualization_wasm",))]
pub use bevy;

#[doc(hidden)]
pub use rand::{
    distributions::{Distribution, Uniform},
    thread_rng, Rng,
};

#[doc(hidden)]
#[cfg(not(feature = "visualization_wasm"))]
pub use {
    crate::utils::monitoring::ui::UI,
    crossterm,
    crossterm::event::poll,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    systemstat::{saturating_sub_bytes, Platform, System},
    tui::{
        backend::{Backend, CrosstermBackend},
        Terminal,
    },
};

#[cfg(feature = "distributed_mpi")]
pub use {
    memoffset::{offset_of, span_of},
    mpi_fork_fnsp::datatype::DynBufferMut,
    mpi_fork_fnsp::datatype::PartitionMut,
    mpi_fork_fnsp::point_to_point as p2p,
    mpi_fork_fnsp::Count,
    mpi_fork_fnsp::{datatype::UserDatatype, traits::*, Address},
};

#[cfg(feature = "distributed_mpi")]
pub extern crate mpi_fork_fnsp;

#[doc(hidden)]
#[cfg(any(feature = "bayesian"))]
pub use {argmin, finitediff, friedrich, statrs};

#[doc(hidden)]
#[cfg(feature = "aws")]
pub use {
    aws_config,
    aws_sdk_lambda,
    aws_sdk_sqs,
    futures::executor::block_on,
    lambda_runtime,
    serde_json,
    serde_json::{json, Value},
    std::fs,
    std::io::BufReader,
    tokio,
    tokio::runtime::Runtime, // 0.3.5
};

/// Options of old_simulate! macro
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Info {
    Verbose,
    Normal,
}

///
/// 2 modes to generate the data
/// Exaustive: Brute force parameter exploration
/// Matched: explore every input with the same indexes
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExploreMode {
    Exaustive,
    Matched,
}

#[doc(hidden)]
#[derive(Clone)]
/// Struct to manage plots inside `Simulation Terminal`
pub struct PlotData {
    /// Plot name
    pub name: String,
    /// Data of a plot. Managed using `HashMap`
    pub series: HashMap<String, Vec<(f64, f64)>>,
    /// Min value of x axis
    pub min_x: f64,
    /// Max value of x axis
    pub max_x: f64,
    /// Min value of y axis
    pub min_y: f64,
    /// Max value of y axis
    pub max_y: f64,
    /// Label of x axis
    pub xlabel: String,
    /// Label of y axis
    pub ylabel: String,
}

#[doc(hidden)]
impl PlotData {
    /// Create new Plot
    pub fn new(name: String, xlabel: String, ylabel: String) -> PlotData {
        PlotData {
            name,
            series: HashMap::new(),
            min_x: f64::MAX,
            max_x: f64::MIN,
            min_y: f64::MAX,
            max_y: f64::MIN,
            xlabel,
            ylabel,
        }
    }
}

/// Available log types to use for `Simulation Terminal` log mechanism.
#[derive(Copy, Clone, Debug)]
pub enum LogType {
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for LogType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogType::Info => write!(f, "Info: "),
            LogType::Warning => write!(f, "Warning: "),
            LogType::Error => write!(f, "Error: "),
            LogType::Critical => write!(f, "Critical: "),
        }
    }
}

#[doc(hidden)]
pub struct Log {
    /// One of 4 availbale types
    pub ltype: LogType,
    /// Log message to display
    pub body: String,
}

lazy_static! {

    /// static HashMap to manage plots of the whole simulation. Used to create tabs and plot inside `UI` module.
    #[doc(hidden)]
    pub static ref DATA: Mutex<HashMap<String, PlotData>> = Mutex::new(HashMap::new());
    /// static Vec to store all Logs and make it availables inside terminal.
    #[doc(hidden)]
    pub static ref LOGS: Mutex<Vec<Log>> = Mutex::new(Vec::new());
    /// static String to save Model description to show as a popup. Press 's' on `Simulation Terminal.
    #[doc(hidden)]
    pub static ref DESCR: Mutex<String> = Mutex::new(String::new());
}

#[doc(hidden)]
/// struct to store machine system info during the simulation.
pub struct Monitoring {
    /// Percentage of memory used
    pub mem_used: Vec<f64>,
    /// Percentage of cpu used
    pub cpu_used: Vec<f64>,
}

#[doc(hidden)]
impl Monitoring {
    pub fn new() -> Self {
        Monitoring {
            mem_used: Vec::new(),
            cpu_used: Vec::new(),
        }
    }
}

lazy_static! {
    /// static object to collect data of monitoring
    #[doc(hidden)]
    pub static ref MONITOR: Arc<Mutex<Monitoring>> = Arc::new(Mutex::new(Monitoring::new()));
}

#[doc(hidden)]
pub use std::sync::mpsc::{self, TryRecvError};

/// Run simulation directly using this macro. By default, `Simulation Terminal` is used
///
/// s: istance of state of simulation
///
/// step: simulation step number
///
/// reps: # of repetitions
///
/// flag boolean: to abilitate TUI (optional, default true)
#[macro_export]
macro_rules! simulate {
    ($s:expr, $step:expr, $reps:expr $(, $flag:expr)?) => {{

        let mut flag = true;
        $(
            flag = $flag;
        )?

        if flag {
            let tick_rate = Duration::from_millis(250);

            let _ = enable_raw_mode();
            let mut stdout = io::stdout();
            let _ = execute!(stdout, EnterAlternateScreen, EnableMouseCapture);

            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend).unwrap();

            let mut last_tick = Instant::now();
            let mut ui = UI::new($step, $reps);

            let mut s = $s;
            let mut state = s.as_state_mut();
            let n_step: u64 = $step;

            let mut monitor = Arc::clone(&MONITOR);
            let (tx, rx) = mpsc::channel();

            thread::spawn(move ||
            loop {
                // System info - Monitoring CPU and Memory used

                let sys = System::new();

                let mem_used = match sys.memory() {
                    Ok(mem) => {
                        (saturating_sub_bytes(mem.total, mem.free).as_u64() as f64 / mem.total.as_u64() as f64)  * 100.
                    },
                    Err(x) =>{
                        log!(LogType::Critical, format!("Error on load mem used"));
                        0.0_f64
                    }
                };

                let cpu_used = match sys.cpu_load_aggregate() {
                    Ok(cpu)=> {
                        thread::sleep(Duration::from_millis(1000));
                        let cpu = cpu.done().unwrap();
                        cpu.user as f64 * 100.0
                    },
                    Err(x) => {
                        log!(LogType::Critical, format!("Error on load cpu used"));
                        0.0_f64
                    }
                };

                {
                    let mut monitor = monitor.lock().unwrap();

                    if monitor.mem_used.len()>100 {
                        monitor.mem_used.remove(0);
                        monitor.cpu_used.remove(0);
                    }

                    monitor.mem_used.push(mem_used);
                    monitor.cpu_used.push(cpu_used);

                }


                match rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            });

            for r in 0..$reps {

                //clean data structure for UI
                DATA.lock().unwrap().clear();
                terminal.clear();

                let start = std::time::Instant::now();
                let mut schedule: Schedule = Schedule::new();
                state.init(&mut schedule);
                //simulation loop
                for i in 0..n_step {

                    terminal.draw(|f| ui.draw(f));
                    schedule.step(state);

                    let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
                    //check for keyboard input
                    if crossterm::event::poll(timeout).unwrap() {
                        //?
                        if let Event::Key(key) = event::read().unwrap(){
                            //?
                            match key.code {
                                KeyCode::Char(c) => ui.on_key(c),
                                KeyCode::Left => ui.on_left(),
                                KeyCode::Up => ui.on_up(),
                                KeyCode::Right => ui.on_right(),
                                KeyCode::Down => ui.on_down(),
                                _ => {
                                    log!(LogType::Critical, format!("Invalid key pressed!"));
                                }
                            }
                        }
                    }
                    if ui.should_quit {
                        disable_raw_mode();
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        );
                        terminal.show_cursor();
                        break;
                    }
                    if state.end_condition(&mut schedule) {
                        disable_raw_mode();
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        );
                        terminal.show_cursor();
                        break;
                    }
                    ui.on_tick(i, (i + 1) as f64 / n_step as f64);
                } //end simulation loop
                let run_duration = start.elapsed();
                ui.on_rep(
                    r,
                    ((schedule.step as f32 / (run_duration.as_nanos() as f32 * 1e-9)) as u64),
                );
                terminal.draw(|f| ui.draw(f));

                if last_tick.elapsed() >= tick_rate {
                    last_tick = Instant::now();
                }

                if ui.should_quit {
                    disable_raw_mode();
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    );
                    terminal.show_cursor();
                    break;
                }
            } //end of repetitions

            let _ = tx.send(());

            loop {
                terminal.draw(|f| ui.draw(f));

                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if crossterm::event::poll(timeout).unwrap() {
                    //?
                    if let Event::Key(key) = event::read().unwrap() {
                        //?
                        match key.code {
                            KeyCode::Char(c) => ui.on_key(c),
                            KeyCode::Left => ui.on_left(),
                            KeyCode::Up => ui.on_up(),
                            KeyCode::Right => ui.on_right(),
                            KeyCode::Down => ui.on_down(),
                            _ => {
                                log!(LogType::Critical, format!("Invalid key pressed!"));
                            }
                        }
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    last_tick = Instant::now();
                }
                if ui.should_quit {
                    disable_raw_mode();
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    );
                    terminal.show_cursor();
                    break;
                }
            }
        } else {

            let mut s = $s;
            let mut state = s.as_state_mut();
            let n_step: u64 = $step;
            //basic simulation without UI
            for r in 0..$reps {
                let mut schedule: Schedule = Schedule::new();
                state.init(&mut schedule);
                //simulation loop
                for i in 0..n_step {
                    schedule.step(state);
                    if state.end_condition(&mut schedule) {
                        break;
                    }
                } //end simulation loop
            } //end of repetitions
            println!("Simulation finished!");
        } //enf if/else flag

    }}; // end pattern macro
} //end macro

/// Add a description to your simulation. You can show a popup with this message.
#[macro_export]
macro_rules! description {
    ($description:expr) => {{
        *DESCR.lock().unwrap() = $description.clone();
    }};
}

///Add a point to a series of an existing plot
#[macro_export]
macro_rules! plot {
    ($name:expr, $serie:expr, $x:expr, $y:expr) => {{
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&$name) {
            let mut pdata = data.get_mut(&$name).unwrap();
            if !pdata.series.contains_key(&$serie) {
                pdata.series.insert($serie.clone(), Vec::new());
            }
            let serie = pdata.series.get_mut(&$serie).unwrap();
            serie.push(($x, $y));

            if $x < pdata.min_x {
                pdata.min_x = $x
            };
            if $x > pdata.max_x {
                pdata.max_x = $x
            };
            if $y < pdata.min_y {
                pdata.min_y = $y
            };
            if $y > pdata.max_y {
                pdata.max_y = $y
            };
        }
    }};
}

/// Create new plot for your simulation
#[macro_export]
macro_rules! addplot {
    ($name:expr, $xlabel:expr, $ylabel:expr) => {{
        let mut data = DATA.lock().unwrap();
        if !data.contains_key(&$name) {
            data.insert($name, PlotData::new($name, $xlabel, $ylabel));
        }
    }};
}

/// Add a log to the simulation logger
#[macro_export]
macro_rules! log {
    ($ltype:expr, $message:expr) => {{
        //TODO: Avoid From String
        let mut logs = LOGS.lock().unwrap();
        logs.insert(
            0,
            Log {
                ltype: $ltype,
                body: $message,
            },
        );
    }};
}

#[macro_export]
/// Run simulation directly using this macro. Not based on `Simulation Terminal`.
///
/// s: istance of state of simulation
///
/// step: simulation step number
///
/// reps: # of repetitions
///
/// info: type of info you want to display during and after simulation
macro_rules! simulate_old {
    ($step:expr, $s:expr, $reps:expr, $info:expr) => {{
        let mut s = $s;
        let mut state = s.as_state_mut();
        let n_step: u64 = $step;

        let mut results: Vec<(Duration, f32)> = Vec::new();
        let option = $info;

        match option {
            Info::Verbose => {
                // println!("\u{1F980} krABMaga v1.0\n");
                // println!(
                //     "{0: >10}|{1: >9}|    {2: >11}|{3: >10}|",
                //     "#Rep", "Steps", "Steps/Seconds", "Time"
                // );
                // println!("--------------------------------------------------");
            }
            Info::Normal => {
                println!("{esc}c", esc = 27 as char);
                println!("\u{1F980} krABMaga v1.0\n");
                println!(
                    "{0: >10}|{1: >9}|    {2: >11}|{3: >10}|",
                    "#Rep", "Steps", "Avg. Steps/Seconds", "Avg. Time"
                );
                println!("----------------------------------------------------------------");
            }
        }
        // print!("{:width$}|", 1, width = 14 - $reps.to_string().len());
        // println!(
        //     "{:width$}|",
        //     n_step,
        //     width = 15 - n_step.to_string().len() - $reps.to_string().len()
        // );

        match option {
            Info::Verbose => {}
            Info::Normal => {
                println!("{esc}c", esc = 27 as char);
            }
        }

        for r in 0..$reps {
            let mut schedule: Schedule = Schedule::new();
            state.init(&mut schedule);
            let start = std::time::Instant::now();
            //let pb = ProgressBar::new(n_step);
            for i in 0..n_step {
                schedule.step(state);
                if state.end_condition(&mut schedule) {
                    break;
                }
                //pb.inc(1);
            }
            //pb.finish_with_message("\u{1F980}");

            let run_duration = start.elapsed();

            match option {
                Info::Verbose => {}
                Info::Normal => {
                    println!("{esc}c", esc = 27 as char);
                    println!("\u{1F980} krABMaga v1.0\n");
                    println!(
                        "{0: >10}|{1: >9}|    {2: >11}|{3: >10}|",
                        "#Rep", "Steps", "Avg. Steps/Seconds", "Avg. Time"
                    );
                    println!("----------------------------------------------------------------");
                }
            }

            // let step_seconds =
            //     format!("{:.0}", schedule.step as f32 / (run_duration.as_secs_f32()));
            // let time = format!("{:.4}", run_duration.as_secs_f32());
            // print!("{:width$}|", (r + 1), width = 14 - $reps.to_string().len());
            // print!(
            //     "{:width$}|",
            //     schedule.step,
            //     width = 15 - n_step.to_string().len() - $reps.to_string().len()
            // );
            // print!("{:width$}", "", width = 13 - step_seconds.len());

            results.push((
                run_duration,
                schedule.step as f32 / (run_duration.as_nanos() as f32 * 1e-9),
            ));

            match option {
                Info::Verbose => {
                    // print!("{}|", step_seconds);
                    // print!("{:width$}", "", width = 9 - time.len());
                    // println!("{}s|", time);
                }
                Info::Normal => {
                    let mut avg_time = 0.0;
                    let mut avg_step_seconds = 0.0;
                    for (time, step_seconds) in &results {
                        avg_time += time.as_secs_f32();
                        avg_step_seconds += step_seconds;
                    }
                    avg_time /= results.len() as f32;
                    avg_step_seconds /= results.len() as f32;
                    let avg_step_seconds = format!("{:.2}", avg_step_seconds);
                    let avg_time = format!("{:.4}", avg_time);
                    print!("{}|", avg_step_seconds);
                    print!("{:width$}", "", width = 9 - avg_time.len());
                    println!("{}s|", avg_time);
                }
            }
        }
        results
    }};
}

#[macro_use]
mod no_exported {
    #[macro_export]
    macro_rules! replace_expr {
        ($_t:tt $sub:expr) => {
            $sub
        };
    }

    //Used to count tokens of an expansion
    #[doc(hidden)]
    #[macro_export]
    macro_rules! count_tts {
        ($($tts:tt)*) => {<[()]>::len(&[$(replace_expr!($tts ())),*])};
    }

    #[macro_export]
    macro_rules! build_configurations{

        ($n_conf: expr, $( $input:ident )*) =>{{
        let mut config_table_index:Vec<Vec<usize>> = Vec::new();
        let mut input_size:usize = 0;
        let mut rep = $n_conf;
        {
            $(
                let mut row:Vec<usize> = Vec::with_capacity($n_conf);
                input_size = $input.len();
                rep /= input_size;
                let mut i = 0;
                for _ in 0..$n_conf{
                    for _ in 0..rep{
                            row.push(i);
                    }
                    i = (i + 1) % input_size;
                }
                config_table_index.push(row);
            )*
        }

        config_table_index
        }};

    }
}

///Create a csv file with the experiment results
///"DataFrame" trait allow the function to know field names and
///params list + output list for each configuration runned
pub fn write_csv<A: DataFrame>(name: &str, dataframe: &[A]) -> Result<(), Box<dyn Error>> {
    let csv_name = format!("{}.csv", name);
    let mut wtr = Writer::from_path(csv_name).expect("error on open the file path");
    //define column name
    wtr.write_record(A::field_names())?;

    for row in dataframe {
        wtr.serialize(row.to_string())?;
    }

    Ok(())
}

#[doc(hidden)]
//Trait implemented dynamically for our dataframe struct.
//Used into "export_dataframe" function
pub trait DataFrame {
    fn field_names() -> &'static [&'static str];
    fn to_string(&self) -> Vec<String>;
}

///Generate parameter values using a Uniform Distribution
///Params: Type, Min, Max and number of samples
///n_samples is optional, if omitted only a single sample is computed
#[macro_export]
macro_rules! gen_param {
    ( $type:ty, $min:expr, $max:expr, $n:expr) => {{
        let minimum: $type;
        let maximum: $type;
        minimum = $min;
        maximum = $max;
        let mut n = $n as usize;

        // Check parameters range to avoid error with Distribution
        let (minimum, maximum) = if minimum > maximum {
            (maximum, minimum)
        } else if minimum == maximum {
            (minimum, maximum + 1 as $type)
        } else {
            (minimum, maximum)
        };

        if n == 0 {
            n = 1;
        }

        let between = Uniform::from(minimum..maximum);
        let mut rng = rand::thread_rng();
        let dist: Vec<$type> = between.sample_iter(&mut rng).take($n).collect();

        dist
    }};

    // gen a single value
    (  $type:ty, $min:expr, $max:expr) => {{
        gen_param!($type, $min, $max, 1)
    }};
}

/// Load parameters from a csv
///
/// input_file: path to the csv
///
/// x: x_ty, couples of field names and their types.
#[macro_export]
macro_rules! load_csv {

    ($input_file: expr, $( $x:ident: $x_ty: ty ),*) =>{{
        let mut rdr = Reader::from_path($input_file).expect("error on read a file from path");
        $(
            let mut $x: Vec<$x_ty> = Vec::new();
        )*
        for result in rdr.records() {
            let record = result.expect("error on unwrap the record in csv file");
            let mut i = 0;
            $(
                let x : $x_ty = record[i].parse().expect("error on parsing the record");
                $x.push(x);
                i += 1;
            )*
        }
        let v = ($( $x, )*);
        v
    }};
}
