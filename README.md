
## Build the with Dfinn Node 💃

To build Dfinn, you will need a proper Substrate development environment. If you need a refresher setting up your Substrate environment, see [Substrate's Getting Started Guide](https://substrate.dev/docs/en/knowledgebase/getting-started/).


```bash
# Fetch the code
git clone https://github.com/Bharathcoorg/Dfinn.git
cd Dfinn

# Build the node (The first build will be long (~30min))
cargo build --release
```

If a cargo not found error shows up in the terminal, manually add Rust to your system path (or restart your system):

```bash
source $HOME/.cargo/env
```

Then, you will want to run the node in dev mode using the following command:

```bash
./target/release/dfinn-node --dev
```

> For people not familiar with Substrate, the --dev flag is a way to run a Substrate-based node in a single node developer configuration for testing purposes. You can learn more about `--dev` in [this Substrate tutorial](https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/interact).

When running a node via the binary file, data is stored in a local directory typically located in ~/.local/shared/polkadex-node/chains/development/db. If you want to start a fresh instance of the node, you can either delete the content of the folder, or run the following command inside the dfinn folder:

```bash
./target/release/dfinn-node purge-chain --dev
```

This will remove the data folder, note that all chain data is now lost.

## Run a local network (two nodes)

- Install `subkey`, `jq`
```bash
curl https://getsubstrate.io -sSf | bash -s --
brew install jq
```

- Generate node key using `subkey`
```bash
Alice_Node_Key=$(subkey generate --scheme Ed25519 --output-type Json | jq -r '.secretSeed')
```

- Run Alice's node

```bash
# Purge any chain data from previous runs
./target/release/dfinn-node purge-chain --base-path /tmp/alice --chain local

# Start Alice's node
./target/release/dfinn-node --base-path /tmp/alice \
  --chain local \
  --alice \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --node-key $Alice_Node_Key \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator
```

```bash
2021-06-30 08:12:38 Dfinn Node    
2021-06-30 08:12:38 ✌️  version 3.0.0-6426a73b-x86_64-macos    
2021-06-30 08:12:38 ❤️  by Substrate DevHub <https://github.com/substrate-developer-hub>, 2017-2021    
2021-06-30 08:12:38 📋 Chain specification: Local Testnet    
2021-06-30 08:12:38 🏷 Node name: Alice    
2021-06-30 08:12:38 👤 Role: AUTHORITY    
2021-06-30 08:12:38 💾 Database: RocksDb at /tmp/alice/chains/local_testnet/db    
2021-06-30 08:12:38 ⛓  Native runtime: node-polkadex-265 (node-polkadex-1.tx2.au10)    
2021-06-30 08:12:39 🔨 Initializing Genesis block/state (state: 0xbe0a…5ef3, header-hash: 0xa55f…7888)    
2021-06-30 08:12:39 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.    
2021-06-30 08:12:39 ⏱  Loaded block-time = 3s from genesis on first-launch    
2021-06-30 08:12:39 👶 Creating empty BABE epoch changes on what appears to be first startup.    
2021-06-30 08:12:39 Using default protocol ID "sup" because none is configured in the chain specs    
2021-06-30 08:12:39 🏷 Local node identity is: 12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp    
2021-06-30 08:12:39 📦 Highest known block at #0    
2021-06-30 08:12:39 〽️ Prometheus server started at 127.0.0.1:9615    
2021-06-30 08:12:39 Listening for new connections on 127.0.0.1:9945.    
2021-06-30 08:12:39 👶 Starting BABE Authorship worker    
2021-06-30 08:12:44 💤 Idle (0 peers), best: #0 (0xa55f…7888), finalized #0 (0xa55f…7888), ⬇ 0 ⬆ 0    
2021-06-30 08:12:49 💤 Idle (0 peers), best: #0 (0xa55f…7888), finalized #0 (0xa55f…7888), ⬇ 0 ⬆ 0    
2021-06-30 08:12:54 💤 Idle (0 peers), best: #0 (0xa55f…7888), finalized #0 (0xa55f…7888), ⬇ 0 ⬆ 0
```

Local node identity is: 12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp shows the Peer ID that Bob will need when booting from Alice's node. This value was determined by the --node-key that was used to start Alice's node.

Now that Alice's node is up and running, Bob can join the network by bootstrapping from her node.
```bash
./target/release/dfinn-node purge-chain --base-path /tmp/bob --chain local
./target/release/dfinn-node \
  --base-path /tmp/bob \
  --chain local \
  --bob \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

If all is going well, after a few seconds, the nodes should peer together and start producing blocks. You should see some lines like the following in the console that started Alice node.

```bash
2021-06-30 08:16:52 Dfinn Node    
2021-06-30 08:16:52 ✌️  version 3.0.0-6426a73b-x86_64-macos    
2021-06-30 08:16:52 ❤️  by Substrate DevHub <https://github.com/substrate-developer-hub>, 2017-2021    
2021-06-30 08:16:52 📋 Chain specification: Local Testnet    
2021-06-30 08:16:52 🏷 Node name: Bob    
2021-06-30 08:16:52 👤 Role: AUTHORITY    
2021-06-30 08:16:52 💾 Database: RocksDb at /tmp/bob/chains/local_testnet/db    
2021-06-30 08:16:52 ⛓  Native runtime: node-polkadex-265 (node-polkadex-1.tx2.au10)    
2021-06-30 08:16:52 🔨 Initializing Genesis block/state (state: 0xbe0a…5ef3, header-hash: 0xa55f…7888)    
2021-06-30 08:16:52 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.    
2021-06-30 08:16:52 ⏱  Loaded block-time = 3s from genesis on first-launch    
2021-06-30 08:16:52 👶 Creating empty BABE epoch changes on what appears to be first startup.    
2021-06-30 08:16:52 Using default protocol ID "sup" because none is configured in the chain specs    
2021-06-30 08:16:52 🏷 Local node identity is: 12D3KooWRHDuuHg5ZQcJhvVDKud9XkFz2Dcs2GQKF9KKuTD6quq7    
2021-06-30 08:16:53 📦 Highest known block at #0    
2021-06-30 08:16:53 Listening for new connections on 127.0.0.1:9946.    
2021-06-30 08:16:53 👶 Starting BABE Authorship worker    
2021-06-30 08:16:53 🔍 Discovered new external address for our node: /ip4/127.0.0.1/tcp/30334/p2p/12D3KooWRHDuuHg5ZQcJhvVDKud9XkFz2Dcs2GQKF9KKuTD6quq7    
2021-06-30 08:16:53 🔍 Discovered new external address for our node: /ip4/192.168.1.37/tcp/30334/p2p/12D3KooWRHDuuHg5ZQcJhvVDKud9XkFz2Dcs2GQKF9KKuTD6quq7    
2021-06-30 08:16:53 Creating inherent data providers took more time than we had left for the slot.    
2021-06-30 08:16:54 🙌 Starting consensus session on top of parent 0xa55fa19cc37ca1f8d93bc06ca1f6fee767f18200516d9e349938601a3fe97888    
2021-06-30 08:16:54 🎁 Prepared block for proposing at 1 [hash: 0x2959db5e42a7192434d3699d335e5d920da73409963e3081ad43afd93a8cdb4b; parent_hash: 0xa55f…7888; extrinsics (1): [0x4431…4eff]]    
2021-06-30 08:16:54 🔖 Pre-sealed block for proposal at 1. Hash now 0x5263ed1cbf1b4edbc887cc87786471819cd0614d8aeaff3a898c0c3ffda245c2, previously 0x2959db5e42a7192434d3699d335e5d920da73409963e3081ad43afd93a8cdb4b.    
2021-06-30 08:16:54 👶 New epoch 0 launching at block 0x5263…45c2 (block slot 541685138 >= start slot 541685138).    
2021-06-30 08:16:54 👶 Next epoch starts at slot 541685338    
2021-06-30 08:16:54 ✨ Imported #1 (0x5263…45c2)    
2021-06-30 08:16:57 🙌 Starting consensus session on top of parent 0x5263ed1cbf1b4edbc887cc87786471819cd0614d8aeaff3a898c0c3ffda245c2    
2021-06-30 08:16:57 🎁 Prepared block for proposing at 2 [hash: 0x0c513e39a88bcb03d113a18ed824bcbaab03881e9dcdeedbe12e71955dcfe05d; parent_hash: 0x5263…45c2; extrinsics (1): [0xf06a…bf04]]    
2021-06-30 08:16:57 🔖 Pre-sealed block for proposal at 2. Hash now 0x4293ecd46db852f5add54a24acfcd1ea12f6c26d5470b61736d7cf0e039e3e39, previously 0x0c513e39a88bcb03d113a18ed824bcbaab03881e9dcdeedbe12e71955dcfe05d.    
2021-06-30 08:16:57 ✨ Imported #2 (0x4293…3e39)    
2021-06-30 08:16:58 💤 Idle (1 peers), best: #2 (0x4293…3e39), finalized #0 (0xa55f…7888), ⬇ 1.7kiB/s ⬆ 1.9kiB/s 
```

## Using docker

The following commands will setup a local dfinn network made of 2 nodes. It's using the node key (0000000000000000000000000000000000000000000000000000000000000001). But you should generate your own node key using the subkey as the above.

```bash
docker build . -t dfinn-node
docker-compose -f 2nodes.yml up --force-recreate
```

## Connecting Polkadot JS Apps to a Local Node
The development node is a Substrate-based node, so you can interact with it using standard Substrate tools. The two provided RPC endpoints are:
- HTTP: `http://127.0.0.1:9933`
- WS: `ws://127.0.0.1:9944`

Start by connecting to it with Polkadot JS Apps. Open a browser to: https://polkadot.js.org/apps/#/explorer. This will open Polkadot JS Apps, which automatically connects to Polkadot MainNet.

Click on the top left corner to open the menu to configure the networks, and then navigate down to open the Development sub-menu. In there, you will want to toggle the "Local Node" option, which points Polkadot JS Apps to ws://127.0.0.1:9944. Next, select the Switch button, and the site should connect to your Polkadex development node.

## How to run a validator
Here is a guide [Run a validator ](./docs/run-a-validator.md)

## Contribute :heart_eyes:
We would love to work with anyone who can contribute their work and improve this project. The details will be shared soon.
