{ name ? "ghcr.io/ferhatys/mongodb-cli-plugin"
, mongodb-cli-plugin
, dockerTools
}:

dockerTools.buildLayeredImage {
  inherit name;
  created = "now";
  config = {
    Entrypoint = [ "${mongodb-cli-plugin}/bin/mongodb-cli-plugin" ];
  };
}
