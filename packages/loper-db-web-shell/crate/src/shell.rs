use crate::arrow_printer::{pretty_format_batches, UTF8_BORDERS_NO_HORIZONTAL};
use crate::loper::{
    LoperServiceClient, LoperServiceConnection, PACKAGE_NAME, PACKAGE_VERSION, JsLoperServiceClient,
};
use crate::key_event::{Key, KeyEvent};
use crate::prompt_buffer::PromptBuffer;
use crate::shell_options::ShellOptions;
use crate::shell_runtime::{ShellRuntime};
use crate::utils::{now, pretty_elapsed};
use crate::vt100;
use crate::xterm::Terminal;
use arrow::array::Array;
use arrow::array::StringArray;
use arrow::datatypes::{DataType};
use chrono::Duration;
use log::warn;
use scopeguard::defer;
use std::cell::RefCell;
use std::collections::{VecDeque};
use std::sync::Arc;
use std::sync::RwLock;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

thread_local! {
    static SHELL: RefCell<Shell> = RefCell::new(Shell::default());
}

const HISTORY_LENGTH: usize = 1000;

/// A shell input context
#[wasm_bindgen]
pub enum ShellInputContext {
    FileInput = 0,
}

/// Shell settings
struct ShellSettings {
    /// Enable query output
    output: bool,
    /// Enable query timer
    timer: bool,
    /// Is WebGL enabled?
    webgl: bool,
}

impl ShellSettings {
    fn default() -> Self {
        Self {
            output: true,
            timer: true,
            webgl: false,
        }
    }
}

/// The shell is the primary entrypoint for the web shell api.
/// It is stored as thread_local singleton and maintains all the state for the interactions with the database service
pub struct Shell {
    /// The shell settings
    settings: ShellSettings,
    /// The actual xterm terminal instance
    terminal: Terminal,
    /// The terminal width
    terminal_width: usize,
    /// The runtime
    runtime: Option<Arc<RwLock<ShellRuntime>>>,
    /// The current line buffer
    input: PromptBuffer,
    /// The input is enabled
    input_enabled: bool,
    /// The input clock
    input_clock: u64,
    /// This history buffer
    history: VecDeque<String>,
    /// This history buffer
    history_cursor: usize,
    /// The database path
    service_url: String,
    /// The client (if any)
    service_client: Option<Arc<RwLock<LoperServiceClient>>>,
    /// The connection (if any)
    service_conn: Option<Arc<RwLock<LoperServiceConnection>>>,
}

impl Shell {
    /// Construct a shell
    fn default() -> Self {
        Self {
            settings: ShellSettings::default(),
            terminal: Terminal::construct(None),
            terminal_width: 100,
            runtime: None,
            input: PromptBuffer::default(),
            input_enabled: false,
            input_clock: 0,
            history: VecDeque::new(),
            history_cursor: 0,
            service_url: "http://0.0.0.0:8080".to_string(),
            service_client: None,
            service_conn: None,
        }
    }

