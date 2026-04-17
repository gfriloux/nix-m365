{inputs, ...}: final: {
  inherit (inputs.self.packages.${final.system}) m365 m365-rs;
}
