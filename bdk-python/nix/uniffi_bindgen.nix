with import <nixpkgs> {};

rustPlatform.buildRustPackage rec {
  pname = "uniffi_bindgen";
  version = "0.15.2";
  src = fetchFromGitHub {
    owner = "mozilla";
    repo = "uniffi-rs";
    rev = "6fa9c06a394b4e9b219fa30fc94e353d17f86e11";
    # rev = "refs/tags/v0.14.1";
    sha256 = "1chahy1ac1r88drpslln2p1b04cbg79ylpxzyyp92s1z7ldm5ddb"; # 0.15.2
    # sha256 = "1mff3f3fqqzqx1yv70ff1yzdnvbd90vg2r477mzzcgisg1wfpwi0"; # 0.14.1
    fetchSubmodules = true;
  } + "/uniffi_bindgen/";

  doCheck = false;
  cargoSha256 = "sha256:08gg285fq8i32nf9kd8s0nn0niacd7sg8krv818nx41i18sm2cf3"; # 0.15.2
  # cargoSha256 = "sha256:01zp3rwlni988h02dqhkhzhwccs7bhwc1alhbf6gbw3av4b0m9cf"; # 0.14.1
  cargoPatches = [ ./uniffi_0.15.2_cargo_lock.patch ];
}
