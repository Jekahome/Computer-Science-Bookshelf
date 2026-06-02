library IEEE;
use IEEE.STD_LOGIC_1164.all;

entity delays_ex1_tb is
end;

architecture sim of delays_ex1_tb is
 
  signal ts_a, ts_b, ts_c : std_logic := '1'; 
  signal ts_y             : std_logic;
begin

 
  UUT: entity work.delays_ex1 
    port map (
      a => ts_a, b => ts_b, c => ts_c, y => ts_y
    );

  
  process
  begin
    wait for 20 ns; 
    
    ts_a <= '0'; ts_b <= '0'; ts_c <= '0'; 
    wait for 20 ns; 
    
    ts_a <= '1'; ts_b <= '0'; ts_c <= '0';
    wait for 20 ns; 
    
    wait;
  end process;

end;