//! Shell startup hooks and initialization scripts for bash, zsh, and fish.

pub fn shell_init_script(shell: &str) -> Option<&'static str> {
    match shell.to_lowercase().as_str() {
        "bash" => Some(
            r#"# RustFetch — Shell startup integration
if [[ $- == *i* ]]; then
    _rustfetch_init_hist=""
    _rustfetch_last_size="$(stty size 2>/dev/null)"
    _rustfetch_prompt_cmd() {
        if [ -z "$_rustfetch_init_hist" ]; then
            _rustfetch_init_hist=$HISTCMD
        elif [ "$HISTCMD" -gt "$_rustfetch_init_hist" ]; then
            trap - WINCH
        fi
    }
    PROMPT_COMMAND+=(_rustfetch_prompt_cmd)
    _rustfetch_on_resize() {
        if [ -z "$_rustfetch_init_hist" ] || [ "$HISTCMD" -le "$_rustfetch_init_hist" ]; then
            if [ -z "$READLINE_LINE" ]; then
                local cur_size
                while :; do
                    cur_size="$(stty size 2>/dev/null)"
                    if [ -z "$cur_size" ] || [ "$cur_size" = "$_rustfetch_last_size" ]; then
                        break
                    fi
                    _rustfetch_last_size="$cur_size"
                    rustfetch --keep-logo --no-update-check
                done
                local _rf_ps1="${PS1@P}"
                printf '%s' "${_rf_ps1//[$'\001\002']/}"
            fi
        else
            trap - WINCH
        fi
    }
    trap '_rustfetch_on_resize' WINCH
    rustfetch
fi
"#,
        ),
        "zsh" => Some(
            r#"# RustFetch — Shell startup integration
if [[ -o interactive ]]; then
    _rustfetch_startup=1
    _rustfetch_last_size="$(stty size 2>/dev/null)"
    _rustfetch_preexec() {
        _rustfetch_startup=0
    }
    autoload -Uz add-zsh-hook 2>/dev/null && add-zsh-hook preexec _rustfetch_preexec 2>/dev/null
    TRAPWINCH() {
        if [[ "$_rustfetch_startup" == "1" && -z "$BUFFER" ]]; then
            local cur_size
            while true; do
                cur_size="$(stty size 2>/dev/null)"
                if [[ -z "$cur_size" || "$cur_size" == "$_rustfetch_last_size" ]]; then
                    break
                fi
                _rustfetch_last_size="$cur_size"
                rustfetch --keep-logo --no-update-check
            done
            zle && zle reset-prompt 2>/dev/null
        fi
    }
    rustfetch
fi
"#,
        ),
        "fish" => Some(
            r#"# RustFetch — Shell startup integration
if status is-interactive
    set -g _rustfetch_startup 1
    set -g _rustfetch_last_size (stty size 2>/dev/null)
    function _rustfetch_preexec --on-event fish_preexec
        set -g _rustfetch_startup 0
    end
    function _rustfetch_on_resize --on-signal WINCH
        if test "$_rustfetch_startup" = "1" -a -z (commandline)
            set -l cur_size (stty size 2>/dev/null)
            if test -n "$cur_size" -a "$cur_size" != "$_rustfetch_last_size"
                set -g _rustfetch_last_size "$cur_size"
                rustfetch --keep-logo --no-update-check
                commandline -f repaint 2>/dev/null
            end
        end
    end
    rustfetch
end
"#,
        ),
        _ => None,
    }
}

pub fn get_shell_target_file(home: &std::path::Path) -> (&'static str, std::path::PathBuf) {
    let shell_path = std::env::var("SHELL").unwrap_or_default();
    let is_zsh = shell_path.ends_with("/zsh") || shell_path == "zsh";
    let is_fish = shell_path.ends_with("/fish") || shell_path == "fish";

    if is_zsh {
        ("zsh", home.join(".zshrc"))
    } else if is_fish {
        ("fish", home.join(".config/fish/config.fish"))
    } else {
        ("bash", home.join(".bashrc"))
    }
}

pub fn strip_rustfetch_hook(content: &str) -> String {
    let mut result = Vec::new();
    let mut in_rustfetch_block = false;
    let mut block_depth = 0;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("# RustFetch") {
            in_rustfetch_block = true;
            block_depth = 0;
            continue;
        }

        if in_rustfetch_block {
            if trimmed.starts_with("if ") || trimmed.starts_with("function ") {
                block_depth += 1;
            }
            if trimmed == "fi" || trimmed == "end" || (block_depth > 0 && trimmed.ends_with("; fi"))
            {
                if block_depth > 0 {
                    block_depth -= 1;
                }
                if block_depth == 0 {
                    in_rustfetch_block = false;
                    continue;
                }
            }
            continue;
        }

        if trimmed.contains("_rustfetch")
            || trimmed.contains("TRAPWINCH")
            || trimmed == "rustfetch"
            || trimmed == "rfetch"
        {
            continue;
        }

        result.push(line);
    }

    let mut cleaned_lines = Vec::new();
    let mut prev_empty = false;
    for line in result {
        let is_empty = line.trim().is_empty();
        if is_empty && prev_empty {
            continue;
        }
        cleaned_lines.push(line);
        prev_empty = is_empty;
    }

    let mut output = cleaned_lines.join("\n");
    if !output.is_empty() && !output.ends_with('\n') {
        output.push('\n');
    }
    output
}

