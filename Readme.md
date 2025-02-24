
# **Project for course Formal Verification of Cryptosystems**

A simple blockchain implementation in Rust with a **peer-to-peer (P2P) network** using **libp2p** and a **Proof of Work (PoW)** consensus mechanism. 


---

## **Getting Started**

### **Clone the Repository**

```bash
git clone https://github.com/your_username/blockchain-p2p.git
cd blockchain-p2p
```


---
### **Documentation**
For the project a documentation has been created using cargo docs.

Open it with:

```bash
cargo doc --open
```

---

### **Running the Project**

Start a node using:

```bash
RUST_LOG=info cargo run
```

To run **multiple peers** (in separate terminals or on different devices in the same network):

```bash
RUST_LOG=info cargo run
```

Peers will discover each other using **mDNS**.

---

## **CLI Commands**

While the node is running, use the following commands in the terminal:

|    **Command**            |     **Description**                         |
|---------------------------|---------------------------------------------|
| `ls p`                    | List connected peers                        |
| `add block <data>`        | Add a new block with `<data>`               |
| `ls chain`                | View the current blockchain                 |

---
