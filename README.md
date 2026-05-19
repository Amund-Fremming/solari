<div align="center">

# solari

<img src="solari-coin.png" alt="solari" width="180" />

**A payment module built for fast shipping and easy setup,**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

</div>

---

## 🪾 Visual Tree

```text
solari/
├── crates/                        # Rust workspace crates
│   └── solari/                    # Unified library (service + adapters + optional API)
│       ├── src/
│       │   ├── adapters/
│       │   ├── handlers/
│       │   ├── storage/
│       │   └── lib.rs
│       └── examples/
├── examples/                      # Example integrations / sandbox apps
│   ├── next-test/
│   ├── expo-test/
│   └── axum-test/
├── .github/
│   └── workflows/
│       ├── publish-rust.yml
│       └── publish-npm.yml
└── packages/                      # JavaScript/TypeScript packages
   └── solari-js/
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).

## Payment setup guides

- [Stripe setup](stripe-how2.md)
- [Apple Pay setup](apple-pay-how2.md)
- [Vipps notes](vipps-how2.md)
