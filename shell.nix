with import <nixpkgs> {};

mkShell {
  name = "bdk-python-shell";
  packages = [ ( import ./nix/uniffi_bindgen.nix ) ];
  buildInputs = with python37.pkgs; [
    pip
    setuptools
  ];
  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib:$LD_LIBRARY_PATH
    alias pip="PIP_PREFIX='$(pwd)/_build/pip_packages' \pip"
    export PYTHONPATH="$(pwd)/_build/pip_packages/lib/python3.7/site-packages:$(pwd):$PYTHONPATH"
    export PATH="$(pwd)/_build/pip_packages/bin:$PATH"
    unset SOURCE_DATE_EPOCH
  '';
}
