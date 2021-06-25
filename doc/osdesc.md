# OS Description files

OS description files are used to detected host os for options such as `onlyon`.

## Where are these files located?

Bundled OSes are included (by `include_str!`) when kelpdot compiles. This YAML list can be found in [/src/config/oses.yaml](https://github.com/Woomymy/kelp/blob/main/src/config/oses.yaml).

You can add custom OSes in `/etc/kelpdot/os.yaml`.

### Describing an OS

```yaml
# /etc/kelpdot/os.yaml
- name: "superos" # Name for `onlyon`
  file: "/" # if this file exist, we are **sure** the OS is O.K 
  priority: 0 # If multiple OSes are detected, the higher priority is more important. Use higher for childer distros
  prettyname: "My super Gentoo-Based distro" # Pretty name of the OS (for printing)
  submatches: [ "gentoo" ] # For exemple, mother distros
```