with import <nixpkgs> {};

mkShellNoCC rec {

    packages = [
        libsodium zeromq
        ninja pkg-config shellcheck
    ];

    env.LIBRARIES = lib.concatStringsSep " " [
        "libsodium" "libzmq"
    ];

    shellHook = /* bash */ ''
        declare -a library_paths=()
        declare -a cpaths=()

        IFS=" "; for lib in ${env.LIBRARIES}; do
            : true # fix treesitter highlight glitch :/
            # -L/nix/store/.../lib
            lib_flag="$( pkg-config --libs-only-L "$lib" )"
            # remove "-L"
            lib_path="''${lib_flag#'-L'}"
            library_paths+=( "$lib_path" )

            # -I/nix/store/.../include
            cflag="$( pkg-config --cflags-only-I "$lib" )"
            # remove "-I"
            search_path="''${cflag#'-I'}"
            cpaths+=( "$search_path" )
        done

        export CPATH=$( IFS=":"; echo "''${cpaths[*]}" )
        export LIBRARY_PATH=$( IFS=":"; echo "''${library_paths[*]}" )
    '';

}
