# set windows-shell := ["C:\\Program Files\\Git\\bin\\sh.exe", "-c"]

set unstable := true
set fallback := true

# List all recipes
list:
    @just --list --unstable

# Format all justfiles with just and other files with treefmt
fmt:
    #!/usr/bin/env nu
    # Nushell version
    ls **/*justfile | each { |it|
        echo $"Formatting ($it.name)"
        do { just --fmt --unstable --justfile $it.name } catch { echo $"Failed to format ($it.name)" }
    }
    treefmt --clear-cache --fail-on-change

fmt-ps:
    #!/usr/bin/env pwsh
    # PowerShell version
    Get-ChildItem -Recurse -Include "justfile","*.justfile",".justfile" | ForEach-Object {
        Write-Host "Formatting $_"
        try {
            just --fmt --unstable --justfile $_
        } catch {
            Write-Host "Failed to format $_"
        }
    }
    treefmt --clear-cache --fail-on-change

fmt-sh:
    #!/usr/bin/env sh
    # Shell version (original)
    find . -name "justfile" -o -name "*.justfile" -o -name ".justfile" | \
    while read -r file; do echo "Formatting $file"; \
    just --fmt --unstable --justfile "$file" || \
    echo "Failed to format $file"; \
    done
    treefmt --clear-cache --fail-on-change

# Push local changes to the remote repository
push MESSAGE:
    jj describe --message "{{ MESSAGE }}"
    jj bookmark set main --revision=@
    jj git push

# Push local changes to the remote repository, interactively.
push-interactive:
    jj describe
    jj bookmark set main --revision=@
    jj git push

# Initialize the SQL Server
sql-up:
    cd environment && \
    podman compose --file sql_server.compose.yml up --detach --no-recreate --build --remove-orphans

# Stop the SQL Server
sql-down:
    cd environment && \
    podman compose --file sql_server.compose.yml down --volumes --remove-orphans
