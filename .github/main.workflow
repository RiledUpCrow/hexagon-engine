workflow "CI" {
  on = "push"
  resolves = ["Push"]
}

action "Build" {
  uses = "actions/docker/cli@8cdf801b322af5f369e00d85e9cf3a7122f49108"
  args = "build --target build -t co0sh/hexagon-engine:dev ."
}

action "Login" {
  uses = "actions/docker/login@8cdf801b322af5f369e00d85e9cf3a7122f49108"
  secrets = ["DOCKER_USERNAME", "DOCKER_PASSWORD"]
}

action "Style" {
  uses = "actions/docker/cli@8cdf801b322af5f369e00d85e9cf3a7122f49108"
  needs = ["Build"]
  args = "run --rm co0sh/hexagon-engine:dev cargo fmt --check"
}

action "Test" {
  uses = "actions/docker/cli@8cdf801b322af5f369e00d85e9cf3a7122f49108"
  needs = ["Build"]
  args = "run --rm co0sh/hexagon-engine:dev cargo test --release"
}

action "Release" {
  uses = "actions/docker/cli@8cdf801b322af5f369e00d85e9cf3a7122f49108"
  needs = ["Style", "Test"]
  args = "build -t co0sh/hexagon-engine:latest ."
}

action "Push" {
  uses = "actions/docker/cli@8cdf801b322af5f369e00d85e9cf3a7122f49108"
  needs = ["Release", "Login"]
  args = "push co0sh/hexagon-engine:latest"
}
