{ lib, fetchFromGitHub, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "tuxfetch";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "wiredhikari";
    repo = pname;
    rev = version;
    sha256 = "1hqps7l5qrjh9f914r5i6kmcz6f1yb951nv4lby0cjnp5l253kps";
  };

  cargoSha256 = "03wf9r2csi6jpa7v5sw5lpxkrk4wfzwmzx7k3991q3bdjzcwnnwp";

  meta = with lib; {
    description = " A command-line system information tool written in rust";
    homepage = "https://github.com/wiredhikari/tuxfetch";
    license = licenses.unlicense;
    maintainers = [ maintainers.tailhook ];
  };
}