    /// Attach to a terminal
    pub fn attach(&mut self, term: Terminal, runtime: ShellRuntime, options: ShellOptions) {
        self.terminal = term;
        self.terminal_width = self.terminal.get_cols() as usize;
        self.runtime = Some(Arc::new(RwLock::new(runtime)));
        self.input.configure(self.terminal_width);
        self.settings.webgl = options.with_webgl();

        // Register on_key callback
        let callback = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            Shell::on_key(e);
            false
        }) as Box<dyn FnMut(_) -> bool>);
        self.terminal
            .attach_custom_key_event_handler(callback.as_ref().unchecked_ref());
        callback.forget();
    }

    /// Run initial setup
    pub async fn initial_setup() -> Result<(), js_sys::Error> {
        // Disconnect any existing connection
        let conn = Shell::with_mut(|s| s.service_conn.clone());
        if let Some(ref conn) = conn {
            let conn_guard = conn.read().unwrap();
            conn_guard.disconnect().await?;
        }

        // Create service client
        let (rt_ptr, service_url) = Shell::with_mut(|s| (s.runtime.clone().unwrap(), s.service_url.clone()));
        let rt = rt_ptr.read().unwrap();
        let client_js: JsLoperServiceClient = rt.configure_client(&service_url).await.unwrap().into();
        let client = LoperServiceClient::from_bindings(client_js);

        // Store new database and reset the connection
        let client = Shell::with_mut(|s| {
            let client = Arc::new(RwLock::new(client));
            s.service_conn = None;
            s.service_client = Some(client.clone());
            client
        });

        let conn = LoperServiceClient::connect(client.clone()).await?;

        // Create connection
        Shell::with_mut(|s| {
            s.service_conn = Some(Arc::new(RwLock::new(conn)));
            s.clear_and_greet();
            s.prompt();
            s.focus();
        });
        Ok(())
    }

    /// Load input history
    pub fn load_history(history: Vec<String>, cursor: usize) {
        let mut h = VecDeque::with_capacity(history.len());
        for entry in &history[cursor..history.len()] {
            h.push_back(entry.clone());
        }
        for entry in &history[0..cursor] {
            h.push_back(entry.clone());
        }
        Shell::with_mut(|s| {
            s.history_cursor = h.len();
            s.history = h;
        });
    }

    /// Write directly to the terminal
    pub fn write(&self, text: &str) {
        self.terminal.write(text);
    }

    /// Write directly to the terminal with newline
    pub fn writeln(&self, text: &str) {
        self.terminal.write(&format!("{}{}", text, vt100::CRLF));
    }

    /// Clear the screen
    pub fn clear(&mut self) {
        self.clear_and_greet();
        self.prompt();
    }

    /// Block all input
    pub fn block_input(&mut self) {
        self.input_enabled = false;
    }

    /// Resume after user input
    pub fn resume_after_input(&mut self, _ctx: ShellInputContext) {
        self.prompt();
    }

    fn remember_command(&mut self, text: String) {
        self.history.push_back(text.clone());
        if self.history.len() > HISTORY_LENGTH {
            self.history.pop_front();
        }
        self.history_cursor = self.history.len();
        if let Some(ref rt) = self.runtime {
            let rt_copy = rt.clone();
            spawn_local(async move {
                match rt_copy.read().unwrap().push_input_to_history(&text).await {
                    Ok(_) => (),
                    Err(_e) => (),
                }
            });
        }
    }

    /// Command handler
    pub async fn on_command(text: String) {
        let trimmed = text.trim();
        Shell::with(|s| s.writeln("")); // XXX We could validate the input first and preserve the prompt

        let cmd = &trimmed[..trimmed.find(' ').unwrap_or_else(|| trimmed.len())];
        let args = trimmed[cmd.len()..].trim();
        match cmd {
            ".clear" => {
                Shell::with_mut(|s| {
                    s.remember_command(text.clone());
                    s.clear();
                });
                return;
            }
            ".help" => Shell::with(|s| {
                s.write(&format!(
                    concat!(
                        "┌ .clear            Clear the shell.\r\n",
                        "└ .output on|off    Print results on or off.\r\n",
                    ),
                ));
            }),
            ".output" => Shell::with_mut(|s| {
                if args.ends_with("on") {
                    s.settings.output = true;
                    s.writeln("Output enabled");
                } else if args.ends_with("off") {
                    s.settings.output = false;
                    s.writeln("Output disabled");
                } else {
                    s.writeln("Usage: .Output [on/off]")
                }
            }),
            ".timer" => Shell::with_mut(|s| {
                if args.ends_with("on") {
                    s.settings.timer = true;
                    s.writeln("Timer enabled");
                } else if args.ends_with("off") {
                    s.settings.timer = false;
                    s.writeln("Timer disabled");
                } else {
                    s.writeln("Usage: .timer [on/off]")
                }
            }),
            cmd => Shell::with(|s| s.writeln(&format!("Unknown command: {}", &cmd))),
        }
        Shell::with_mut(|s| {
            s.remember_command(text.clone());
            s.writeln("");
            s.prompt();
        });
    }

    /// Command handler
    async fn on_sql(text: String) {
        defer!({
            Shell::with_mut(|s| {
                s.remember_command(text.clone());
                s.writeln("");
                s.prompt();
            })
        });

        // Get the database connection
        let (maybe_conn, use_timer, terminal_width) = Shell::with(|shell| {
            shell.writeln("");
            (
                shell.service_conn.clone(),
                shell.settings.timer,
                shell.terminal_width,
            )
        });
        // Lock the connection
        let conn = match maybe_conn {
            Some(ref conn) => conn.read().unwrap(),
            None => {
                Shell::with_mut(|s| {
                    s.writeln("Error: connection not set");
                });
                return;
            }
        };

        // Run the query
        let start = now();
        let batches = match conn.run_query(&text).await {
            Ok(batches) => batches,
            Err(e) => {
                let mut msg: String = e.message().into();
                msg = msg.replace("\n", "\r\n");
                Shell::with_mut(|s| {
                    s.writeln(&msg);
                });
                return;
            }
        };
        let elapsed = if use_timer {
            Duration::milliseconds((now() - start) as i64)
        } else {
            Duration::milliseconds(0)
        };

        // Detect explain result
        if batches.len() == 1 {
            let first = batches.first().unwrap();
            let schema = &first.schema();
            let fields = schema.fields();
            if fields.len() == 2
                && fields[0].name() == "explain_key"
                && fields[1].name() == "explain_value"
                && first.num_rows() == 1
                && first.column(0).data_type().eq(&DataType::Utf8)
                && first.column(1).data_type().eq(&DataType::Utf8)
            {
                let array = first
                    .column(1)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap();
                if !array.is_null(0) {
                    let mut explain = array.value(0).to_string();
                    explain = explain.replace("\n", "\r\n");
                    Shell::with_mut(|s| {
                        s.write(&explain);
                    });
                    return;
                }
            }
        }

        Shell::with_mut(|s| {
            // Print the table
            if s.settings.output {
                let pretty_table = pretty_format_batches(
                    &batches,
                    terminal_width as u16,
                    UTF8_BORDERS_NO_HORIZONTAL,
                )
                .unwrap_or_default();
                s.writeln(&pretty_table);
            }

            // Print elapsed time (if requested)
            if s.settings.timer {
                s.writeln(&format!(
                    "{bold}Elapsed:{normal} {elapsed}",
                    elapsed = pretty_elapsed(&elapsed),
                    bold = vt100::MODE_BOLD,
                    normal = vt100::MODES_OFF,
                ));
            }
        });
    }

    /// Flush output buffer to the terminal
    pub fn flush(&mut self) {
        self.input.flush(&self.terminal);
    }

    /// Highlight input text (if sql)
    fn highlight_input() {
        let (input, input_clock) = Shell::with_mut(|s| (s.input.collect(), s.input_clock));
        if input.trim_start().starts_with('.') {
            return;
        }
        let db_ptr = Shell::with(|s| s.service_client.clone()).unwrap();
        spawn_local(async move {
            let db = match db_ptr.read() {
                Ok(guard) => guard,
                Err(_) => return,
            };
            let tokens = match db.tokenize(&input).await {
                Ok(t) => t,
                Err(_) => return,
            };
            Shell::with_mut(|s| {
                if s.input_clock != input_clock {
                    return;
                }
                s.input.highlight_sql(tokens);
                s.flush();
            });
        });
    }

    /// Process on-key event
    fn on_key(keyboard_event: web_sys::KeyboardEvent) {
        if !Shell::with(|s| s.input_enabled) {
            return;
        }
        if &keyboard_event.type_() != "keydown" {
            return;
        }
        let event = KeyEvent::from_event(keyboard_event.clone());
        match event.key {
            Key::Enter => {
                let input = Shell::with_mut(|s| {
                    s.input_clock += 1;
                    s.input.collect()
                });
                // Is a command?
                if input.trim_start().starts_with('.') {
                    Shell::with_mut(|s| s.block_input());
                    spawn_local(Shell::on_command(input));
                } else {
                    // Ends with semicolon?
                    if input.trim_end().ends_with(';') {
                        Shell::with_mut(|s| s.block_input());
                        spawn_local(Shell::on_sql(input));
                    } else {
                        Shell::with_mut(|s| {
                            s.input.consume(event);
                            s.flush();
                        });
                    }
                }
            }
            Key::ArrowUp => {
                let should_highlight = Shell::with_mut(|s| -> bool {
                    if s.history_cursor > 0 {
                        s.history_cursor -= 1;
                        s.input_clock += 1;
                        s.input.replace(&s.history[s.history_cursor]);
                        s.flush();
                        return true;
                    }
                    false
                });
                if should_highlight {
                    Shell::highlight_input();
                }
            }
            Key::ArrowDown => {
                let should_highlight = Shell::with_mut(|s| -> bool {
                    if s.history_cursor < s.history.len() {
                        s.history_cursor += 1;
                        s.input.replace(if s.history_cursor < s.history.len() {
                            s.history[s.history_cursor].as_str()
                        } else {
                            ""
                        });
                        s.input_clock += 1;
                        s.flush();
                        return true;
                    }
                    false
                });
                if should_highlight {
                    Shell::highlight_input();
                }
            }
            Key::Backspace => {
                Shell::with_mut(|s| {
                    s.input_clock += 1;
                    s.input.consume(event);
                    s.flush();
                });
                Shell::highlight_input();
            }
            Key::ArrowLeft | Key::ArrowRight => {
                Shell::with_mut(|s| {
                    s.input_clock += 1;
                    s.input.consume(event);
                    s.flush();
                });
            }
            _ => {
                if keyboard_event.ctrl_key() || keyboard_event.meta_key() {
                    spawn_local(Shell::on_key_combination(keyboard_event, event));
                    return;
                }
                Shell::with_mut(|s| {
                    s.input_clock += 1;
                    s.input.consume(event);
                    s.flush();
                });
                Shell::highlight_input();
            }
        }
    }

    /// Handle pressed key combinations such as ctrl+c & ctrl+v
    async fn on_key_combination(keyboard_event: web_sys::KeyboardEvent, event: KeyEvent) {
        let rt_ptr = Shell::with_mut(|s| s.runtime.clone()).unwrap();
        let rt = rt_ptr.read().unwrap();
        if keyboard_event.ctrl_key() || keyboard_event.meta_key() {
            match event.key {
                Key::Char('v') => match rt.read_clipboard_text().await {
                    Ok(v) => {
                        Shell::with_mut(|s| {
                            s.input
                                .insert_text(&v.as_string().unwrap_or_else(|| ' '.to_string()));
                            s.input.flush(&s.terminal);
                        });
                        Shell::highlight_input();
                    }
                    Err(e) => warn!("Failed to read from clipboard: {:?}", e.to_string()),
                },
                Key::Char('a') => {
                    Shell::with_mut(|s| {
                        s.input.move_cursor_to(0);
                        s.input.flush(&s.terminal);
                    });
                }
                Key::Char('e') => {
                    Shell::with_mut(|s| {
                        s.input.move_cursor_to_end();
                        s.input.flush(&s.terminal);
                    });
                }
                Key::Char('c') => (),
                _ => {}
            }
        }
    }

    /// Clear the screen and print the greeter
    fn clear_and_greet(&self) {
        self.write(&format!(
            concat!(
                "{clear_screen}{cursor_home}",
                "{bold}Loper Web Shell{normal}{endl}",
                "┌ Client: {bold}{package_name}@{package_version}{normal}{endl}",
                "├ Remote: {bold}{url}{normal}{endl}",
                "└ Session: {bold}ad-hoc{normal}{endl}",
                "{endl}",
                "Enter .help for usage hints.{endl}",
                "{endl}"
            ),
            clear_screen = vt100::CLEAR_SCREEN,
            cursor_home = vt100::CURSOR_HOME,
            bold = vt100::MODE_BOLD,
            normal = vt100::MODES_OFF,
            endl = vt100::CRLF,
            package_name = PACKAGE_NAME,
            package_version = PACKAGE_VERSION,
            url = self.service_url.clone()
        ));
    }

    /// Write the prompt
    pub fn prompt(&mut self) {
        self.input.start_new();
        self.input.flush(&self.terminal);
        self.input_enabled = true;
    }

    /// Focus on the terminal
    pub fn focus(&self) {
        self.terminal.focus();
    }

    // Borrow shell immutable
    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&Shell) -> R,
    {
        SHELL.with(|s| f(&s.borrow()))
    }

    // Borrow shell mutable
    pub fn with_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut Shell) -> R,
    {
        SHELL.with(|s| f(&mut s.borrow_mut()))
    }
}
