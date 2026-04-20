# Stellar Sosmed

A simple decentralized Sosmed app built on Stellar using Soroban.

---

## 🧠 About The Project

Stellar Notes is a lightweight decentralized application (DApp) that lets you create, store, and manage notes directly on the blockchain.

Instead of relying on traditional databases, all notes are stored inside a smart contract. This means your data isn't controlled by any single server — it's transparent, persistent, and verifiable on-chain.

The goal of this project is not to replace full-featured apps like Notion, but to explore how far a simple idea like note-taking can go when combined with blockchain technology.

---

## 🎯 Why This Exists

Most note apps today are:
- centralized
- dependent on a provider
- and can modify or remove your data

This project experiments with a different approach:
> What if your notes actually *belong to you*?

With Stellar Notes:
- Data lives on-chain
- Access is handled by smart contract logic
- No backend server needed

---

## ⚙️ Features

### ✍️ Create Notes
- Add notes with a title and content
- Each note gets a unique ID automatically
- Stored permanently in contract storage

### 📖 View Notes
- Retrieve all notes in one call
- Structured format (easy to connect with frontend)
- Always reflects the latest blockchain state

### 🗑️ Delete Notes
- Remove notes using their ID
- Updates storage instantly
- Keeps data clean and manageable

---

## 🔐 Security & Transparency

- All operations are executed on-chain
- No hidden backend logic
- Data integrity is guaranteed by the smart contract
- Anyone can verify transactions via the Stellar network

---

## 🧱 Tech Stack

- **Rust**
- **Soroban SDK**
- **Stellar Blockchain**

---

## 🚀 How It Works

The contract exposes 3 main functions:

- `create_note(title, content)`  
  → Add a new note

- `get_notes()`  
  → Retrieve all notes

- `delete_note(id)`  
  → Delete a specific note

Deploy the contract, call the functions, and you're good to go.

---

## 🔮 Future Improvements

This project is intentionally simple, but there's a lot of room to expand:

### Near Term
- Note encryption (basic privacy layer)
- Tagging / categories
- Markdown support
- Search functionality

### Mid Term
- Shared notes (multi-user access)
- Permission system (read/write roles)
- Notifications (off-chain integration)

### Long Term
- Cross-chain support
- Decentralized frontend (IPFS)
- AI-assisted summaries
- Zero-knowledge privacy layer
- DAO-based governance

---

## 🧪 Use Cases

- Personal note storage (on-chain)
- Immutable logs / journaling
- Learning project for Soroban development
- Base layer for more complex DApps

---

## 📌 Contract Info
ID smartcontract = CCXD4YI4B2NZGP2FQMYTRG7NXQRQ647S4TR4ICUW3PHMVCBEWTMECAN5