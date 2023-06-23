# Rampart: The local password manager nobody needs

Rampart is a simple password manager that encrypts your passwords using a master password that is only ever stored in an encrypted state on your computer. This means that your passwords are never stored in plaintext on your computer, and are only decrypted when you need them.

## Installation

#### No builds will be kept of this project, so you will have to build it yourself.

## Building
#### Download cargo using curl (You can also just go to rustup.rs)

 ```
 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 ```

#### Download the source code or clone the repo, then in the Taskly directory run:

```bash
cargo build --release
```

## TODO list:
- [ ] Add a way to change the master password
- [ ] Add a way to change the password for a specific site
- [ ] Add a search feature
- [ ] Add a way to change the encryption algorithm (might be a cool feature, don't know of any other password managers that do this, but it might be a bit overkill)

## Closing Notes
#### Second Rust project, so I'm sure there are a lot of things that could be done better. If you have any suggestions, feel free to open an issue or a pull request.
## What's next?
#### maybe some new features, mostly just bug fixes and code cleanup. This project was meant to be my dip into encryption, so I think I might've handled it well enough.
## What about Taskly?
#### It's sort-of dead... I'm not sure if I'll ever finish it, but I'm more likely to work on this than Taskly. Probably just switch between em and finish the TODO lists.