use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};
use std::{error::Error, io};

#[derive(Clone)]
struct Hotkey {
    software: &'static str,
    keys: &'static str,
    description: &'static str,
}

lazy_static::lazy_static! {
    static ref ALL_HOTKEYS: Vec<Hotkey> = vec![
        // =======================
        // VS Code
        // =======================
        // General
        Hotkey { software: "vscode", keys: "Ctrl+Shift+P", description: "Show Command Palette" },
        Hotkey { software: "vscode", keys: "Ctrl+P", description: "Quick Open, Go to File..." },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+N", description: "New window/instance" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+W", description: "Close window/instance" },
        Hotkey { software: "vscode", keys: "Ctrl+,", description: "User Settings" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+S", description: "Keyboard Shortcuts" },
        // Basic editing
        Hotkey { software: "vscode", keys: "Ctrl+X", description: "Cut line (empty selection)" },
        Hotkey { software: "vscode", keys: "Ctrl+C", description: "Copy line (empty selection)" },
        Hotkey { software: "vscode", keys: "Alt+Up/Down", description: "Move line up/down" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Up/Down", description: "Copy line up/down" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+K", description: "Delete line" },
        Hotkey { software: "vscode", keys: "Ctrl+Enter", description: "Insert line below" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+Enter", description: "Insert line above" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+\\", description: "Jump to matching bracket" },
        Hotkey { software: "vscode", keys: "Ctrl+] / [", description: "Indent/outdent line" },
        Hotkey { software: "vscode", keys: "Home / End", description: "Go to beginning/end of line" },
        Hotkey { software: "vscode", keys: "Ctrl+Home", description: "Go to beginning of file" },
        Hotkey { software: "vscode", keys: "Ctrl+End", description: "Go to end of file" },
        Hotkey { software: "vscode", keys: "Ctrl+Up/Down", description: "Scroll line up/down" },
        Hotkey { software: "vscode", keys: "Alt+PgUp/PgDn", description: "Scroll page up/down" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+[", description: "Fold (collapse) region" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+]", description: "Unfold (uncollapse) region" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+[", description: "Fold (collapse) all subregions" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+]", description: "Unfold (uncollapse) all subregions" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+0", description: "Fold (collapse) all regions" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+J", description: "Unfold (uncollapse) all regions" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+C", description: "Add line comment" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+U", description: "Remove line comment" },
        Hotkey { software: "vscode", keys: "Ctrl+/", description: "Toggle line comment" },
        Hotkey { software: "vscode", keys: "Shift+Alt+A", description: "Toggle block comment" },
        Hotkey { software: "vscode", keys: "Alt+Z", description: "Toggle word wrap" },
        // Navigation
        Hotkey { software: "vscode", keys: "Ctrl+T", description: "Show all Symbols" },
        Hotkey { software: "vscode", keys: "Ctrl+G", description: "Go to Line..." },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+O", description: "Go to Symbol..." },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+M", description: "Show Problems panel" },
        Hotkey { software: "vscode", keys: "F8", description: "Go to next error or warning" },
        Hotkey { software: "vscode", keys: "Shift+F8", description: "Go to previous error or warning" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+Tab", description: "Navigate editor group history" },
        Hotkey { software: "vscode", keys: "Alt+Left / Right", description: "Go back / forward" },
        Hotkey { software: "vscode", keys: "Ctrl+M", description: "Toggle Tab key moves focus" },
        // Search and replace
        Hotkey { software: "vscode", keys: "Ctrl+F", description: "Find" },
        Hotkey { software: "vscode", keys: "Ctrl+H", description: "Replace" },
        Hotkey { software: "vscode", keys: "F3 / Shift+F3", description: "Find next/previous" },
        Hotkey { software: "vscode", keys: "Alt+Enter", description: "Select all occurences of Find match" },
        Hotkey { software: "vscode", keys: "Ctrl+D", description: "Add selection to next Find match" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+D", description: "Move last selection to next Find match" },
        Hotkey { software: "vscode", keys: "Alt+C / R / W", description: "Toggle case-sensitive / regex / whole word" },
        // Multi-cursor and selection
        Hotkey { software: "vscode", keys: "Alt+Click", description: "Insert cursor" },
        Hotkey { software: "vscode", keys: "Ctrl+Alt+Up/Down", description: "Insert cursor above / below" },
        Hotkey { software: "vscode", keys: "Ctrl+U", description: "Undo last cursor operation" },
        Hotkey { software: "vscode", keys: "Shift+Alt+I", description: "Insert cursor at end of each line selected" },
        Hotkey { software: "vscode", keys: "Ctrl+L", description: "Select current line" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+L", description: "Select all occurrences of current selection" },
        Hotkey { software: "vscode", keys: "Ctrl+F2", description: "Select all occurrences of current word" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Right", description: "Expand selection" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Left", description: "Shrink selection" },
        // Rich languages editing
        Hotkey { software: "vscode", keys: "Ctrl+Space", description: "Trigger suggestion" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+Space", description: "Trigger parameter hints" },
        Hotkey { software: "vscode", keys: "Shift+Alt+F", description: "Format document" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+F", description: "Format selection" },
        Hotkey { software: "vscode", keys: "F12", description: "Go to Definition" },
        Hotkey { software: "vscode", keys: "Alt+F12", description: "Peek Definition" },
        Hotkey { software: "vscode", keys: "Ctrl+K F12", description: "Open Definition to the side" },
        Hotkey { software: "vscode", keys: "Ctrl+.", description: "Quick Fix" },
        Hotkey { software: "vscode", keys: "Shift+F12", description: "Show References" },
        Hotkey { software: "vscode", keys: "F2", description: "Rename Symbol" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+X", description: "Trim trailing whitespace" },
        Hotkey { software: "vscode", keys: "Ctrl+K M", description: "Change file language" },
        // Editor management
        Hotkey { software: "vscode", keys: "Ctrl+F4, Ctrl+W", description: "Close editor" },
        Hotkey { software: "vscode", keys: "Ctrl+K F", description: "Close folder" },
        Hotkey { software: "vscode", keys: "Ctrl+\\", description: "Split editor" },
        Hotkey { software: "vscode", keys: "Ctrl+1 / 2 / 3", description: "Focus into 1st, 2nd or 3rd editor group" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+Left/Right", description: "Focus into previous/next editor group" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+PgUp/PgDn", description: "Move editor left/right" },
        Hotkey { software: "vscode", keys: "Ctrl+K Left / Right", description: "Move active editor group" },
        // File management
        Hotkey { software: "vscode", keys: "Ctrl+N", description: "New File" },
        Hotkey { software: "vscode", keys: "Ctrl+O", description: "Open File..." },
        Hotkey { software: "vscode", keys: "Ctrl+S", description: "Save" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+S", description: "Save As..." },
        Hotkey { software: "vscode", keys: "Ctrl+K S", description: "Save All" },
        Hotkey { software: "vscode", keys: "Ctrl+F4", description: "Close" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+W", description: "Close All" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+T", description: "Reopen closed editor" },
        Hotkey { software: "vscode", keys: "Ctrl+K Enter", description: "Keep preview mode editor open" },
        Hotkey { software: "vscode", keys: "Ctrl+Tab", description: "Open next" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+Tab", description: "Open previous" },
        Hotkey { software: "vscode", keys: "Ctrl+K P", description: "Copy path of active file" },
        Hotkey { software: "vscode", keys: "Ctrl+K R", description: "Reveal active file in Explorer" },
        Hotkey { software: "vscode", keys: "Ctrl+K O", description: "Show active file in new window/instance" },
        // Display
        Hotkey { software: "vscode", keys: "F11", description: "Toggle full screen" },
        Hotkey { software: "vscode", keys: "Shift+Alt+0", description: "Toggle editor layout (horizontal/vertical)" },
        Hotkey { software: "vscode", keys: "Ctrl+=", description: "Zoom in" },
        Hotkey { software: "vscode", keys: "Ctrl+-", description: "Zoom out" },
        Hotkey { software: "vscode", keys: "Ctrl+B", description: "Toggle Sidebar visibility" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+E", description: "Show Explorer / Toggle focus" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+F", description: "Show Search" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+G", description: "Show Source Control" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+D", description: "Show Debug" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+X", description: "Show Extensions" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+H", description: "Replace in files" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+J", description: "Toggle Search details" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+U", description: "Show Output panel" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+V", description: "Open Markdown preview" },
        Hotkey { software: "vscode", keys: "Ctrl+K V", description: "Open Markdown preview to the side" },
        Hotkey { software: "vscode", keys: "Ctrl+`", description: "Show integrated terminal" },
        // Debug
        Hotkey { software: "vscode", keys: "F9", description: "Toggle breakpoint" },
        Hotkey { software: "vscode", keys: "F5", description: "Start/Continue" },
        Hotkey { software: "vscode", keys: "Shift+F5", description: "Stop" },
        Hotkey { software: "vscode", keys: "F11 / Shift+F11", description: "Step into/out" },
        Hotkey { software: "vscode", keys: "F10", description: "Step over" },

        // =======================
        // LazyVim & Neovim Core
        // =======================
        // Core Vim Movement
        Hotkey { software: "lazyvim", keys: "h / j / k / l", description: "Move left / down / up / right" },
        Hotkey { software: "lazyvim", keys: "w / W", description: "Move to next word (W ignores punctuation)" },
        Hotkey { software: "lazyvim", keys: "e / E", description: "Move to end of word" },
        Hotkey { software: "lazyvim", keys: "b / B", description: "Move to previous word" },
        Hotkey { software: "lazyvim", keys: "0", description: "Move to start of line" },
        Hotkey { software: "lazyvim", keys: "^", description: "Move to first non-blank character of line" },
        Hotkey { software: "lazyvim", keys: "$", description: "Move to end of line" },
        Hotkey { software: "lazyvim", keys: "gg", description: "Move to first line of file" },
        Hotkey { software: "lazyvim", keys: "G", description: "Move to last line of file" },
        Hotkey { software: "lazyvim", keys: "{ / }", description: "Move to previous / next paragraph" },
        Hotkey { software: "lazyvim", keys: "Ctrl+d / Ctrl+u", description: "Scroll half page down / up" },
        Hotkey { software: "lazyvim", keys: "Ctrl+f / Ctrl+b", description: "Scroll full page down / up" },
        Hotkey { software: "lazyvim", keys: "%", description: "Jump to matching bracket/brace" },
        // Core Vim Editing
        Hotkey { software: "lazyvim", keys: "i / I", description: "Insert before cursor / at beginning of line" },
        Hotkey { software: "lazyvim", keys: "a / A", description: "Append after cursor / at end of line" },
        Hotkey { software: "lazyvim", keys: "o / O", description: "Open new line below / above" },
        Hotkey { software: "lazyvim", keys: "x / X", description: "Delete character under / before cursor" },
        Hotkey { software: "lazyvim", keys: "r / R", description: "Replace single character / Enter Replace mode" },
        Hotkey { software: "lazyvim", keys: "s / S", description: "Substitute character / line" },
        Hotkey { software: "lazyvim", keys: "c / C", description: "Change (with motion) / Change to end of line" },
        Hotkey { software: "lazyvim", keys: "cc", description: "Change current line" },
        Hotkey { software: "lazyvim", keys: "d / D", description: "Delete (with motion) / Delete to end of line" },
        Hotkey { software: "lazyvim", keys: "dd", description: "Delete current line" },
        Hotkey { software: "lazyvim", keys: "y / yy", description: "Yank (copy with motion) / Yank current line" },
        Hotkey { software: "lazyvim", keys: "p / P", description: "Paste after / before cursor" },
        Hotkey { software: "lazyvim", keys: "u", description: "Undo" },
        Hotkey { software: "lazyvim", keys: "Ctrl+r", description: "Redo" },
        Hotkey { software: "lazyvim", keys: ".", description: "Repeat last change" },
        Hotkey { software: "lazyvim", keys: "~", description: "Toggle case of character under cursor" },
        // Core Vim Search
        Hotkey { software: "lazyvim", keys: "/", description: "Search forward" },
        Hotkey { software: "lazyvim", keys: "?", description: "Search backward" },
        Hotkey { software: "lazyvim", keys: "n / N", description: "Next / previous search match" },
        Hotkey { software: "lazyvim", keys: "* / #", description: "Search forward / backward for word under cursor" },
        // Core Vim Visual Mode
        Hotkey { software: "lazyvim", keys: "v", description: "Start character-wise visual mode" },
        Hotkey { software: "lazyvim", keys: "V", description: "Start line-wise visual mode" },
        Hotkey { software: "lazyvim", keys: "Ctrl+v", description: "Start block-wise visual mode" },
        Hotkey { software: "lazyvim", keys: "> / <", description: "Indent / outdent selected lines" },
        // Core Vim Window & Tab
        Hotkey { software: "lazyvim", keys: "Ctrl+w s / :sp", description: "Split window horizontally" },
        Hotkey { software: "lazyvim", keys: "Ctrl+w v / :vsp", description: "Split window vertically" },
        Hotkey { software: "lazyvim", keys: "Ctrl+w q", description: "Close window" },
        Hotkey { software: "lazyvim", keys: "Ctrl+w =", description: "Make all windows equal size" },
        Hotkey { software: "lazyvim", keys: "gt / gT", description: "Next / previous tab" },

        // LazyVim Specific: Find / Telescope
        Hotkey { software: "lazyvim", keys: "<leader>ff", description: "Find files (Telescope)" },
        Hotkey { software: "lazyvim", keys: "<leader>fF", description: "Find files in cwd (Telescope)" },
        Hotkey { software: "lazyvim", keys: "<leader>fg", description: "Find git files" },
        Hotkey { software: "lazyvim", keys: "<leader>fr", description: "Recent files" },
        Hotkey { software: "lazyvim", keys: "<leader>fR", description: "Recent files (cwd)" },
        Hotkey { software: "lazyvim", keys: "<leader>fc", description: "Find config file" },
        Hotkey { software: "lazyvim", keys: "<leader>fb", description: "Find buffers" },
        // Search
        Hotkey { software: "lazyvim", keys: "<leader>sg", description: "Live grep (root dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>sG", description: "Live grep (cwd)" },
        Hotkey { software: "lazyvim", keys: "<leader>sw", description: "Search word (root dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>sW", description: "Search word (cwd)" },
        Hotkey { software: "lazyvim", keys: "<leader>sb", description: "Search current buffer" },
        Hotkey { software: "lazyvim", keys: "<leader>sh", description: "Search help pages" },
        Hotkey { software: "lazyvim", keys: "<leader>sk", description: "Search keymaps" },
        Hotkey { software: "lazyvim", keys: "<leader>sc", description: "Search command history" },
        Hotkey { software: "lazyvim", keys: "<leader>sC", description: "Search commands" },
        Hotkey { software: "lazyvim", keys: "<leader>sd", description: "Search document diagnostics" },
        Hotkey { software: "lazyvim", keys: "<leader>sD", description: "Search workspace diagnostics" },
        Hotkey { software: "lazyvim", keys: "<leader>sm", description: "Jump to mark" },
        Hotkey { software: "lazyvim", keys: "<leader>sR", description: "Resume search" },
        // Explorer
        Hotkey { software: "lazyvim", keys: "<leader>e", description: "Toggle Neo-tree (root dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>E", description: "Toggle Neo-tree (cwd)" },
        // Buffers
        Hotkey { software: "lazyvim", keys: "<leader>bd", description: "Delete buffer" },
        Hotkey { software: "lazyvim", keys: "<leader>bD", description: "Delete buffer and window" },
        Hotkey { software: "lazyvim", keys: "<leader>bo", description: "Delete other buffers" },
        Hotkey { software: "lazyvim", keys: "<leader>bl", description: "Delete buffers to the left" },
        Hotkey { software: "lazyvim", keys: "<leader>br", description: "Delete buffers to the right" },
        Hotkey { software: "lazyvim", keys: "<leader>bb", description: "Switch to other buffer" },
        Hotkey { software: "lazyvim", keys: "<S-h> / [b", description: "Previous buffer" },
        Hotkey { software: "lazyvim", keys: "<S-l> / ]b", description: "Next buffer" },
        // Windows
        Hotkey { software: "lazyvim", keys: "<leader>w-", description: "Split window below" },
        Hotkey { software: "lazyvim", keys: "<leader>w|", description: "Split window right" },
        Hotkey { software: "lazyvim", keys: "<leader>wd", description: "Delete window" },
        Hotkey { software: "lazyvim", keys: "<C-h/j/k/l>", description: "Navigate windows" },
        // LSP & Code Navigation
        Hotkey { software: "lazyvim", keys: "gd", description: "Go to definition" },
        Hotkey { software: "lazyvim", keys: "gr", description: "Go to references" },
        Hotkey { software: "lazyvim", keys: "gD", description: "Go to declaration" },
        Hotkey { software: "lazyvim", keys: "gI", description: "Go to implementation" },
        Hotkey { software: "lazyvim", keys: "gy", description: "Go to type definition" },
        Hotkey { software: "lazyvim", keys: "K", description: "Hover documentation" },
        Hotkey { software: "lazyvim", keys: "gK", description: "Signature help" },
        Hotkey { software: "lazyvim", keys: "<c-k>", description: "Signature help (insert mode)" },
        Hotkey { software: "lazyvim", keys: "<leader>ca", description: "Code action" },
        Hotkey { software: "lazyvim", keys: "<leader>cr", description: "Rename symbol" },
        Hotkey { software: "lazyvim", keys: "<leader>cf", description: "Format document" },
        Hotkey { software: "lazyvim", keys: "[d", description: "Previous diagnostic" },
        Hotkey { software: "lazyvim", keys: "]d", description: "Next diagnostic" },
        Hotkey { software: "lazyvim", keys: "gl", description: "Line diagnostics" },
        // Git
        Hotkey { software: "lazyvim", keys: "<leader>gg", description: "Lazygit" },
        Hotkey { software: "lazyvim", keys: "<leader>gG", description: "Lazygit (cwd)" },
        // App / Misc
        Hotkey { software: "lazyvim", keys: "<leader>l", description: "Lazy plugin manager" },
        Hotkey { software: "lazyvim", keys: "<leader>cm", description: "Mason (package manager)" },
        Hotkey { software: "lazyvim", keys: "<leader>qq", description: "Quit all" },
        Hotkey { software: "lazyvim", keys: "<leader>uh", description: "Toggle Inlay Hints" },

        // =======================
        // tmux
        // =======================
        // Sessions & Windows
        Hotkey { software: "tmux", keys: "Prefix + c", description: "Create new window" },
        Hotkey { software: "tmux", keys: "Prefix + d", description: "Detach session" },
        Hotkey { software: "tmux", keys: "Prefix + s", description: "List sessions" },
        Hotkey { software: "tmux", keys: "Prefix + w", description: "List windows" },
        Hotkey { software: "tmux", keys: "Prefix + n", description: "Next window" },
        Hotkey { software: "tmux", keys: "Prefix + p", description: "Previous window" },
        Hotkey { software: "tmux", keys: "Prefix + &", description: "Kill current window" },
        Hotkey { software: "tmux", keys: "Prefix + ,", description: "Rename current window" },
        Hotkey { software: "tmux", keys: "Prefix + $", description: "Rename current session" },
        // Panes
        Hotkey { software: "tmux", keys: "Prefix + %", description: "Split window vertically (left/right)" },
        Hotkey { software: "tmux", keys: "Prefix + \"", description: "Split window horizontally (top/bottom)" },
        Hotkey { software: "tmux", keys: "Prefix + x", description: "Kill current pane" },
        Hotkey { software: "tmux", keys: "Prefix + z", description: "Toggle pane zoom (maximize)" },
        Hotkey { software: "tmux", keys: "Prefix + <Arrow>", description: "Switch to pane in direction" },
        Hotkey { software: "tmux", keys: "Prefix + Ctrl+<Arrow>", description: "Resize pane in direction" },
        Hotkey { software: "tmux", keys: "Prefix + {", description: "Swap pane left" },
        Hotkey { software: "tmux", keys: "Prefix + }", description: "Swap pane right" },
        Hotkey { software: "tmux", keys: "Prefix + q", description: "Show pane numbers" },
        Hotkey { software: "tmux", keys: "Prefix + !", description: "Break pane into new window" },
        // Copy Mode & Misc
        Hotkey { software: "tmux", keys: "Prefix + [", description: "Enter copy mode" },
        Hotkey { software: "tmux", keys: "Prefix + ]", description: "Paste from buffer" },
        Hotkey { software: "tmux", keys: "Prefix + t", description: "Show large clock" },
        Hotkey { software: "tmux", keys: "Prefix + ?", description: "List all keybindings" },
    ];
}

const SOFTWARES: &[&str] = &["all", "vscode", "lazyvim", "tmux"];

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Search,
    SelectSoftware,
}

struct App {
    mode: Mode,
    query: String,
    state: ListState,
    filtered_indices: Vec<(usize, i64)>, // index in ALL_HOTKEYS, score

    selected_software: String,

    software_query: String,
    software_state: ListState,
    filtered_softwares: Vec<String>,
}

impl App {
    fn new() -> App {
        let mut app = App {
            mode: Mode::Search,
            query: String::new(),
            state: ListState::default(),
            filtered_indices: Vec::new(),
            selected_software: "all".to_string(),
            software_query: String::new(),
            software_state: ListState::default(),
            filtered_softwares: SOFTWARES.iter().map(|s| s.to_string()).collect(),
        };
        app.update_search();
        app
    }

    fn update_search(&mut self) {
        let matcher = SkimMatcherV2::default();
        let mut matches = Vec::new();

        for (i, hotkey) in ALL_HOTKEYS.iter().enumerate() {
            if self.selected_software != "all" && hotkey.software != self.selected_software {
                continue;
            }

            if self.query.is_empty() {
                matches.push((i, 0));
            } else {
                // Match against both keys and description
                let target = format!("{} {}", hotkey.keys, hotkey.description);
                if let Some(score) = matcher.fuzzy_match(&target, &self.query) {
                    matches.push((i, score));
                }
            }
        }

        if !self.query.is_empty() {
            matches.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by score descending
        }

        self.filtered_indices = matches;

        if self.filtered_indices.is_empty() {
            self.state.select(None);
        } else {
            self.state.select(Some(0));
        }
    }

    fn update_software_search(&mut self) {
        let matcher = SkimMatcherV2::default();
        let mut matches = Vec::new();

        if self.software_query.is_empty() {
            self.filtered_softwares = SOFTWARES.iter().map(|s| s.to_string()).collect();
        } else {
            for sw in SOFTWARES.iter() {
                if let Some(score) = matcher.fuzzy_match(sw, &self.software_query) {
                    matches.push((sw.to_string(), score));
                }
            }
            matches.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered_softwares = matches.into_iter().map(|(s, _)| s).collect();
        }

        if self.filtered_softwares.is_empty() {
            self.software_state.select(None);
        } else {
            self.software_state.select(Some(0));
        }
    }

    fn next(&mut self) {
        match self.mode {
            Mode::Search => {
                let i = match self.state.selected() {
                    Some(i) => {
                        if i >= self.filtered_indices.len().saturating_sub(1) {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                if !self.filtered_indices.is_empty() {
                    self.state.select(Some(i));
                }
            }
            Mode::SelectSoftware => {
                let i = match self.software_state.selected() {
                    Some(i) => {
                        if i >= self.filtered_softwares.len().saturating_sub(1) {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                if !self.filtered_softwares.is_empty() {
                    self.software_state.select(Some(i));
                }
            }
        }
    }

    fn previous(&mut self) {
        match self.mode {
            Mode::Search => {
                let i = match self.state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.filtered_indices.len().saturating_sub(1)
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                if !self.filtered_indices.is_empty() {
                    self.state.select(Some(i));
                }
            }
            Mode::SelectSoftware => {
                let i = match self.software_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.filtered_softwares.len().saturating_sub(1)
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                if !self.filtered_softwares.is_empty() {
                    self.software_state.select(Some(i));
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>>
where
    <B as Backend>::Error: 'static + Error,
{
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match app.mode {
                    Mode::Search => match key.code {
                        KeyCode::Esc => return Ok(()),
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Backspace => {
                            app.query.pop();
                            app.update_search();
                        }
                        KeyCode::Char(':') if app.query.is_empty() => {
                            app.mode = Mode::SelectSoftware;
                            app.software_query.clear();
                            app.update_software_search();
                        }
                        KeyCode::Char(c) => {
                            app.query.push(c);
                            app.update_search();
                        }
                        _ => {}
                    },
                    Mode::SelectSoftware => match key.code {
                        KeyCode::Esc => {
                            app.mode = Mode::Search;
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Tab => {
                            if let Some(i) = app.software_state.selected() {
                                if let Some(sw) = app.filtered_softwares.get(i) {
                                    app.software_query = sw.clone();
                                    app.update_software_search();
                                }
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(i) = app.software_state.selected() {
                                if let Some(sw) = app.filtered_softwares.get(i) {
                                    app.selected_software = sw.clone();
                                    app.mode = Mode::Search;
                                    app.update_search();
                                }
                            } else {
                                app.mode = Mode::Search;
                            }
                        }
                        KeyCode::Backspace => {
                            app.software_query.pop();
                            app.update_software_search();
                        }
                        KeyCode::Char(c) => {
                            app.software_query.push(c);
                            app.update_software_search();
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

// Helper to center a rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// NullDark Theme Color Tokens
const THEME_BG_OUTER: Color = Color::Rgb(24, 24, 24);      // #181818
const THEME_BG_PAGE: Color = Color::Rgb(30, 30, 30);       // #1e1e1e
const THEME_BG_PANEL: Color = Color::Rgb(36, 36, 36);      // #242424
const THEME_BG_ACTIVE: Color = Color::Rgb(42, 42, 42);     // #2a2a2a
const THEME_BORDER: Color = Color::Rgb(54, 54, 54);        // #363636
const THEME_BORDER_SUBTLE: Color = Color::Rgb(44, 44, 44); // #2c2c2c

const THEME_ACCENT: Color = Color::Rgb(214, 6, 69);        // #d60645 (Signature Magenta)
const THEME_ACCENT_BRIGHT: Color = Color::Rgb(222, 27, 84);// #de1b54
const THEME_SUCCESS: Color = Color::Rgb(16, 185, 129);     // #10b981

const THEME_TEXT_BRIGHT: Color = Color::Rgb(189, 189, 189);// #bdbcbc
const THEME_TEXT_NORMAL: Color = Color::Rgb(168, 168, 168);// #a8a8a8
const THEME_TEXT_MUTED: Color = Color::Rgb(120, 120, 120); // #787878
const THEME_TEXT_DIM: Color = Color::Rgb(85, 85, 85);      // #555555

fn ui(f: &mut Frame, app: &mut App) {
    // Fill entire terminal with NullDark outer background (#181818)
    f.render_widget(Block::default().style(Style::default().bg(THEME_BG_OUTER)), f.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // Brand Header
            Constraint::Length(3), // Search Bar
            Constraint::Min(1),    // Results List
            Constraint::Length(1), // Status Footer
        ].as_ref())
        .split(f.area());

    // 1. Brand Header
    let header_line = Line::from(vec![
        Span::styled("NULL", Style::default().fg(THEME_TEXT_MUTED).add_modifier(Modifier::BOLD)),
        Span::styled("HOTKEYS", Style::default().fg(THEME_TEXT_BRIGHT).add_modifier(Modifier::BOLD)),
        Span::styled(" | ", Style::default().fg(THEME_BORDER)),
        Span::styled("KEYBOARD SHORTCUT WORKSPACE", Style::default().fg(THEME_TEXT_MUTED)),
        Span::styled("  •  ", Style::default().fg(THEME_BORDER)),
        Span::styled(format!("[{}]", app.selected_software.to_uppercase()), Style::default().fg(THEME_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled(format!(" ({} shortcuts)", app.filtered_indices.len()), Style::default().fg(THEME_TEXT_DIM)),
    ]);
    f.render_widget(Paragraph::new(header_line), chunks[0]);

    // 2. Search Input
    let cursor_char = if app.mode == Mode::Search { "█" } else { "" };
    let input_line = Line::from(vec![
        Span::styled("> ", Style::default().fg(THEME_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled(&app.query, Style::default().fg(THEME_TEXT_BRIGHT)),
        Span::styled(cursor_char, Style::default().fg(THEME_ACCENT)),
    ]);
    let input = Paragraph::new(input_line)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(THEME_BORDER))
                .style(Style::default().bg(THEME_BG_PANEL))
                .title(Span::styled(" SEARCH ", Style::default().fg(THEME_TEXT_MUTED).add_modifier(Modifier::BOLD)))
        );
    f.render_widget(input, chunks[1]);

    // 3. Results List
    let items: Vec<ListItem> = app
        .filtered_indices
        .iter()
        .map(|&(i, _)| {
            let hotkey = &ALL_HOTKEYS[i];

            let sw_span = Span::styled(
                format!("[{:<7}] ", hotkey.software.to_uppercase()),
                Style::default().fg(THEME_TEXT_MUTED),
            );

            let keys_span = Span::styled(
                format!("{:<26}", hotkey.keys),
                Style::default()
                    .fg(THEME_TEXT_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            );

            let desc_span = Span::styled(
                hotkey.description,
                Style::default().fg(THEME_TEXT_NORMAL),
            );
            ListItem::new(Line::from(vec![sw_span, keys_span, desc_span]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(THEME_BORDER))
                .style(Style::default().bg(THEME_BG_PAGE))
                .title(Span::styled(" SHORTCUTS ", Style::default().fg(THEME_TEXT_MUTED).add_modifier(Modifier::BOLD)))
        )
        .highlight_style(
            Style::default()
                .bg(THEME_BG_ACTIVE)
                .fg(THEME_ACCENT_BRIGHT)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[2], &mut app.state);

    // 4. Status Footer
    let footer_line = Line::from(vec![
        Span::styled("↑↓ ", Style::default().fg(THEME_TEXT_BRIGHT)),
        Span::styled("Navigate  ", Style::default().fg(THEME_TEXT_DIM)),
        Span::styled(": ", Style::default().fg(THEME_TEXT_BRIGHT)),
        Span::styled("Select Software  ", Style::default().fg(THEME_TEXT_DIM)),
        Span::styled("Esc ", Style::default().fg(THEME_TEXT_BRIGHT)),
        Span::styled("Clear  ", Style::default().fg(THEME_TEXT_DIM)),
        Span::styled("q / Ctrl+C ", Style::default().fg(THEME_TEXT_BRIGHT)),
        Span::styled("Quit", Style::default().fg(THEME_TEXT_DIM)),
    ]);
    f.render_widget(Paragraph::new(footer_line), chunks[3]);

    // 5. Software Selection Modal
    if let Mode::SelectSoftware = app.mode {
        let area = centered_rect(50, 45, f.area());
        f.render_widget(Clear, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(THEME_ACCENT))
            .title(Span::styled(" FILTER BY SOFTWARE (Tab autocomplete, Enter select, Esc cancel) ", Style::default().fg(THEME_TEXT_BRIGHT).add_modifier(Modifier::BOLD)))
            .style(Style::default().bg(THEME_BG_PANEL));
        f.render_widget(popup_block, area);

        let sw_input_line = Line::from(vec![
            Span::styled(":", Style::default().fg(THEME_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(&app.software_query, Style::default().fg(THEME_TEXT_BRIGHT)),
            Span::styled("█", Style::default().fg(THEME_ACCENT)),
        ]);
        let sw_input = Paragraph::new(sw_input_line)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default().fg(THEME_BORDER_SUBTLE))
            );
        f.render_widget(sw_input, popup_chunks[0]);

        let sw_items: Vec<ListItem> = app
            .filtered_softwares
            .iter()
            .map(|s| {
                let is_current = app.selected_software == *s;
                if is_current {
                    ListItem::new(Line::from(vec![
                        Span::styled(format!("{:<15} ", s.to_uppercase()), Style::default().fg(THEME_TEXT_BRIGHT).add_modifier(Modifier::BOLD)),
                        Span::styled("[ACTIVE]", Style::default().fg(THEME_SUCCESS).add_modifier(Modifier::BOLD)),
                    ]))
                } else {
                    ListItem::new(Span::styled(s.to_uppercase(), Style::default().fg(THEME_TEXT_NORMAL)))
                }
            })
            .collect();

        let sw_list = List::new(sw_items)
            .highlight_style(
                Style::default()
                    .bg(THEME_BG_ACTIVE)
                    .fg(THEME_ACCENT)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        f.render_stateful_widget(sw_list, popup_chunks[1], &mut app.software_state);
    }
}
