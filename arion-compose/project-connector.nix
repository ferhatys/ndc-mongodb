# Run v3 MongoDB connector and engine with supporting databases. To start this
# project run:
#
#     arion -f arion-compose/project-connector.nix up -d
#

{ pkgs, ... }:
let
  connector-port = "7130";
  engine-port = "7100";
  mongodb-port = "27017";
in
{
  project.name = "mongodb-connector";

  services = {
    connector = import ./service-mongodb-connector.nix {
      inherit pkgs;
      port = connector-port;
      hostPort = connector-port;
      otlp-endpoint = "http://jaeger:4317";
      service.depends_on = {
        jaeger.condition = "service_healthy";
      };
    };

    mongodb = import ./service-mongodb.nix {
      inherit pkgs;
      port = mongodb-port;
      hostPort = mongodb-port;
      volumes = [
        "mongodb:/data/db"
        (import ./fixtures-mongodb.nix).chinook
      ];
    };

    engine = import ./service-engine.nix {
      inherit pkgs;
      port = engine-port;
      hostPort = engine-port;
      connector-url = "http://connector:${connector-port}";
      otlp-endpoint = "http://jaeger:4317";
      service.depends_on = {
        auth-hook.condition = "service_started";
        jaeger.condition = "service_healthy";
      };
    };

    auth-hook = import ./service-dev-auth-webhook.nix { inherit pkgs; };

    jaeger = import ./service-jaeger.nix { inherit pkgs; };
  };

  docker-compose.volumes = {
    mongodb = null;
  };
}

