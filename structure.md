# Solari Repository Map

## Visual Tree

```text
solari/
├── crates/                        # Rust workspace crates
│   ├── solari-core/
│   │   └── src/
│   │       ├── modules/
│   │       │   ├── vipps/
│   │       │   ├── apple_pay/
│   │       │   └── stripe/
│   │       ├── traits.rs
│   │       ├── core.rs
│   │       └── lib.rs
│   └── solari-client/
│       └── src/
│           ├── handlers/
│           │   ├── webhook.rs
│           │   └── api.rs
│           ├── storage/
│           │   └── traits.rs
│           ├── core.rs
│           └── lib.rs
├── examples/                      # Example integrations / sandbox apps
│   ├── next-test/
│   ├── expo-test/
│   └── axum-test/
└── packages/                      # JavaScript/TypeScript packages
   └── solari-js/
```
