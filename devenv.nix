{ ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  enterShell = ''
    echo "Starting shell"
  '';

  enterTest = ''
    echo "Running tests"
  '';
}
