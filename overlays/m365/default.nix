{ channels, inputs, ... }:

final: prev: {
  m365 = inputs.self.packages.${final.system}.m365;
}
