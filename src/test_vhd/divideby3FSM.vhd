library ieee;
use ieee.std_logic_1164.all;

entity divideby3fsm is
  port (
    clk   : in    std_logic;
    reset : in    std_logic;
    y     : out   std_logic
  );
end entity divideby3fsm;

architecture synth of divideby3fsm is
  type statetype is (s0, s1, s2);
  signal state, nextstate : statetype;

begin
  -- регистр состояния
  process (clk, reset) is
  begin
    if (reset) then
      state <= s0;
    elsif rising_edge(clk) then
      state <= nextstate;
    end if;
  end process;

  -- логика следующего состояния
  nextstate <= s1 when state = s0 else
               s2 when state = s1 else
               s0;
  -- выходная логика
  y <= '1' when state = s0 else
       '0';
end architecture synth;
