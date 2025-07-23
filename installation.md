For Linux 


1.Install Solana CLI

sh -c "$(curl -sSfL https://release.solana.com/stable/install)"


2.Install Rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

3.Install Anchor

cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
Get anchor version 
anchor --version
It should show something like this 
anchor-cli x.y.z


4. Starting with anchor project 

anchor init demo_coin --typescript
cd demo_coin

5.Running and Deploying Contract 

anchor build
anchor deploy 



Run this command to enable rust in current shell
source $HOME/.cargo/env

Then test
cargo --version

it should show something like this
cargo 1.88.0 (version may vary)


also if getting any error regarding installation of link cc or something run below command
sudo apt update && sudo apt install build-essential pkg-config libssl-dev -y
and after running the above command rerun the below command to install anchor nicely with almost ( 714 dependecies)
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

check anchor version by this 
anchor --version



Setting solana on devnet
solana config set --url https://api.devnet.solana.com
solana-keygen new --outfile ~/.config/solana/id.json
solana airdrop 2


and check solana address and balance 
solana address
solana balance


