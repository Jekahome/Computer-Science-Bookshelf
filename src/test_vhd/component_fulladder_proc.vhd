library ieee;

use ieee.std_logic_1164.all;

entity component_fulladder_proc is
  port (
    a    : in    std_logic;
    b    : in    std_logic;
    cin  : in    std_logic;
    s    : out   std_logic;
    cout : out   std_logic
  );
end entity component_fulladder_proc;

architecture synth of component_fulladder_proc is

begin
  process (all) is
    variable p, g : std_logic;
  begin
    p    := a xor b; -- блокирующее
    g    := a and b; -- блокирующее
    s    <= p xor cin;
    cout <= g or (p and cin);
  end process;
end architecture synth;
