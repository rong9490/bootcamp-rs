```
    https://github.com/tyr-rust-bootcamp
    cargo new --bin
    cargo init app --bin
    cargo init crates/claps --lib
    
    执行具体crate, 进入子目录 cargo run --example xx
    
    cargo install cargo-generate
    cargo generate tyr-rust-bootcamp/template
    pipx install pre-commit
    cargo install --locked cargo-deny
    cargo install typos-cli
    cargo install git-cliff
    cargo install cargo-nextest --locked
```

```aiignore
    https://github.com/tyr-rust-bootcamp/template
    https://github.com/tyr-rust-bootcamp/01-rcli
    https://github.com/tyr-rust-bootcamp/02-simple-redis

    https://github.com/tyr-rust-bootcamp/10-clientapps
    https://github.com/tyr-rust-bootcamp/10-chatapp
```

```aiignore
    Workspace
    Binary: bootcamp
    crates: * / toolbox / section01
    
    cargo new bootcamp (--bin)
    cargo new toolbox --lib
    
    resolver = "2"
    
    cargo run -p bootcamp
    
    顶层依赖
    [workspace.dependencies]
    rand = "0.8.5"
    子层依赖
    rand = { workspace = true }
    
    cargo test 执行全部crate的测试
    cargo test -p bootcamp 执行指定crate
```