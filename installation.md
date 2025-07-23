1.Install Solana CLI

sh -c "$(curl -sSfL https://release.solana.com/stable/install)"


2.Install Rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

3.Install Anchor

cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked


4. Starting with anchor project 

anchor init demo_coin --typescript
cd demo_coin

5.Running and Deploying Contract 

anchor build
anchor deploy 
