library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity sillyfunction_tb is
end entity sillyfunction_tb;

architecture bench of sillyfunction_tb is
  -- Simulation step period
  constant clk_period : time := 10 ns;
  
  -- Signals for DUT connection
  signal a : std_logic;
  signal b : std_logic;
  signal c : std_logic;
  signal y : std_logic;
begin

  -- Direct instantiation of Device Under Test (DUT)
  dut : entity work.sillyfunction
  port map (
    a => a,
    b => b,
    c => c,
    y => y
  );

  -- Stimulus and verification process
  stimulus : process
  begin
    -- 1. Combination 000 -> Expect 1
    a <= '0'; b <= '0'; c <= '0'; wait for clk_period;
    assert (y = '1') report "Error on combination 000! Expected 1." severity failure;
    
    -- 2. Combination 001 -> Expect 0
    a <= '0'; b <= '0'; c <= '1'; wait for clk_period;
    assert (y = '0') report "Error on combination 001! Expected 0." severity failure;
    
    -- 3. Combination 010 -> Expect 0
    a <= '0'; b <= '1'; c <= '0'; wait for clk_period;
    assert (y = '0') report "Error on combination 010! Expected 0." severity failure;
    
    -- 4. Combination 011 -> Expect 0
    a <= '0'; b <= '1'; c <= '1'; wait for clk_period;
    assert (y = '0') report "Error on combination 011! Expected 0." severity failure;
    
    -- 5. Combination 100 -> Expect 1
    a <= '1'; b <= '0'; c <= '0'; wait for clk_period;
    assert (y = '1') report "Error on combination 100! Expected 1." severity failure;
    
    -- 6. Combination 101 -> Expect 1
    a <= '1'; b <= '0'; c <= '1'; wait for clk_period;
    assert (y = '1') report "Error on combination 101! Expected 1." severity failure;
    
    -- 7. Combination 110 -> Expect 0
    a <= '1'; b <= '1'; c <= '0'; wait for clk_period;
    assert (y = '0') report "Error on combination 110! Expected 0." severity failure;
    
    -- 8. Combination 111 -> Expect 0
    a <= '1'; b <= '1'; c <= '1'; wait for clk_period;
    assert (y = '0') report "Error on combination 111! Expected 0." severity failure;

    -- Final message upon success
    report "All tests passed successfully! Component logic is correct." severity note;
    
    wait; -- Stop simulation
  end process;

end architecture bench;