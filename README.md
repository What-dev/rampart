# Rampart: The local password manager nobody needs

Rampart is a simple password manager that encrypts your passwords using a master password that is only ever stored in an encrypted state on your computer. This means that your passwords are never stored in plaintext on your computer, and are only decrypted when you need them.

## Installation

#### No builds will be kept of this project, so you will have to build it yourself.

## Building
#### Download cargo using curl (You can also just go to rustup.rs)

 ```
 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 ```

#### Download the source code or clone the repo, then in the Rampart directory run:

```bash
cargo build --release
```

## TODO list:
- [ ] Add a way to change the master password (might just make it transfer it one by one to a new file, then delete the old one, dk yet.)
- [x] Add a way to change the password for a specific site
- [ ] Add a search feature
- [ ] Add a way to change the encryption algorithm (might be a cool feature, don't know of any other password managers that do this, but it might be a bit overkill)

