default: gen lint

gen:
    flutter pub get
    flutter_rust_bridge_codegen \
        --rust-input native/src/api.rs \
        --dart-output lib/bridge_generated.dart \
        --extra-c-output-path macos/Runner/ \
        --dart-decl-output lib/bridge_definitions.dart

# --c-output ios/Runner/bridge_generated.h \
# --wasm

lint:
    cd native && cargo fmt
    dart format .

clean:
    flutter clean
    cd native && cargo clean
    
serve *args='':
    flutter pub run flutter_rust_bridge:serve {{args}}

platform := os()

run:
    flutter run -d {{platform}}

# vim:expandtab:sw=4:ts=4
