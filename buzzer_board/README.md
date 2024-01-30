# embedded_buzzer

Quiz buzzer system with the [STM32H745XI discovery board][board].
Inspired by an [ESR Labs LabDays (hackathon) project][labdays_proj].

## Setup

Configure your local network interface to have a static IP in the same subnet
as the board (`192.168.100.1/24`):

```
2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc mq state UP group default qlen 1000
    ...
    inet 192.168.100.1/24 brd 192.168.100.255 scope global noprefixroute enp11s0
       valid_lft forever preferred_lft forever
```

Allow traffic on port `8000` in your firewall:

```bash
sudo ufw status
Status: active

To                         Action      From
--                         ------      ----
...
8000                       ALLOW       Anywhere
...
```

Set the right `udev` rules for your device. You can get the parameters from
`dmesg`. Example:

```
# /etc/udev/rules.d/70-st-link.rules

# STM32H745 MCU
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374e", TAG+="uaccess"
```

Install `probe-rs-cli`

```bash
cargo install probe-rs-cli
```

## Run

```
# Server
cd server
cargo run --release

# Board
cd buzzer_board
cargo run --bin buzzer
```

The two frontends are available at

- http://127.0.0.1:3000/reaction
- http://127.0.0.1:3000/quiz

[board]: https://www.st.com/en/evaluation-tools/stm32h745i-disco.html
[labdays_proj]: https://github.com/sameernegi17/QuizBuzzerSystem
