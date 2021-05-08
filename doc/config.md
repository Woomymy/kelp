<!---Disable markdownlint warnings if 2 headers have the same name-->
<!--markdownlint-disable-file MD024-->
# Configuration guide

- [Homefiles](#homefiles)
- [Rootfiles](#rootfiles)

## Homefiles

> The "Homefiles" key contains an array with the informations about the files located at /home/$USER/

### Structure

```yaml
homefiles:
    - name: "I3 configuration" # String: The name of the file (optional)
      path: .config/i3/config # String: The path to the file / directory relative to /home/$USER
      backuponly: false # Use true if you want the file to not be reinstalled with `kelpdot install` (optional) (default: false)
      onlyon: gentoo # Specify if the file can only be installed in one distro (optional) (default: None)
```

## Rootfiles

> The "Rootfiles" key contains an array with the informations about the files with path relative to /

### Structure

```yaml
rootfiles:
    - name: SysCtl configuration # String: The name of the file (optional)
      path: /etc/sysctl.conf # String: The path to the file / directory
      backuponly: true # Use true if you want the file to not be reinstalled with `kelpdot install` (optional) (default: false)
      onlyon: gentoo # Specify if the file can only be installed in one distro (optional) (default: None)
```

## Postsave

> The scripts to run **AFTER** saving the dotfiles

```yaml
postsave:
    - path: "scripts/usefullscript.sh" # The path to the script, relative to the kelp.yaml file
      elevated: true # Use true to run the script with eleveted privileges (optional) (default: false)
```

## Prerun

> The scripts to run **BEFORE** installing the dotfiles

```yaml
postsave:
    - path: "scripts/usefullscript.sh" # The path to the script, relative to the kelp.yaml file
      elevated: true # Use true to run the script with eleveted privileges (optional) (default: false)
```

## Postrun

> The scripts to run **AFTER** installing the dotfiles

```yaml
postsave:
    - path: "scripts/usefullscript.sh" # The path to the script, relative to the kelp.yaml file
      elevated: true # Use true to run the script with eleveted privileges (optional) (default: false)
```
