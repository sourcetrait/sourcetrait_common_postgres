#!/usr/bin/env nu

mut env_changed = false

# General tools 
brew install xz 

cargo install cargo-expand

# MacOS packaging tools
brew tap ceejbot/tap
brew install tomato formulaic

# PostgreSQL Extensions (pgrx)
brew install icu4c pkg-config
cargo install --locked cargo-pgrx

if ($env.PKG_CONFIG_PATH? | is-empty) {
    let val = "/opt/homebrew/opt/icu4c/lib/pkgconfig"
    if ($val | path exists) {
        if (open $nu.config-path | lines | where $it =~ "$env.PKG_CONFIG_PATH =" | is-empty) {
            $"$env.PKG_CONFIG_PATH = \"($val)\"\n" | save --append $nu.config-path
            $env_changed = true
        }
    } else {
        print $"(ansi orange)setup:(ansi reset) unable to set $env.PKG_CONFIG_PATH"
    }
}

if $env_changed {
    print $"\n(ansi yellow)setup:(ansi reset) done. environment has changed."
    print $"reload with: (ansi cyan)source $nu.config-path(ansi reset)"
} else {
    print $"\n(ansi yellow)setup:(ansi reset) done."
}
