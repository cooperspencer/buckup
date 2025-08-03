# BUCKUP
### Buckle up, here comes the backup!

![Robbie](assets/robbie_small.webp)

## What is Buckup?
**Buckup** is a tool for backing up repositories from GitHub.

Each backup is stored as a **bare repository**, preserving all Git history.

## Configuration
Example configuration:
```yaml
source:
  github:
    - token: <token>
      users:
        - kubernetes

destination:
  local:
    - path: backup
```

## Future Plans
**Planned source support:**
- ~~Gitea / Forgejo~~
- Gogs
- GitLab
- OneDev
- SourceHut

**Planned destination support:**
- Gitea / Forgejo
- Gogs
- GitLab
- OneDev
- SourceHut
- GitHub
