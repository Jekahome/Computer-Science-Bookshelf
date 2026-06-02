library ieee;

use ieee.std_logic_1164.all;

entity patternmoore is
  port (
    clk   : in    std_logic;
    reset : in    std_logic;
    a     : in    std_logic;
    y     : out   std_logic
  );
end entity patternmoore;

architecture synth of patternmoore is
  type statetype is (s0, s1, s2);
  signal state, nextstate : statetype;

begin

  -- регистр состояния
  process (clk, reset) is begin
    if (reset = '1') then
      state <= s0;
    elsif rising_edge(clk) then
      state <= nextstate;
    end if;
  end process;

  -- логика следующего состояния
  process (all) is begin
    case state is
      when s0 =>
        if (a = '1') then nextstate <= s0;
        else nextstate <= s1;
        end if;
      when s1 =>
        if (a = '1') then nextstate <= s2;
        else nextstate <= s1;
        end if;
      when s2 =>
        if (a = '1') then nextstate <= s0;
        else nextstate <= s1;
        end if;
      when others =>
        nextstate <= s0;
    end case;

  end process;

  -- выходная логика
  y <= '1' when state = s2 else
       '0';

end architecture synth;
