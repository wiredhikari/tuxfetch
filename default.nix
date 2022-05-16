{ lib, fetchFromGitHub, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "tuxfetch";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "wiredhikari";
    repo = pname;
    rev = version;
    sha256 = "1xlcnyy7zcnq78d9fv6k3vsawvbwzvailp21qskzsbpx299h3wpc";
  };

  cargoSha256 = "161nhazif24f7m555khmdvp9as2kndd47aisi8nbdwx91advyzkf";

  meta = with lib; {
    description = " A command-line system information tool written in rust";
    homepage = "https://github.com/wiredhikari/tuxfetch";
    license = licenses.unlicense;
    maintainers = [ maintainers.tailhook ];
  };
}
