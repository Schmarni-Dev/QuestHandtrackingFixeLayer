build:
    cargo build

test:
    cp ./target/debug/libquest_handtracking_fix_layer.so .
    XR_ENABLE_API_LAYERS=XR_APILAYER_NOVENDOR_quest_hand_fixes XR_API_LAYER_PATH=$PWD ~/Code/bevy_oxr/target/release/examples/overlay

install: 
    just build
    mkdir --parents ~/.local/share/openxr/1/api_layers/implicit.d
    cp ./target/debug/libquest_handtracking_fix_layer.so ~/.local/share/openxr/1/api_layers/implicit.d/
    cp ./XR_APILAYER_NOVENDOR_quest_hand_fixes.json ~/.local/share/openxr/1/api_layers/implicit.d/
		

build-and-test:
    just build && just test
