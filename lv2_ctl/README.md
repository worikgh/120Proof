# Control LV2 Simulators

Usage: `lv2_ctl <list of LV2 data>`

Wraps around [`mod-host`](https://github.com/moddevices/mod-host) facilitating convenient use of it.

## Goal

* Read settings for LV2 simulator from a file 
* Edit the settings while it is in use
* Save the settings back to a file

## List of LV2 Data

Use `serdi` from the [serd](https://gitlab.com/drobilla/serd) project to get a list of all the LV2 simulators and their data

```bash
find /usr/lib/lv2/ -name "*.ttl"  | perl -e '$p = 0; while($z = <>){chomp $z;  print `serdi  -p $p $z`;$p++}' > /tmp/lv2.dat
```

## Running This

```
puppy@raspberrypi:~/120Proof/lv2_ctl $ find /usr/lib/lv2/ -name "*.ttl"  | perl -e '$p = 0; while($z = <>){chomp $z;  print `serdi  -p $p $z`;$p++}' > /tmp/lv2.dat
puppy@raspberrypi:~/120Proof/lv2_ctl $ cat /tmp/lv2.dat |cargo run --release
```

## Modes/Screens

* Secreens are selected using the function keys `F1` and `F2`
* `F1` enters `List` state
* `F2` enters `Command` state

### List State

* Lists all known simulators

Key Bindings
---

| Key   | List State           | Command State        |
| Left  |                      | Port Adj. Down       |
| Right |                      | Port Adj. Up         |
| Down  | Next Simulator       | Next Port            |
| Up    | Previous Simulator   | Previous Port        |
| 'q'   | Quit programme       | Quit Programme       |
| 'u'   | Unselect Simulator   | Unselect Port        |
| 'g'   | Go to top of list    | Go to top of list    |
| 'G'   | Go to bottom of list | Go to bottom of list |
| 'n'   |                      | Next port            |
| 'p'   |                      | Previous port        |
| 's'   |                      | Save file name       |
| Enter | (De)Select simulator | (De)Select port      |
|       |                      |                      |

### Command State
