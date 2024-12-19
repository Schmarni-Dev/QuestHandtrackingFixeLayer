build:
    cargo build

test:
    XR_ENABLE_API_LAYERS=XR_APILAYER_NOVENDOR_quest_hand_fixes XR_API_LAYER_PATH=$PWD ~/Code/bevy_oxr/target/release/examples/overlay

build-and-test:
    just build && just test
