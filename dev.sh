#!/bin/bash

cargo-build() {
    docker exec \
    -it \
    docker_datamkt_1 \
    /root/.cargo/bin/cargo build --release --target=wasm32-unknown-unknown
}

wasm-gc() {
    docker exec \
    -it \
    docker_datamkt_1 \
    /root/.cargo/bin/wasm-gc target/wasm32-unknown-unknown/release/datamkt.wasm datamkt_gc.wasm
}

wasm-opt() {
    docker exec \
    -it \
    docker_datamkt_1 \
    wasm-opt datamkt_gc.wasm --output datamkt_gc_opt.wasm -Oz
}

build() {
    cargo-build
    wasm-gc
    wasm-opt
}

wallet-create() {
    docker exec \
    -it \
    docker_keosd_1 \
    cleos \
    --url http://nodeosd:8888 \
    --wallet-url http://127.0.0.1:8900 \
    wallet create \
    -n datamkt1 \
    --to-console
}

wallet-unlock() {
    docker exec \
    -it \
    docker_keosd_1 \
    bash -c 'cleos \
    --url http://nodeosd:8888 \
    --wallet-url http://127.0.0.1:8900 \
    wallet unlock \
    -n datamkt1 \
    --password ${WALLET_PASSWORD}'
}

wallet-import() {
docker exec -it docker_keosd_1 \
    bash -c 'cleos \
    --url http://nodeosd:8888 \
    --wallet-url http://127.0.0.1:8900 \
    wallet import \
    -n datamkt1 \
    --private-key ${EOS_PRIVKEY}'
}

account-create() {
    docker exec \
    -it \
    docker_keosd_1 \
    bash -c 'cleos \
    --url http://nodeosd:8888 \
    --wallet-url http://127.0.0.1:8900 \
    create account eosio datamkt1 ${EOS_PUBKEY}'
}

set-abi-code() {
    docker exec \
    -it \
    docker_keosd_1 \
    cleos \
    --url http://nodeosd:8888 \
    --wallet-url http://127.0.0.1:8900 \
    set abi datamkt1 datamkt.abi.json
    set code datamkt1 datamkt_gc_opt.wasm
}

install_eoslime() {
    docker exec \
    -it \
    docker_nodeosd_1 \
    npm install --prefix /project/testing -y --save-dev --verbose
}

run_test() {
    docker exec \
    -it \
    docker_nodeosd_1 \
    npm test --prefix testing tests/basic_sub_operations.js
}

run() {
    build
    wallet-unlock
    account-create
    set-abi-code
    install_eoslime
    run_test
}

if [ "$1" == "run" ]; then
    echo "run"
    run
fi

if [ "$1" == "build" ]; then
    echo "build"
    build
fi

if [ "$1" == "set" ]; then
    echo "set"
    set-abi-code
fi

if [ "$1" == "wallet-create" ]; then
    echo "create wallet"
    wallet-create
fi

if [ "$1" == "wallet-unlock" ]; then
    echo "wallet unlock"
    wallet-unlock
fi

if [ "$1" == "account-create" ]; then
    echo "creating account"
    account-create
fi

if [ "$1" == "import" ]; then
    echo "wallet import"
    wallet-import
fi

if [ "$1" == "test" ]; then
    echo "test"
    install_eoslime # It's okay to run this repeatedly.
    run_test
fi
