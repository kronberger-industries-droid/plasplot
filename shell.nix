{ pkgs }:

{
  buildInputs = with pkgs; [
    pkg-config
    fontconfig
    ];
}
