library ieee;

use ieee.std_logic_1164.all;

entity component_latch is
  port (
    clk : in    std_logic;
    d   : in    std_logic_vector(3 downto 0);
    q   : out   std_logic_vector(3 downto 0)
  );
end entity component_latch;

architecture synth of component_latch is

begin
  process (clk, d) is
  begin
    if (clk = '1') then
      q <= d;
    end if;
  end process;
end architecture synth;
