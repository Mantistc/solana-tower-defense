# Game made for learning purposes :)

# Solana Tower Defense

A **Tower Defense game** built with **Bevy** and integrated with **Solana** blockchain. Defend your base while interacting with Solana for in-game transactions, rewards, and wallet-based mechanics.

---

## **Features**
- Built with **Bevy** game engine
- **Solana wallet integration** for in-game purchases
- **Real-time balance updates** from Solana blockchain
- **Towers & Upgrades** with different strategies
- **Waves of enemies** increasing in difficulty

---

## **Getting Started**

### **Requirements**
1. **Rust** (latest stable version)
2. **Bevy** (game engine)
3. **A Solana Keypair json file** for transactions

---

### **Installation**

#### **1) Clone this repository**
```bash
git clone https://github.com/Mantistc/solana-tower-defense.git
cd solana-tower-defense
```

#### **2) Create your configuration file**
Create a `cfg.toml` file based on the example file:
```bash
cp cfg.toml.example cfg.toml
```
Then, edit it and add your custom settings.

#### **3) Build the application**
```bash
cargo build --release
```

#### **4) Run the game**
```bash
cargo run --release
```

---

## **How It Works**
- **Spend SOL**: Use your Solana wallet to interact with the game economy.
- **Deploy Towers**: Strategically place different towers to stop enemies.
- **Upgrade Defenses**: Improve towers using earned in-game currency.
- **Battle Waves**: Face increasing enemy difficulty as waves progress.

---

## License
This project is licensed under the **MIT License** – see the [LICENSE](./LICENSE) file for details.

---

<p align="center">
  Made with ❤️ by <a href="https://twitter.com/lich01_" target="_blank">@lich.sol</a>
</p>

