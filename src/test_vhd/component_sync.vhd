library ieee;

use ieee.std_logic_1164.all;

entity component_sync is
  port (
    clk : in    std_logic;
    d   : in    std_logic;
    q   : out   std_logic
  );
end entity component_sync;

architecture good of component_sync is

  signal n1 : std_logic;
begin
  process (clk) is
  begin
    if rising_edge(clk) then
      n1 <= d;
      q  <= n1;
    end if;
  end process;
end architecture good;
