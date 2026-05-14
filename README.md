<div align="center">

# solari

<img src="solari-coin.png" alt="solari" width="180" />

**A payment module built for fast shipping and easy setup, with storage, history APIs, and webhooks included.**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

</div>

---

## 🪾 Visual Tree

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

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).
