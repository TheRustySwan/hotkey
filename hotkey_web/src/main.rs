use dioxus::prelude::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

#[derive(Clone, PartialEq)]
struct Hotkey {
    software: &'static str,
    keys: &'static str,
    description: &'static str,
}

lazy_static::lazy_static! {
    static ref ALL_HOTKEYS: Vec<Hotkey> = vec![
        // VS Code
        Hotkey { software: "vscode", keys: "Ctrl+Shift+P", description: "Show Command Palette" },
        Hotkey { software: "vscode", keys: "Ctrl+P", description: "Quick Open, Go to File..." },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+N", description: "New window/instance" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+W", description: "Close window/instance" },
        Hotkey { software: "vscode", keys: "Ctrl+,", description: "User Settings" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+S", description: "Keyboard Shortcuts" },
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
        Hotkey { software: "vscode", keys: "Ctrl+T", description: "Show all Symbols" },
        Hotkey { software: "vscode", keys: "Ctrl+G", description: "Go to Line..." },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+O", description: "Go to Symbol..." },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+M", description: "Show Problems panel" },
        Hotkey { software: "vscode", keys: "F8", description: "Go to next error or warning" },
        Hotkey { software: "vscode", keys: "Shift+F8", description: "Go to previous error or warning" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+Tab", description: "Navigate editor group history" },
        Hotkey { software: "vscode", keys: "Alt+Left / Right", description: "Go back / forward" },
        Hotkey { software: "vscode", keys: "Ctrl+M", description: "Toggle Tab key moves focus" },
        Hotkey { software: "vscode", keys: "Ctrl+F", description: "Find" },
        Hotkey { software: "vscode", keys: "Ctrl+H", description: "Replace" },
        Hotkey { software: "vscode", keys: "F3 / Shift+F3", description: "Find next/previous" },
        Hotkey { software: "vscode", keys: "Alt+Enter", description: "Select all occurences of Find match" },
        Hotkey { software: "vscode", keys: "Ctrl+D", description: "Add selection to next Find match" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+D", description: "Move last selection to next Find match" },
        Hotkey { software: "vscode", keys: "Alt+Click", description: "Insert cursor" },
        Hotkey { software: "vscode", keys: "Ctrl+Alt+Up / Down", description: "Insert cursor above / below" },
        Hotkey { software: "vscode", keys: "Ctrl+U", description: "Undo last cursor operation" },
        Hotkey { software: "vscode", keys: "Shift+Alt+I", description: "Insert cursor at end of each line selected" },
        Hotkey { software: "vscode", keys: "Ctrl+I", description: "Select current line" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+L", description: "Select all occurrences of current selection" },
        Hotkey { software: "vscode", keys: "Ctrl+F2", description: "Select all occurrences of current word" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Right", description: "Expand selection" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Left", description: "Shrink selection" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Drag", description: "Column (box) selection" },
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
        Hotkey { software: "vscode", keys: "Ctrl+F4 / Ctrl+W", description: "Close editor" },
        Hotkey { software: "vscode", keys: "Ctrl+K F", description: "Close folder" },
        Hotkey { software: "vscode", keys: "Ctrl+\\", description: "Split editor" },
        Hotkey { software: "vscode", keys: "Ctrl+1 / 2 / 3", description: "Focus into 1st, 2nd or 3rd editor group" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+Left/Right", description: "Focus into previous/next editor group" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+PgUp/PgDn", description: "Move editor left/right" },
        Hotkey { software: "vscode", keys: "Ctrl+K Left / Right", description: "Move active editor group" },
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
        Hotkey { software: "vscode", keys: "F11", description: "Toggle full screen" },
        Hotkey { software: "vscode", keys: "Shift+Alt+0", description: "Toggle editor layout (horizontal/vertical)" },
        Hotkey { software: "vscode", keys: "Ctrl+= / -", description: "Zoom in/out" },
        Hotkey { software: "vscode", keys: "Ctrl+B", description: "Toggle Primary Side Bar visibility" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+E", description: "Show Explorer / Toggle Focus" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+F", description: "Show Search" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+G", description: "Show Source Control" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+D", description: "Show Run and Debug" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+X", description: "Show Extensions" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+H", description: "Replace in files" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+J", description: "Toggle Search details" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+U", description: "Show Output panel" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+V", description: "Open Markdown preview" },
        Hotkey { software: "vscode", keys: "Ctrl+K V", description: "Open Markdown preview to the side" },
        Hotkey { software: "vscode", keys: "Ctrl+K Z", description: "Zen Mode (Esc Esc to exit)" },
        Hotkey { software: "vscode", keys: "F9", description: "Toggle breakpoint" },
        Hotkey { software: "vscode", keys: "F5", description: "Start / Continue" },
        Hotkey { software: "vscode", keys: "Shift+F5", description: "Stop" },
        Hotkey { software: "vscode", keys: "F11 / Shift+F11", description: "Step into / out" },
        Hotkey { software: "vscode", keys: "F10", description: "Step over" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+I", description: "Show hover" },
        Hotkey { software: "vscode", keys: "Ctrl+`", description: "Show integrated terminal" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+`", description: "Create new terminal" },
        Hotkey { software: "vscode", keys: "Ctrl+C", description: "Copy selection" },
        Hotkey { software: "vscode", keys: "Ctrl+V", description: "Paste into active terminal" },
        Hotkey { software: "vscode", keys: "Ctrl+Up / Down", description: "Scroll up / down" },
        Hotkey { software: "vscode", keys: "Shift+PgUp / PgDn", description: "Scroll page up / down" },
        Hotkey { software: "vscode", keys: "Ctrl+Home / End", description: "Scroll to top / bottom" },

        // LazyVim
        Hotkey { software: "lazyvim", keys: "<leader>ff", description: "Find files" },
        Hotkey { software: "lazyvim", keys: "<leader><space>", description: "Find files (Root Dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>sg", description: "Grep / Search text" },
        Hotkey { software: "lazyvim", keys: "<leader>/", description: "Grep (Root Dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>fb", description: "Buffers list" },
        Hotkey { software: "lazyvim", keys: "<leader>,", description: "Switch Buffer" },
        Hotkey { software: "lazyvim", keys: "<leader>fr", description: "Recent files" },
        Hotkey { software: "lazyvim", keys: "<leader>fn", description: "New file" },
        Hotkey { software: "lazyvim", keys: "<leader>e", description: "Toggle Neo-tree (Explorer)" },
        Hotkey { software: "lazyvim", keys: "<leader>fe", description: "Explorer Neo-tree (Root Dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>bb", description: "Switch to other buffer" },
        Hotkey { software: "lazyvim", keys: "<leader>bd", description: "Delete/Close buffer" },
        Hotkey { software: "lazyvim", keys: "<leader>bD", description: "Delete buffer and window" },
        Hotkey { software: "lazyvim", keys: "[b", description: "Previous buffer" },
        Hotkey { software: "lazyvim", keys: "]b", description: "Next buffer" },
        Hotkey { software: "lazyvim", keys: "<leader>bp", description: "Toggle pin buffer" },
        Hotkey { software: "lazyvim", keys: "<leader>bP", description: "Delete non-pinned buffers" },
        Hotkey { software: "lazyvim", keys: "<leader>ww", description: "Other window" },
        Hotkey { software: "lazyvim", keys: "<leader>wd", description: "Delete window" },
        Hotkey { software: "lazyvim", keys: "<leader>w-", description: "Split window below" },
        Hotkey { software: "lazyvim", keys: "<leader>w|", description: "Split window right" },
        Hotkey { software: "lazyvim", keys: "<C-h/j/k/l>", description: "Navigate windows" },
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
        Hotkey { software: "lazyvim", keys: "<leader>gg", description: "Lazygit" },
        Hotkey { software: "lazyvim", keys: "<leader>gG", description: "Lazygit (cwd)" },
        Hotkey { software: "lazyvim", keys: "<leader>l", description: "Lazy plugin manager" },
        Hotkey { software: "lazyvim", keys: "<leader>cm", description: "Mason tool installer" },
        Hotkey { software: "lazyvim", keys: "<leader>qq", description: "Quit all" },
        Hotkey { software: "lazyvim", keys: "<leader>qs", description: "Restore session" },
        Hotkey { software: "lazyvim", keys: "<leader>ql", description: "Restore last session" },
        Hotkey { software: "lazyvim", keys: "<leader>qd", description: "Don't save current session" },
        Hotkey { software: "lazyvim", keys: "<leader>ft", description: "Terminal (Root Dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>fT", description: "Terminal (cwd)" },
        Hotkey { software: "lazyvim", keys: "<c-/>", description: "Terminal (Root Dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>uC", description: "Select colorscheme" },
        Hotkey { software: "lazyvim", keys: "<leader>un", description: "Dismiss all notifications" },

        // Tmux
        Hotkey { software: "tmux", keys: "Ctrl + b", description: "Default prefix key" },
        Hotkey { software: "tmux", keys: "Prefix + c", description: "Create new window" },
        Hotkey { software: "tmux", keys: "Prefix + ,", description: "Rename current window" },
        Hotkey { software: "tmux", keys: "Prefix + &", description: "Kill current window" },
        Hotkey { software: "tmux", keys: "Prefix + n", description: "Next window" },
        Hotkey { software: "tmux", keys: "Prefix + p", description: "Previous window" },
        Hotkey { software: "tmux", keys: "Prefix + 0..9", description: "Select window by number" },
        Hotkey { software: "tmux", keys: "Prefix + w", description: "List windows interactively" },
        Hotkey { software: "tmux", keys: "Prefix + %", description: "Split pane vertically (left/right)" },
        Hotkey { software: "tmux", keys: "Prefix + \"", description: "Split pane horizontally (top/bottom)" },
        Hotkey { software: "tmux", keys: "Prefix + Arrow", description: "Move focus to pane in direction" },
        Hotkey { software: "tmux", keys: "Prefix + o", description: "Rotate through panes" },
        Hotkey { software: "tmux", keys: "Prefix + ;", description: "Toggle between current and previous pane" },
        Hotkey { software: "tmux", keys: "Prefix + x", description: "Kill current pane" },
        Hotkey { software: "tmux", keys: "Prefix + z", description: "Toggle pane zoom (fullscreen)" },
        Hotkey { software: "tmux", keys: "Prefix + {", description: "Swap pane left" },
        Hotkey { software: "tmux", keys: "Prefix + }", description: "Swap pane right" },
        Hotkey { software: "tmux", keys: "Prefix + q", description: "Show pane numbers" },
        Hotkey { software: "tmux", keys: "Prefix + !", description: "Break pane into new window" },
        Hotkey { software: "tmux", keys: "Prefix + [", description: "Enter copy mode" },
        Hotkey { software: "tmux", keys: "Prefix + ]", description: "Paste from buffer" },
        Hotkey { software: "tmux", keys: "Prefix + t", description: "Show large clock" },
        Hotkey { software: "tmux", keys: "Prefix + ?", description: "List all keybindings" },
    ];
}

const SOFTWARES: &[&str] = &["all", "vscode", "lazyvim", "tmux"];

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Search,
    SelectSoftware,
}

#[allow(non_snake_case)]
fn App() -> Element {
    let mut query = use_signal(|| String::new());
    let mut selected_software = use_signal(|| "all".to_string());
    let mut mode = use_signal(|| Mode::Search);
    let mut selected_index = use_signal(|| 0_usize);
    let copied_id = use_signal(|| Option::<usize>::None);
    
    let mut software_query = use_signal(|| String::new());
    let mut sw_selected_index = use_signal(|| 0_usize);

    let filtered_indices = use_memo(move || {
        let matcher = SkimMatcherV2::default();
        let q = query();
        let sw = selected_software();
        let mut matches = Vec::new();

        for (i, hotkey) in ALL_HOTKEYS.iter().enumerate() {
            if sw != "all" && hotkey.software != sw {
                continue;
            }

            if q.is_empty() {
                matches.push((i, 0));
            } else {
                let target = format!("{} {} {}", hotkey.software, hotkey.keys, hotkey.description);
                if let Some(score) = matcher.fuzzy_match(&target, &q) {
                    matches.push((i, score));
                }
            }
        }

        if !q.is_empty() {
            matches.sort_by(|a, b| b.1.cmp(&a.1));
        }
        matches
    });

    let filtered_softwares = use_memo(move || {
        let matcher = SkimMatcherV2::default();
        let sq = software_query();
        let mut matches = Vec::new();
        if sq.is_empty() {
            return SOFTWARES.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        } else {
            for sw in SOFTWARES.iter() {
                if let Some(score) = matcher.fuzzy_match(sw, &sq) {
                    matches.push((sw.to_string(), score));
                }
            }
            matches.sort_by(|a, b| b.1.cmp(&a.1));
            matches.into_iter().map(|(s, _)| s).collect::<Vec<_>>()
        }
    });

    let fi_len = filtered_indices().len();
    if fi_len > 0 && selected_index() >= fi_len {
        selected_index.set(0);
    }
    
    let fs_len = filtered_softwares().len();
    if fs_len > 0 && sw_selected_index() >= fs_len {
        sw_selected_index.set(0);
    }

    let copy_shortcut = move |idx: usize, keys: &'static str| {
        let clean_keys = keys.replace('\\', "\\\\").replace('\'', "\\'");
        let js = format!("navigator.clipboard.writeText('{}');", clean_keys);
        let _ = document::eval(&js);
        let mut cid = copied_id;
        cid.set(Some(idx));
    };

    let onkeydown = move |evt: Event<KeyboardData>| {
        let key = evt.key();
        
        match mode() {
            Mode::Search => {
                match key.to_string().as_str() {
                    "ArrowDown" => {
                        evt.prevent_default();
                        let fi_len = filtered_indices().len();
                        if fi_len > 0 {
                            let curr = selected_index();
                            selected_index.set(if curr >= fi_len - 1 { 0 } else { curr + 1 });
                        }
                    }
                    "ArrowUp" => {
                        evt.prevent_default();
                        let fi_len = filtered_indices().len();
                        if fi_len > 0 {
                            let curr = selected_index();
                            selected_index.set(if curr == 0 { fi_len - 1 } else { curr - 1 });
                        }
                    }
                    "Enter" => {
                        evt.prevent_default();
                        let fi = filtered_indices();
                        if let Some(&(i, _)) = fi.get(selected_index()) {
                            let hotkey = &ALL_HOTKEYS[i];
                            copy_shortcut(i, hotkey.keys);
                        }
                    }
                    "Escape" => {
                        if !query().is_empty() {
                            query.set(String::new());
                            selected_index.set(0);
                        }
                    }
                    ":" if query().is_empty() => {
                        evt.prevent_default();
                        mode.set(Mode::SelectSoftware);
                        software_query.set(String::new());
                        sw_selected_index.set(0);
                    }
                    _ => {}
                }
            }
            Mode::SelectSoftware => {
                match key.to_string().as_str() {
                    "Escape" => {
                        evt.prevent_default();
                        mode.set(Mode::Search);
                    }
                    "ArrowDown" => {
                        evt.prevent_default();
                        let fs_len = filtered_softwares().len();
                        if fs_len > 0 {
                            let curr = sw_selected_index();
                            sw_selected_index.set(if curr >= fs_len - 1 { 0 } else { curr + 1 });
                        }
                    }
                    "ArrowUp" => {
                        evt.prevent_default();
                        let fs_len = filtered_softwares().len();
                        if fs_len > 0 {
                            let curr = sw_selected_index();
                            sw_selected_index.set(if curr == 0 { fs_len - 1 } else { curr - 1 });
                        }
                    }
                    "Tab" => {
                        evt.prevent_default();
                        let fs = filtered_softwares();
                        let curr = sw_selected_index();
                        if let Some(sw) = fs.get(curr) {
                            software_query.set(sw.clone());
                        }
                    }
                    "Enter" => {
                        evt.prevent_default();
                        let fs = filtered_softwares();
                        let curr = sw_selected_index();
                        if let Some(sw) = fs.get(curr) {
                            selected_software.set(sw.clone());
                            mode.set(Mode::Search);
                            query.set(String::new());
                            selected_index.set(0);
                        }
                    }
                    _ => {}
                }
            }
        }
    };

    let fi = filtered_indices();
    let fs = filtered_softwares();
    let total_count = ALL_HOTKEYS.len();
    let matched_count = fi.len();

    rsx! {
        style {
            r#"
            :root {{
                --theme-bg-outer: #181818;
                --theme-bg-page: #1e1e1e;
                --theme-bg-panel: #242424;
                --theme-bg-active: #2a2a2a;
                --theme-border: #363636;
                --theme-border-subtle: #2c2c2c;

                --theme-accent: #d60645;
                --theme-accent-bright: #de1b54;
                --theme-accent-subtle: rgba(214, 6, 69, 0.14);
                --theme-accent-glow: rgba(214, 6, 69, 0.4);

                --theme-success: #10b981;
                --theme-success-subtle: rgba(16, 185, 129, 0.15);
                --theme-danger: #e04848;
                --theme-warning: #f59e0b;
                --theme-info: #38bdf8;

                --text-bright: #bdbcbc;
                --text-normal: #a8a8a8;
                --text-muted: #787878;
                --text-dim: #555555;
                --text-link: #d60645;

                --font-interface: 'Inter', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                --font-monospace: 'JetBrains Mono', monospace;
            }}

            * {{
                box-sizing: border-box;
                margin: 0;
                padding: 0;
            }}

            html, body {{
                width: 100%;
                height: 100%;
                background-color: var(--theme-bg-outer);
                color: var(--text-normal);
                font-family: var(--font-interface);
                font-size: 13px;
                line-height: 1.5;
                overflow: hidden;
                -webkit-font-smoothing: antialiased;
            }}

            /* Custom Sleek Scrollbar */
            ::-webkit-scrollbar {{
                width: 10px;
                height: 10px;
            }}
            ::-webkit-scrollbar-track {{
                background: transparent;
            }}
            ::-webkit-scrollbar-thumb {{
                background-color: var(--theme-border);
                border-radius: 5px;
                border: 2px solid var(--theme-bg-panel);
                background-clip: padding-box;
            }}
            ::-webkit-scrollbar-thumb:hover {{
                background-color: var(--text-muted);
            }}

            /* Material Symbols Sharp */
            .material-symbols-sharp {{
                font-family: 'Material Symbols Sharp';
                font-weight: normal;
                font-style: normal;
                font-size: 18px;
                line-height: 1;
                letter-spacing: normal;
                text-transform: none;
                display: inline-flex;
                align-items: center;
                justify-content: center;
                white-space: nowrap;
                user-select: none;
                vertical-align: middle;
                flex-shrink: 0;
            }}

            /* Outlineless Filter Buttons */
            .filter-tab-btn {{
                background: transparent;
                border: none;
                outline: none;
                color: var(--text-muted);
                font-family: var(--font-interface);
                font-size: 11px;
                font-weight: 500;
                text-transform: uppercase;
                letter-spacing: 0.3px;
                padding: 4px 10px;
                border-radius: 11px;
                cursor: pointer;
                display: inline-flex;
                align-items: center;
                gap: 5px;
                transition: color 0.15s ease, background-color 0.15s ease;
                white-space: nowrap;
                user-select: none;
            }}
            .filter-tab-btn:hover {{
                color: #ffffff;
            }}
            .filter-tab-btn.active {{
                background-color: var(--theme-accent-subtle);
                color: var(--theme-accent);
                font-weight: 600;
            }}

            /* Hotkey List Item */
            .hotkey-row {{
                display: flex;
                align-items: center;
                padding: 8px 16px;
                border-bottom: 1px solid var(--theme-border-subtle);
                transition: background-color 0.12s ease;
                cursor: pointer;
                font-size: 12px;
                position: relative;
                user-select: none;
            }}
            .hotkey-row:hover {{
                background-color: var(--theme-bg-active);
            }}
            .hotkey-row.selected {{
                background-color: var(--theme-bg-panel);
                box-shadow: inset 3px 0 0 var(--theme-accent);
            }}

            /* Keyboard chip */
            .key-chip {{
                font-family: var(--font-monospace);
                font-size: 11px;
                font-weight: 500;
                color: var(--text-bright);
                background-color: var(--theme-bg-outer);
                border: 1px solid var(--theme-border);
                border-radius: 3px;
                padding: 2px 7px;
                display: inline-flex;
                align-items: center;
                transition: border-color 0.15s ease, color 0.15s ease;
            }}
            .hotkey-row.selected .key-chip {{
                border-color: var(--theme-accent);
                color: var(--theme-accent-bright);
            }}

            /* Action icon button */
            .icon-btn {{
                background: transparent;
                border: none;
                outline: none;
                color: var(--text-muted);
                cursor: pointer;
                padding: 4px;
                border-radius: 3px;
                display: inline-flex;
                align-items: center;
                justify-content: center;
                transition: color 0.15s ease;
            }}
            .icon-btn:hover {{
                color: #ffffff;
            }}

            .cursor {{
                display: inline-block;
                width: 7px;
                height: 1.1em;
                background-color: var(--theme-accent);
                vertical-align: text-bottom;
                animation: blink 1s step-end infinite;
            }}
            @keyframes blink {{ 50% {{ opacity: 0; }} }}
            "#
        }

        div {
            tabindex: 0,
            autofocus: true,
            onkeydown: onkeydown,
            style: "width: 100%; height: 100%; display: flex; flex-direction: column; outline: none; background-color: var(--theme-bg-outer);",

            // 1. BRAND HEADER BAR (Section 2 of NullDark Spec)
            header {
                style: "height: 44px; background-color: var(--theme-bg-outer); border-bottom: 1px solid var(--theme-border); padding: 0 16px; display: flex; align-items: center; justify-content: space-between; flex-shrink: 0; user-select: none;",
                
                // Left Brand Anchor
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    img {
                        src: "icon.svg",
                        style: "height: 26px; width: auto; object-fit: contain; display: block; border: none; background: transparent;",
                        alt: "NullDark Mark",
                    }
                    div {
                        style: "display: flex; align-items: baseline;",
                        span { style: "font-size: 13px; font-weight: 700; color: var(--text-muted); letter-spacing: 0.2px;", "NULL" }
                        span { style: "font-size: 13px; font-weight: 700; color: var(--text-bright); letter-spacing: 0.2px;", "HOTKEYS" }
                    }
                    span { style: "color: var(--theme-border); font-size: 13px; font-weight: 300; margin: 0 4px;", "|" }
                    span { style: "font-size: 11px; font-weight: 500; text-transform: uppercase; letter-spacing: 0.3px; color: var(--text-muted);", "KEYBOARD SHORTCUT WORKSPACE" }
                }

                // Right Status Indicators
                div {
                    style: "display: flex; align-items: center; gap: 14px;",
                    
                    // Boundary-Free Status Indicator
                    div {
                        style: "display: flex; align-items: center; gap: 6px; font-size: 11px; color: var(--text-normal); letter-spacing: 0.2px;",
                        span { style: "width: 7px; height: 7px; border-radius: 50%; background-color: var(--theme-success); box-shadow: 0 0 6px rgba(16, 185, 129, 0.4); display: inline-block;" }
                        span { "OPERATIONAL" }
                    }

                    // Tinted Status Pill
                    div {
                        style: "height: 22px; border-radius: 11px; padding: 0 10px; background: var(--theme-accent-subtle); color: var(--theme-accent); font-size: 11px; font-weight: 600; display: inline-flex; align-items: center; border: none;",
                        "{matched_count} SHORTCUTS"
                    }
                }
            }

            // 2. SEARCH & FILTER TOOLBAR
            div {
                style: "background-color: var(--theme-bg-outer); border-bottom: 1px solid var(--theme-border); padding: 8px 16px; display: flex; align-items: center; justify-content: space-between; gap: 16px; flex-shrink: 0;",
                
                // Search Input Box
                div {
                    style: "position: relative; flex: 1; max-width: 480px; display: flex; align-items: center;",
                    span {
                        class: "material-symbols-sharp",
                        style: "position: absolute; left: 10px; font-size: 18px; color: var(--text-muted); pointer-events: none;",
                        "search"
                    }
                    input {
                        r#type: "text",
                        autofocus: true,
                        style: "background: var(--theme-bg-panel); border: 1px solid var(--theme-border); border-radius: 4px; padding: 6px 36px 6px 34px; color: var(--text-bright); font-size: 12px; font-family: var(--font-interface); width: 100%; outline: none; transition: border-color 0.15s, box-shadow 0.15s;",
                        placeholder: "Search shortcuts (e.g. format, git, pane, buffer)...",
                        value: "{query()}",
                        oninput: move |evt| {
                            query.set(evt.value());
                            selected_index.set(0);
                        },
                    }
                    if !query().is_empty() {
                        button {
                            class: "icon-btn",
                            style: "position: absolute; right: 6px;",
                            onclick: move |_| {
                                query.set(String::new());
                            },
                            span { class: "material-symbols-sharp", style: "font-size: 16px;", "close" }
                        }
                    } else {
                        span {
                            style: "position: absolute; right: 8px; font-size: 10px; color: var(--text-dim); background: var(--theme-bg-outer); border: 1px solid var(--theme-border); border-radius: 3px; padding: 1px 5px; font-family: var(--font-monospace); pointer-events: none;",
                            ":"
                        }
                    }
                }

                // Pipe-separated Software Pills
                div {
                    style: "display: flex; align-items: center; gap: 4px;",
                    
                    button {
                        class: if selected_software() == "all" { "filter-tab-btn active" } else { "filter-tab-btn" },
                        onclick: move |_| {
                            selected_software.set("all".to_string());
                            selected_index.set(0);
                        },
                        "ALL"
                    }
                    span { style: "color: var(--theme-border); font-size: 12px; margin: 0 2px;", "|" }
                    button {
                        class: if selected_software() == "vscode" { "filter-tab-btn active" } else { "filter-tab-btn" },
                        onclick: move |_| {
                            selected_software.set("vscode".to_string());
                            selected_index.set(0);
                        },
                        "VS CODE"
                    }
                    span { style: "color: var(--theme-border); font-size: 12px; margin: 0 2px;", "|" }
                    button {
                        class: if selected_software() == "lazyvim" { "filter-tab-btn active" } else { "filter-tab-btn" },
                        onclick: move |_| {
                            selected_software.set("lazyvim".to_string());
                            selected_index.set(0);
                        },
                        "LAZYVIM"
                    }
                    span { style: "color: var(--theme-border); font-size: 12px; margin: 0 2px;", "|" }
                    button {
                        class: if selected_software() == "tmux" { "filter-tab-btn active" } else { "filter-tab-btn" },
                        onclick: move |_| {
                            selected_software.set("tmux".to_string());
                            selected_index.set(0);
                        },
                        "TMUX"
                    }
                    span { style: "color: var(--theme-border); font-size: 12px; margin: 0 2px;", "|" }
                    button {
                        class: "filter-tab-btn",
                        onclick: move |_| {
                            mode.set(Mode::SelectSoftware);
                            software_query.set(String::new());
                            sw_selected_index.set(0);
                        },
                        span { class: "material-symbols-sharp", style: "font-size: 14px; margin-right: 2px;", "tune" }
                        "SWITCH (:)"
                    }
                }
            }

            // 3. MAIN WORKSPACE CANVAS
            div {
                style: "background-color: var(--theme-bg-page); flex: 1; display: flex; flex-direction: column; overflow: hidden; position: relative;",

                // Table Header
                div {
                    style: "background-color: var(--theme-bg-panel); height: 32px; display: flex; align-items: center; padding: 0 16px; border-bottom: 1px solid var(--theme-border); font-size: 11px; font-weight: 600; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.5px; user-select: none; flex-shrink: 0;",
                    div { style: "width: 100px; flex-shrink: 0;", "SOFTWARE" }
                    div { style: "width: 270px; flex-shrink: 0;", "KEY COMBINATION" }
                    div { style: "flex: 1;", "ACTION / DESCRIPTION" }
                    div { style: "width: 60px; text-align: right;", "COPY" }
                }

                // Table Rows
                div {
                    style: "flex: 1; overflow-y: auto; overflow-x: hidden;",

                    if fi.is_empty() {
                        div {
                            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; gap: 10px; color: var(--text-muted); user-select: none;",
                            span { class: "material-symbols-sharp", style: "font-size: 36px; color: var(--text-dim);", "search_off" }
                            div { style: "font-size: 13px; color: var(--text-bright); font-weight: 500;", "No shortcuts found" }
                            div { style: "font-size: 11px; color: var(--text-muted);", "Try another keyword or press ':' to change software" }
                        }
                    } else {
                        {
                            fi.iter().enumerate().map(|(idx, &(i, _))| {
                                let hotkey = &ALL_HOTKEYS[i];
                                let is_selected = mode() == Mode::Search && idx == selected_index();
                                let is_copied = copied_id() == Some(i);
                                let row_class = if is_selected { "hotkey-row selected" } else { "hotkey-row" };
                                let keys_text = hotkey.keys;
                                
                                rsx! {
                                    div {
                                        key: "{i}",
                                        class: "{row_class}",
                                        onclick: move |_| {
                                            selected_index.set(idx);
                                            copy_shortcut(i, keys_text);
                                        },

                                        // Software Tag
                                        div {
                                            style: "width: 100px; flex-shrink: 0;",
                                            span {
                                                style: "font-size: 10px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.4px; color: var(--text-muted); background: var(--theme-bg-outer); padding: 2px 6px; border-radius: 3px;",
                                                "{hotkey.software}"
                                            }
                                        }

                                        // Key Combination Chip
                                        div {
                                            style: "width: 270px; flex-shrink: 0; padding-right: 12px; display: flex; align-items: center; gap: 4px;",
                                            span {
                                                class: "key-chip",
                                                "{hotkey.keys}"
                                            }
                                        }

                                        // Description
                                        div {
                                            style: "flex: 1; color: var(--text-normal); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 12px;",
                                            "{hotkey.description}"
                                        }

                                        // Copy Action
                                        div {
                                            style: "width: 60px; text-align: right;",
                                            if is_copied {
                                                span {
                                                    style: "font-size: 11px; color: var(--theme-success); font-weight: 500; display: inline-flex; align-items: center; gap: 2px;",
                                                    span { class: "material-symbols-sharp", style: "font-size: 14px;", "check" }
                                                    "COPIED"
                                                }
                                            } else {
                                                button {
                                                    class: "icon-btn",
                                                    title: "Copy shortcut",
                                                    onclick: move |evt| {
                                                        evt.stop_propagation();
                                                        copy_shortcut(i, keys_text);
                                                    },
                                                    span { class: "material-symbols-sharp", style: "font-size: 16px;", "content_copy" }
                                                }
                                            }
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            }

            // 4. SOFTWARE SELECTION MODAL
            if mode() == Mode::SelectSoftware {
                div {
                    style: "position: fixed; inset: 0; background: rgba(0, 0, 0, 0.7); backdrop-filter: blur(4px); display: flex; align-items: center; justify-content: center; z-index: 1000; user-select: none;",
                    onclick: move |_| {
                        mode.set(Mode::Search);
                    },
                    div {
                        style: "width: 440px; background-color: var(--theme-bg-panel); border-radius: 6px; box-shadow: 0 20px 50px rgba(0, 0, 0, 0.8); overflow: hidden; display: flex; flex-direction: column; border: none;",
                        onclick: move |evt| {
                            evt.stop_propagation();
                        },

                        // Modal Header
                        div {
                            style: "padding: 12px 16px; border-bottom: 1px solid var(--theme-border); display: flex; align-items: center; justify-content: space-between;",
                            div {
                                style: "display: flex; align-items: center; gap: 6px;",
                                span { class: "material-symbols-sharp", style: "font-size: 16px; color: var(--theme-accent);", "filter_list" }
                                span { style: "font-size: 13px; font-weight: 600; color: var(--text-bright);", "Filter by Software" }
                            }
                            button {
                                class: "icon-btn",
                                onclick: move |_| mode.set(Mode::Search),
                                span { class: "material-symbols-sharp", style: "font-size: 16px;", "close" }
                            }
                        }

                        // Search Input
                        div {
                            style: "padding: 12px 16px; border-bottom: 1px solid var(--theme-border-subtle);",
                            div {
                                style: "position: relative; display: flex; align-items: center;",
                                span {
                                    style: "position: absolute; left: 10px; font-size: 12px; color: var(--theme-accent); font-family: var(--font-monospace);",
                                    ":"
                                }
                                input {
                                    r#type: "text",
                                    autofocus: true,
                                    style: "width: 100%; background: var(--theme-bg-outer); border: 1px solid var(--theme-border); border-radius: 4px; padding: 6px 12px 6px 24px; color: var(--text-bright); font-size: 12px; outline: none;",
                                    placeholder: "Type software name (Tab to complete, Enter to select)...",
                                    value: "{software_query()}",
                                    oninput: move |evt| {
                                        software_query.set(evt.value());
                                        sw_selected_index.set(0);
                                    },
                                }
                            }
                        }

                        // Software List
                        div {
                            style: "max-height: 220px; overflow-y: auto; padding: 4px 0;",
                            {
                                fs.iter().enumerate().map(|(idx, sw)| {
                                    let sw_str = sw.clone();
                                    let is_active = idx == sw_selected_index();
                                    let is_current = selected_software() == *sw;
                                    let item_style = if is_active {
                                        "padding: 8px 16px; display: flex; align-items: center; justify-content: space-between; font-size: 12px; background: var(--theme-bg-active); color: var(--theme-accent); font-weight: 600; cursor: pointer;"
                                    } else {
                                        "padding: 8px 16px; display: flex; align-items: center; justify-content: space-between; font-size: 12px; color: var(--text-normal); cursor: pointer;"
                                    };

                                    rsx! {
                                        div {
                                            key: "{sw}",
                                            style: "{item_style}",
                                            onclick: move |_| {
                                                selected_software.set(sw_str.clone());
                                                mode.set(Mode::Search);
                                                query.set(String::new());
                                                selected_index.set(0);
                                            },
                                            div {
                                                style: "display: flex; align-items: center; gap: 8px;",
                                                if is_current {
                                                    span { class: "material-symbols-sharp", style: "font-size: 16px; color: var(--theme-accent);", "check" }
                                                } else {
                                                    span { style: "width: 16px; display: inline-block;" }
                                                }
                                                span { style: "text-transform: uppercase;", "{sw}" }
                                            }
                                            if is_current {
                                                span { style: "font-size: 10px; color: var(--text-muted);", "ACTIVE" }
                                            }
                                        }
                                    }
                                })
                            }
                        }
                    }
                }
            }

            // 5. STATUS FOOTER BAR (Section 6 of NullDark Spec)
            footer {
                style: "height: 28px; background-color: var(--theme-bg-outer); border-top: 1px solid var(--theme-border); padding: 0 16px; display: flex; align-items: center; justify-content: space-between; font-size: 11px; color: var(--text-muted); user-select: none; flex-shrink: 0;",
                
                // Keyboard helpers
                div {
                    style: "display: flex; align-items: center; gap: 14px;",
                    span {
                        span { style: "font-family: var(--font-monospace); color: var(--text-bright);", "↑↓ " }
                        "Navigate"
                    }
                    span {
                        span { style: "font-family: var(--font-monospace); color: var(--text-bright);", "Enter " }
                        "Copy Shortcut"
                    }
                    span {
                        span { style: "font-family: var(--font-monospace); color: var(--text-bright);", ": " }
                        "Select Software"
                    }
                    span {
                        span { style: "font-family: var(--font-monospace); color: var(--text-bright);", "Esc " }
                        "Clear Search"
                    }
                }

                // Items Count
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    span { "Category: " }
                    span { style: "color: var(--theme-accent); font-weight: 600; text-transform: uppercase;", "{selected_software()}" }
                    span { style: "color: var(--theme-border); margin: 0 2px;", "•" }
                    span { "Showing {matched_count} of {total_count}" }
                }
            }
        }
    }
}
