# 👻 **_Perona_**

[![CI](https://github.com/NukeZzZin/Perona/actions/workflows/ci.yml/badge.svg)](https://github.com/NukeZzZin/Perona/actions/workflows/ci.yml)
[![LICENSE](https://img.shields.io/badge/license-AGPL%20v3-blue.svg)](https://github.com/NukeZzZin/Perona/blob/master/LICENSE)

This project is just a simple discord bot, made in **_Rust_** using **_Serenity_**, **_Tokio_**, **_Dotenv_**, **_MongoDB_**. With builds in Windows, Linux and others systems using **_Cargo_**.

## 🌎 **_Prerequisites_**

- Create a database with [**_MongoDB_**](https://www.mongodb.com).
- Install the [**_Rust_**](https://www.rust-lang.org/tools/install).
- Install Rust Dependencies using **_`cargo check`_** in your terminal.
- Setup [**_Discord Developer Portal_**](https://discord.com/developers/applications) settings.
- Setup local environments in **_`.env`_** file, as in example in **_`.env.example`_**.

### 🚚 **_Installation_**

1. Clone the repository using **_`git clone https://github.com/NukeZzZin/Perona.git`_** in your terminal.
2. Install Rust Dependencies using **_`cargo check`_** in your terminal.

#### 🐱‍💻 **_Local Developing run_**

1. Use **_`cargo run --verbose`_** in your terminal.

#### 🏎️ **_Local production run_**

1. Use **_`cargo build --release --verbose`_** in your terminal.
2. Use **_`./target/release/perona`_** in your terminal.

#### 📦 **_Docker production run_**

1. Build docker container using **_`docker-compose build`_** in your terminal.
2. Initialize docker container using **_`docker-compose up -d`_** in your terminal.

## 📝 **_License_**

> **_You can check out the full license [here](https://github.com/NukeZzZin/Perona/blob/master/LICENSE)._**

_This project is licensed under the terms of the **_GNU AFFERO GENERAL PUBLIC LICENSE v3.0_** license._
