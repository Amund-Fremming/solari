set dotenv-load := true

default:
    @just --list

stripe:
    cargo run -p solari --example stripe_scenarios --features stripe

vipps:
    cargo run -p solari --example vipps_scenarios --features vipps

all:
    @just stripe
    @just vipps
