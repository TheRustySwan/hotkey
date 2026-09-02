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
        Hotkey { software: "vscode", keys: "Alt+C / R / W", description: "Toggle case-sensitive / regex / whole word" },
        Hotkey { software: "vscode", keys: "Alt+Click", description: "Insert cursor" },
        Hotkey { software: "vscode", keys: "Ctrl+Alt+Up/Down", description: "Insert cursor above / below" },
        Hotkey { software: "vscode", keys: "Ctrl+U", description: "Undo last cursor operation" },
        Hotkey { software: "vscode", keys: "Shift+Alt+I", description: "Insert cursor at end of each line selected" },
        Hotkey { software: "vscode", keys: "Ctrl+L", description: "Select current line" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+L", description: "Select all occurrences of current selection" },
        Hotkey { software: "vscode", keys: "Ctrl+F2", description: "Select all occurrences of current word" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Right", description: "Expand selection" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Left", description: "Shrink selection" },
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
        Hotkey { software: "vscode", keys: "Ctrl+F4, Ctrl+W", description: "Close editor" },
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
        Hotkey { software: "vscode", keys: "F9", description: "Toggle breakpoint" },
        Hotkey { software: "vscode", keys: "F5", description: "Start/Continue" },
        Hotkey { software: "vscode", keys: "Shift+F5", description: "Stop" },
        Hotkey { software: "vscode", keys: "F11 / Shift+F11", description: "Step into/out" },
        Hotkey { software: "vscode", keys: "F10", description: "Step over" },

        // LazyVim & Neovim Core
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
        Hotkey { software: "lazyvim", keys: "/", description: "Search forward" },
        Hotkey { software: "lazyvim", keys: "?", description: "Search backward" },
        Hotkey { software: "lazyvim", keys: "n / N", description: "Next / previous search match" },
        Hotkey { software: "lazyvim", keys: "* / #", description: "Search forward / backward for word under cursor" },
        Hotkey { software: "lazyvim", keys: "v", description: "Start character-wise visual mode" },
        Hotkey { software: "lazyvim", keys: "V", description: "Start line-wise visual mode" },
        Hotkey { software: "lazyvim", keys: "Ctrl+v", description: "Start block-wise visual mode" },
        Hotkey { software: "lazyvim", keys: "> / <", description: "Indent / outdent selected lines" },
        Hotkey { software: "lazyvim", keys: "Ctrl+w s / :sp", description: "Split window horizontally" },
        Hotkey { software: "lazyvim", keys: "Ctrl+w v / :vsp", description: "Split window vertically" },
        Hotkey { software: "lazyvim", keys: "Ctrl+w q", description: "Close window" },
        Hotkey { software: "lazyvim", keys: "Ctrl+w =", description: "Make all windows equal size" },
        Hotkey { software: "lazyvim", keys: "gt / gT", description: "Next / previous tab" },
        Hotkey { software: "lazyvim", keys: "<leader>ff", description: "Find files (Telescope)" },
        Hotkey { software: "lazyvim", keys: "<leader>fF", description: "Find files in cwd (Telescope)" },
        Hotkey { software: "lazyvim", keys: "<leader>fg", description: "Find git files" },
        Hotkey { software: "lazyvim", keys: "<leader>fr", description: "Recent files" },
        Hotkey { software: "lazyvim", keys: "<leader>fR", description: "Recent files (cwd)" },
        Hotkey { software: "lazyvim", keys: "<leader>fc", description: "Find config file" },
        Hotkey { software: "lazyvim", keys: "<leader>fb", description: "Find buffers" },
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
        Hotkey { software: "lazyvim", keys: "<leader>e", description: "Toggle Neo-tree (root dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>E", description: "Toggle Neo-tree (cwd)" },
        Hotkey { software: "lazyvim", keys: "<leader>bd", description: "Delete buffer" },
        Hotkey { software: "lazyvim", keys: "<leader>bD", description: "Delete buffer and window" },
        Hotkey { software: "lazyvim", keys: "<leader>bo", description: "Delete other buffers" },
        Hotkey { software: "lazyvim", keys: "<leader>bl", description: "Delete buffers to the left" },
        Hotkey { software: "lazyvim", keys: "<leader>br", description: "Delete buffers to the right" },
        Hotkey { software: "lazyvim", keys: "<leader>bb", description: "Switch to other buffer" },
        Hotkey { software: "lazyvim", keys: "<S-h> / [b", description: "Previous buffer" },
        Hotkey { software: "lazyvim", keys: "<S-l> / ]b", description: "Next buffer" },
        Hotkey { software: "lazyvim", keys: "<leader>w-", description: "Split window below" },
        Hotkey { software: "lazyvim", keys: "<leader>w|", description: "Split window right" },
        Hotkey { software: "lazyvim", keys: "<leader>wd", description: "Delete window" },
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
        Hotkey { software: "lazyvim", keys: "<leader>cm", description: "Mason (package manager)" },
        Hotkey { software: "lazyvim", keys: "<leader>qq", description: "Quit all" },
        Hotkey { software: "lazyvim", keys: "<leader>uh", description: "Toggle Inlay Hints" },

        // tmux
        Hotkey { software: "tmux", keys: "Prefix + c", description: "Create new window" },
        Hotkey { software: "tmux", keys: "Prefix + d", description: "Detach session" },
        Hotkey { software: "tmux", keys: "Prefix + s", description: "List sessions" },
        Hotkey { software: "tmux", keys: "Prefix + w", description: "List windows" },
        Hotkey { software: "tmux", keys: "Prefix + n", description: "Next window" },
        Hotkey { software: "tmux", keys: "Prefix + p", description: "Previous window" },
        Hotkey { software: "tmux", keys: "Prefix + &", description: "Kill current window" },
        Hotkey { software: "tmux", keys: "Prefix + ,", description: "Rename current window" },
        Hotkey { software: "tmux", keys: "Prefix + $", description: "Rename current session" },
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

fn App() -> Element {
    let mut query = use_signal(|| String::new());
    let mut selected_software = use_signal(|| "all".to_string());
    let mut mode = use_signal(|| Mode::Search);
    let mut selected_index = use_signal(|| 0_usize);
    
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
                let target = format!("{} {}", hotkey.keys, hotkey.description);
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

    let onkeydown = move |evt: Event<KeyboardData>| {
        let key = evt.key();
        
        match mode() {
            Mode::Search => {
                match key.to_string().as_str() {
                    "ArrowDown" => {
                        let fi_len = filtered_indices().len();
                        if fi_len > 0 {
                            let curr = selected_index();
                            selected_index.set(if curr >= fi_len - 1 { 0 } else { curr + 1 });
                        }
                    }
                    "ArrowUp" => {
                        let fi_len = filtered_indices().len();
                        if fi_len > 0 {
                            let curr = selected_index();
                            selected_index.set(if curr == 0 { fi_len - 1 } else { curr - 1 });
                        }
                    }
                    "Backspace" => {
                        let mut q = query();
                        if !q.is_empty() {
                            q.pop();
                            query.set(q);
                        }
                    }
                    ":" if query().is_empty() => {
                        mode.set(Mode::SelectSoftware);
                        software_query.set(String::new());
                        sw_selected_index.set(0);
                    }
                    char_str if char_str.len() == 1 => {
                        let mut q = query();
                        q.push_str(char_str);
                        query.set(q);
                    }
                    _ => {}
                }
            }
            Mode::SelectSoftware => {
                match key.to_string().as_str() {
                    "Escape" => mode.set(Mode::Search),
                    "ArrowDown" => {
                        let fs_len = filtered_softwares().len();
                        if fs_len > 0 {
                            let curr = sw_selected_index();
                            sw_selected_index.set(if curr >= fs_len - 1 { 0 } else { curr + 1 });
                        }
                    }
                    "ArrowUp" => {
                        let fs_len = filtered_softwares().len();
                        if fs_len > 0 {
                            let curr = sw_selected_index();
                            sw_selected_index.set(if curr == 0 { fs_len - 1 } else { curr - 1 });
                        }
                    }
                    "Tab" => {
                        let fs = filtered_softwares();
                        let curr = sw_selected_index();
                        if let Some(sw) = fs.get(curr) {
                            software_query.set(sw.clone());
                        }
                    }
                    "Enter" => {
                        let fs = filtered_softwares();
                        let curr = sw_selected_index();
                        if let Some(sw) = fs.get(curr) {
                            selected_software.set(sw.clone());
                            mode.set(Mode::Search);
                            query.set(String::new());
                        }
                    }
                    "Backspace" => {
                        let mut sq = software_query();
                        if !sq.is_empty() {
                            sq.pop();
                            software_query.set(sq);
                        }
                    }
                    char_str if char_str.len() == 1 => {
                        let mut sq = software_query();
                        sq.push_str(char_str);
                        software_query.set(sq);
                    }
                    _ => {}
                }
            }
        }
    };

    let fi = filtered_indices();
    let fs = filtered_softwares();

    rsx! {
        style {
            r#"
            body {{
                background-color: #161616;
                color: #f2f4f8;
                font-family: 'JetBrainsMono NF', 'JetBrainsMono Nerd Font', 'JetBrains Mono', monospace;
                margin: 0;
                padding: 20px;
                display: flex;
                flex-direction: column;
                height: 100vh;
                box-sizing: border-box;
                overflow: hidden;
            }}
            .panel {{
                border: 1px solid #393939;
                padding: 10px;
                margin-top: 12px;
                margin-bottom: 10px;
                position: relative;
            }}
            .results-panel {{
                border: 1px solid #393939;
                position: relative;
                margin-top: 12px;
                padding: 10px;
                flex-grow: 1;
                display: flex;
                flex-direction: column;
                min-height: 0;
            }}
            .panel-title {{
                position: absolute;
                top: -10px;
                left: 10px;
                background: #161616;
                padding: 0 5px;
                color: #ee5396;
            }}
            .input-box {{
                color: #33b1ff;
                outline: none;
            }}
            .list-container {{
                flex-grow: 1;
                overflow-y: auto;
                min-height: 0;
            }}
            .list-item {{
                padding: 2px 0;
                display: flex;
            }}
            .list-item.selected {{
                background-color: #262626;
                font-weight: bold;
            }}
            .prefix-tag {{
                white-space: pre;
                color: #ee5396;
            }}
            .sw-tag {{
                color: #be95ff;
                width: 120px;
                flex-shrink: 0;
            }}
            .keys-tag {{
                color: #3ddbd9;
                font-weight: bold;
                width: 270px;
                flex-shrink: 0;
            }}
            .desc-tag {{
                color: #f2f4f8;
            }}
            .popup-overlay {{
                position: absolute;
                top: 0; left: 0; right: 0; bottom: 0;
                display: flex;
                align-items: center;
                justify-content: center;
            }}
            .popup {{
                background: #161616;
                border: 1px solid #ee5396;
                width: 400px;
                position: relative;
                padding: 10px;
                display: flex;
                flex-direction: column;
            }}
            .popup-input {{
                border-bottom: 1px solid #393939;
                color: #42be65;
                padding-bottom: 5px;
                margin-bottom: 10px;
            }}
            .cursor {{
                display: inline-block;
                width: 8px;
                height: 1em;
                background-color: currentColor;
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
            style: "flex-grow: 1; display: flex; flex-direction: column; outline: none;",
            
            div { class: "panel",
                div { class: "panel-title", "Search Hotkeys [{selected_software()}] (Type to search, ':' for software, Up/Down to navigate, Esc to quit)" }
                div { class: "input-box",
                    "> {query()}"
                    if mode() == Mode::Search { span { class: "cursor" } }
                }
            }

            div { class: "results-panel",
                div { class: "panel-title", "Results" }
                div { class: "list-container",
                    {
                        fi.iter().enumerate().map(|(idx, &(i, _))| {
                            let hotkey = &ALL_HOTKEYS[i];
                            let is_selected = if mode() == Mode::Search && idx == selected_index() { "selected" } else { "" };
                            let prefix = if is_selected == "selected" { ">> " } else { "   " };
                            
                            rsx! {
                                div {
                                    class: "list-item {is_selected}",
                                    key: "{i}",
                                    span { class: "prefix-tag", "{prefix}" }
                                    span { class: "sw-tag", "[{hotkey.software}]" }
                                    span { class: "keys-tag", "{hotkey.keys}" }
                                    span { class: "desc-tag", "{hotkey.description}" }
                                }
                            }
                        })
                    }
                }
            }

            if mode() == Mode::SelectSoftware {
                div { class: "popup-overlay",
                    div { class: "popup",
                        div { class: "panel-title", "Select Software (Tab to autocomplete, Enter to select)" }
                        div { class: "popup-input",
                            ":{software_query()}"
                            span { class: "cursor" }
                        }
                        div { style: "max-height: 200px; overflow-y: auto;",
                            {
                                fs.iter().enumerate().map(|(idx, sw)| {
                                    let is_selected = if idx == sw_selected_index() { "selected" } else { "" };
                                    let prefix = if is_selected == "selected" { "> " } else { "  " };
                                    rsx! {
                                        div {
                                            class: "list-item {is_selected}",
                                            key: "{sw}",
                                            span { class: "prefix-tag", "{prefix}" }
                                            span { "{sw}" }
                                        }
                                    }
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}
