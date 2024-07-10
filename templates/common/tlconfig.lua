return {
    include_dir = {
        "src",
         "{{ meta.factorio.dirs.mods }}",
    },
    source_dir = "src",
    build_dir = "build",
    include = {
        "**/*.tl"
    },
    global_env_def = "types/factorio"
}