pub fn install_shell_hook() {
    let home = match std::env::var("HOME") {
        Ok(h) => std::path::PathBuf::from(h),
        Err(_) => {
            eprintln!("Error: HOME environment variable not set.");
            return;
        }
    };

    let (shell_name, target_file) = get_shell_target_file(&home);

    let script = match shell_init_script(shell_name) {
        Some(s) => s,
        None => return,
    };

    let existing = std::fs::read_to_string(&target_file).unwrap_or_default();
    let cleaned = strip_rustfetch_hook(&existing);

    let mut new_content = cleaned;
    if !new_content.is_empty() && !new_content.ends_with('\n') {
        new_content.push('\n');
    }
    new_content.push_str(script);

    if let Some(parent) = target_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    match std::fs::write(&target_file, new_content) {
        Ok(()) => println!(
            "Successfully installed RustFetch startup hook to {}",
            target_file.display()
        ),
        Err(e) => eprintln!("Failed to write to {}: {e}", target_file.display()),
    }
}

pub fn uninstall_shell_hook() {
    let home = match std::env::var("HOME") {
        Ok(h) => std::path::PathBuf::from(h),
        Err(_) => {
            eprintln!("Error: HOME environment variable not set.");
            return;
        }
    };

    let (_, target_file) = get_shell_target_file(&home);
    if !target_file.exists() {
        println!("No shell configuration found at {}", target_file.display());
        return;
    }

    let existing = std::fs::read_to_string(&target_file).unwrap_or_default();
    let cleaned = strip_rustfetch_hook(&existing);

    match std::fs::write(&target_file, cleaned) {
        Ok(()) => println!(
            "Successfully removed RustFetch hook from {}",
            target_file.display()
        ),
        Err(e) => eprintln!("Failed to write to {}: {e}", target_file.display()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_rustfetch_hook_old_format() {
        let input = r#"# .bashrc
export PATH="$HOME/.local/bin:$PATH"

# RustFetch — Automatic responsive redraw for tiling WMs (MangoWM, Hyprland, Sway, etc.)
if [[ $- == *i* ]]; then
    _rustfetch_last_size=""
    _rustfetch_on_resize() {
        local cur_size="$(stty size 2>/dev/null)"
        if [ -n "$cur_size" ] && [ "$cur_size" != "$_rustfetch_last_size" ]; then
            _rustfetch_last_size="$cur_size"
            if [ -z "$READLINE_LINE" ]; then
                clear
                rustfetch
            fi
        fi
    }
    trap '_rustfetch_on_resize' WINCH
    _rustfetch_last_size="$(stty size 2>/dev/null)"
    rustfetch
fi

alias ll='ls -la'
"#;

        let cleaned = strip_rustfetch_hook(input);
        assert!(!cleaned.contains("_rustfetch"));
        assert!(!cleaned.contains("WINCH"));
        assert!(!cleaned.contains("clear"));
        assert!(cleaned.contains(r#"export PATH="$HOME/.local/bin:$PATH""#));
        assert!(cleaned.contains("alias ll='ls -la'"));
    }

    #[test]
    fn test_strip_rustfetch_hook_new_bash() {
        let input = r#"# .bashrc
export PATH="$HOME/.local/bin:$PATH"

# RustFetch — Shell startup integration
if [[ $- == *i* ]]; then
    rustfetch
fi

alias ll='ls -la'
"#;

        let cleaned = strip_rustfetch_hook(input);
        assert!(!cleaned.contains("RustFetch"));
        assert!(cleaned.contains(r#"export PATH="$HOME/.local/bin:$PATH""#));
        assert!(cleaned.contains("alias ll='ls -la'"));
    }

    #[test]
    fn test_strip_rustfetch_hook_zsh_and_fish() {
        let zsh_input = r#"# RustFetch — Automatic responsive redraw for tiling WMs (MangoWM, Hyprland, Sway, etc.)
if [[ -o interactive ]]; then
    _rustfetch_last_size=""
    TRAPWINCH() {
        local cur_size="${COLUMNS}x${LINES}"
        if [[ -n "$cur_size" && "$cur_size" != "$_rustfetch_last_size" ]]; then
            _rustfetch_last_size="$cur_size"
            if [[ -z "$BUFFER" ]]; then
                clear
                rustfetch
                zle && zle reset-prompt 2>/dev/null
            fi
        fi
    }
    _rustfetch_last_size="${COLUMNS}x${LINES}"
    rustfetch
fi
"#;
        let cleaned_zsh = strip_rustfetch_hook(zsh_input);
        assert!(cleaned_zsh.trim().is_empty());

        let fish_input = r#"# RustFetch — Automatic responsive redraw for tiling WMs (MangoWM, Hyprland, Sway, etc.)
if status is-interactive
    set -g _rustfetch_last_size ""
    function _rustfetch_on_resize --on-signal WINCH
        set -l cur_size (tput cols 2>/dev/null)x(tput lines 2>/dev/null)
        if test -n "$cur_size" -a "$cur_size" != "$_rustfetch_last_size"
            set -g _rustfetch_last_size "$cur_size"
            if test -z (commandline)
                clear
                rustfetch
                commandline -f repaint 2>/dev/null
            end
        end
    end
    set -g _rustfetch_last_size (tput cols 2>/dev/null)x(tput lines 2>/dev/null)
    rustfetch
end
"#;
        let cleaned_fish = strip_rustfetch_hook(fish_input);
        assert!(cleaned_fish.trim().is_empty());
    }
}
