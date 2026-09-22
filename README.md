<div align="center">
  <img src="./assets/logo.png" width="250">
</div>

<div align="center">
  <h1>Bent0</h1>
  <p>A simple and lightweight hacking environment manager written in Rust.</p>
</div>

---

> [!WARNING]
> **Bent0 is under active development.**
> The project is not yet fully functional and may not work as expected out of the box.
> If you'd like to contribute, feel free to open a [pull request](https://github.com/diurned/bento/pulls).
> Found a bug, vulnerability, or something unusual? Please open an [issue](https://github.com/diurned/bento/issues).

---

## Why Bent0?

Bent0 is a hacking environment manager written in Rust that supports containers and microVMs. This project aims to help professional and amateur hackers manage their hacking environments. Bent0 uses containerd for managing containers because it's the standard way. Docker and Kubernetes themselves use it for managing containers on Linux environments. Bent0 can also manage microVMs (KVM) due to security reasons because it has better isolation than containers for use cases like malware analysis.

---

## Roadmap

- [ ] containers
  - [ ] pull images
  - [ ] remove images
  - [ ] create containers
  - [ ] remove containers
- [ ] microvms
  - [ ] pull images
  - [ ] remove images
  - [ ] create microvm
  - [ ] remove microvm

---

## The Grimoire

The Grimoire is the contains the official Bent0's documentation. You can read it [here](https://diurned.github.io/grimoire/bento).
