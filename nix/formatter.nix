{ pkgs }:

pkgs.writeShellApplication {
  name = "xtm-rustfmt";
  runtimeInputs = [ pkgs.cargo pkgs.rustfmt ];
  text = ''
    set -euo pipefail
    cd "$PWD"
    cargo fmt --all --manifest-path "$PWD/Cargo.toml" -- --config max_width=80 --config comment_width=80 --config wrap_comments=true --config format_code_in_doc_comments=true
  '';
}
