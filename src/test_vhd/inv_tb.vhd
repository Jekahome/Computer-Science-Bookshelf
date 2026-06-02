
library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity inv_tb is
end entity;

architecture bench of inv_tb is
  -- Clock period
  constant clk_period : time := 10 ns;
  -- Generics
  -- Ports
  signal a : std_logic_vector(3 downto 0);
  signal y : std_logic_vector(3 downto 0);
begin

  inv_inst : entity work.inv
    port map
    (
      a => a,
      y => y
    );
  -- clk <= not clk after clk_period/2;
 
  stimulus : process
  begin
 
    a <= "0000";
    wait for clk_period;
 
    a <= "1111";
    wait for clk_period;
 
    a <= "0101";
    wait for clk_period;
 
    a <= "1010";
    wait for clk_period;
 
    a <= "1100";
    wait for clk_period;
 
    wait;
  end process;

end architecture;