# svnstatus

Displays svn status on stdout (to be used by starship)

In `~/.config/starship.toml` I have

```

[custom.svn]
when = "svn --non-interactive info >/dev/null 2>&1"
command = "~/bin/svnstatus"
symbol = "svn"
format = '\[[$symbol ($output)]($style)\]'

```
