# ASUS Power Detector+ for Linux

nvcontrol monitors the six 12V-2x6 power pins on supported ASUS ROG Astral
graphics cards. It reports measured voltage, current and power per pin, connector
power, current balance and load-gated health findings.

## Supported cards

The Linux protocol is verified on subsystem `1043:89e3` (ROG Astral RTX 5090
OC). nvcontrol also recognizes the published Astral family IDs `89ea`, `8a61`,
`89ec`, `89de`, `8a2e`, `8a2b` and `8a45`; those SKUs inherit the same layout but
have not all been independently measured on Linux.

Unknown ASUS boards are never probed automatically.

## Usage

```bash
nvctl asus detect
nvctl asus power
nvctl asus power --json
nvctl asus power --watch
```

The GUI Power tab and TUI also display this telemetry. A failed refresh clears
the old value instead of presenting stale connector data.

## Data sources

nvcontrol selects the safest available source in this order:

1. A standard Linux hwmon device whose `name` is `astral12vhpwr`. The hwmon
   index is not stable, so nvcontrol discovers it by name and reads `in0`–`in5`
   with `curr1`–`curr6`.
2. A native SMBus read on NVIDIA adapter index 1 belonging to the selected GPU.
   This fallback does not invoke `i2cget` and never scans unrelated host buses.

Installing `astral-hwmon` is optional. It provides the cleanest system-wide
interface because `sensors`, nvcontrol and other monitoring software can share
the same cached, kernel-managed readings. nvcontrol's fallback keeps the feature
useful without requiring an out-of-tree module.

## Wire format

The IT8915FN answers at I2C address `0x2b`. A correct read is an
`I2C_SMBUS_I2C_BLOCK_DATA` transaction for 24 bytes starting at command `0x80`.
A generic repeated-start block read is not equivalent and has been observed to
return zeros on this adapter.

The frame contains six reversed four-byte records:

```text
u16 big-endian millivolts, u16 big-endian milliamps
```

Pin 1 is at byte offset 20 and pin 6 is at byte offset 0. nvcontrol rejects
all-zero frames, voltages outside 6–13 V, currents above 30 A, and incomplete
hwmon reads. Connector power is the sum of each pin's measured voltage times
measured current; it is not estimated using a fixed 12 V value.

## Health rules

Health is a read-only snapshot, not an instruction to shut down the machine.
Low-load current imbalance is expected, so imbalance/open-pin rules are gated on
connector load.

| Finding | Warning | Critical |
|---|---:|---:|
| Any pin current | 9.2 A | 9.5 A |
| Any pin voltage | below 11.4 V | below 11.0 V |
| Minimum/maximum pin current at 20 A+ total | below 70% | below 60% |
| Apparently open pin at 10 A+ total | — | below 0.5 A |

The balance percentage is only reported at 20 A or more total connector current.
nvcontrol does not automatically change the GPU power limit, terminate games or
shut down the host. Automated mitigation needs its own carefully debounced policy
and is intentionally out of scope.

## Permissions and troubleshooting

The hwmon interface is normally readable without direct I2C access. The fallback
must open `/dev/i2c-N` for read/write because an SMBus read sends a command byte;
it does not write telemetry or configuration data to the chip.

If the fallback reports `Permission denied`, grant the user access to the device's
owning group (commonly `i2c`) and log in again, or install `astral-hwmon`. Do not
run the whole GUI as root.

If no source is found, confirm that the NVIDIA driver exposes an adapter named
`NVIDIA i2c adapter 1 at <pci-address>` beneath the GPU. The NVIDIA 610 open
kernel module does not itself provide 12VHPWR hwmon sensors; `astral-hwmon` is a
separate community module.

## Safety and provenance

The direct backend performs only address selection and the documented SMBus
read transaction. It contains no register-data write path. nvcontrol implements
the public wire format independently and does not bundle code from the GPL
kernel module.

Protocol references:

- [LACT issue 906: ASUS Power Detector+ protocol research](https://github.com/ilya-zlobintsev/LACT/issues/906)
- [astral-hwmon](https://github.com/ksokolowski/astral-hwmon)

## See also

- [ASUS Astral notes](asus-astral.md)
- [RTX 5090 setup](rtx-5090-setup.md)
