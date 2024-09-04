{ lib, fetchFromGitHub, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "brotomc-agentrs";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "eveeifyeve";
    repo = pname;
    rev = version;
    hash = lib.fakeHash;
  };

  cargoHash = lib.fakeHash;

  meta = with lib; {
    description = "Brontomc Agent in rust";
    homepage = "https://github.com/eveeifyeve/brontomc-agentrs";
    license = licenses.mit;
    maintainers = with maintainers; [ eveeifyeve ];
  };
}